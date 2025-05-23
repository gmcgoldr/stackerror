# Stack Error

Stack Error reduces the up-front cost of designing an error handling solution for your project, 
so that you focus on writing great libraries and applications. Stack Error has three goals:

1. Provide ergonomics similar to [`anyhow`](https://docs.rs/anyhow/latest/anyhow/).
2. Create informative error messages that facilitate debugging.
3. Provide typed data that facilitates runtime error handling.

## Overview

- Build informative error messages for debugging with minimal effort. The error message is co-located with the error source, which helps document your code.

  ```rust
  use stackerror::prelude::*;

  pub fn process_data(data: &str) -> StackResult<String> {
      let data: Vec<String> = serde_json::from_str(data)
          .map_err(stack_map!(StackError, "data is not a list of strings"))?;
      data.first()
          .cloned()
          .ok_or_else(stack_else!(StackError, "data is empty"))
  }
  ```

  In this example,
  [`stack_map!`] and [`stack_err!`] build a new instance of [`StackError`], 
  adding file name and line number information to the message. 
  In the case of [`stack_err!`], 
  the error message stacks onto the existing error.
  Note that macros are used to simplify common operations, 
  and the same outcome can be achieved using closures instead of macros.

  If the data isn't a list,
  the resulting error message would look like:

  ```
  Error: expected value at line 1 column 1
  src/process.rs:4 data is not a list of strings
  ```

  The serde error is printed first,
  followed by the StackError message with file name and line number. 

- Handle errors at runtime by inspecting an optional error code.

  ```rust
  let data = if data.err_code() == Some(&ErrorCode::HttpTooManyRequests) {
      // retry
  } else {
      data?
  };
  ```

  [`ErrorCode`] includes HTTP error codes,
  [`std::io::ErrorKind`] codes,
  and a handful of runtime codes to cover non-HTTP and non-IO cases.
  You can derive your own error codes as described later in the examples.

- Define your own error type, allowing you to create custom methods such as [`std::convert::From`] implementations.

- Provides error types that implement [`std::error::Error`]. Errors are compatible with the broader Rust ecosystem.


## Rationale

The Rust error handling ecosystem is largely built on two libraries: [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) and [`thiserror`](https://docs.rs/thiserror/latest/thiserror/). Stack Error aims to explore the space between these two libraries: providing ergonomic error handling and an error type that is suitable for library development.

### Anyhow comparison

Using [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) makes development quick as error handling is nearly always just a matter of adding the `?` operator. But this can slow down the debugging process. Consider this example:

```rust
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct Config {
    key: String,
}

fn print_config(data: &str) -> Result<()> {
    let config: Config = serde_json::from_str(data)?;
    println!("{:?}", config);
    Ok(())
}

fn main() -> Result<()> {
    print_config(r#"{"key": "value", "invalid"}"#)?;
    Ok(())
}
```

The resulting error message is: ``Error: expected `:` at line 1 column 27``. The message clearly states what went wrong, but not _how_ it went wrong: what was the program decoding, and why is the value needed. Running with `RUST_BACKTRACE=1` prints a backtrace that can answer these questions, though it contains nearly 20 unrelated frames. Debugging this example is feasible, but a bit cumbersome.

Handling all errors with the `?` operator speeds up the writing process, but can hinder the overall development process by making debugging more difficult. `anyhow` offers an alternative: the `anyhow::Context` trait.

Using the `context` method helps provide clear error messages that answer _what_ went wrong and _how_ it went wrong. It also helps document the code by co-locating error sources with their corresponding error messages. This is the inspiration for Stack Error's `stack_err` method.

As a an application grows, the distinction between application and library can become blurred as modules are introduced to support the application code. You might eventually find you want to handle some errors. You can use `anyhow::Error::downcast`, but this is cumbersome as you need to try to downcast to every possible error type.

### Thiserror comparison

The [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) library provides flexible tools to facilitate the creation of custom error types. There is no one way to use it, so it's hard to make a direct comparison other than to say: Stack Error is opinionated and aims to be minimal but generally useful. This reduces the effort required to develop (and stick to) a good error handling strategy for your project. By contrast, when using `thiserror`, you have to make many decisions at the start of a project about where and how to define your errors. And if you don't put much thought into the design of your errors, you could end up:

- Constantly duplicating error messages in enum variant names and error message strings;
- Creating generic errors that result in poor debugging messages and insufficient runtime information for error handling;
- Creating too many errors exposing the internals of your library or application, making it hard to refactor;
- Having to move back-and-forth between writing code and defining error variants in separate files.

## Library structure

The core of the library is the [`StackError`] struct and the [`ErrorStacks`] trait. 
The [`ErrorCode`] enum can be used to add error codes to any [`ErrorStacks`]. 
And the [`stack_msg!`], [`stack_err!`], [`stack_map!`] and [`stack_else!`] 
macros are provided to simplify common operations on [`Result`]s, 
and to add file name and line number information to error messages.

Typically, you will access these using the [`prelude`] module which also defines [`StackResult`].

## Examples

Create your error type by using the [`derive_stack_error`] macro:

```rust
// src/errors.rs

pub use stackerror::prelude::*;

#[derive_stack_error]
struct Error(StackError);

pub type Result<T> = std::result::Result<T, Error>;
```

You can build a new error from anything that is [`std::fmt::Display`]:

```rust
use stackerror::prelude::*;

fn process_data() -> StackResult<()> {
    Err(StackError::new("failed to process data"))
}
```

You can include file and line information in error messages using the [`stack_msg!`] macro:

```rust
use stackerror::prelude::*;

fn process_data() -> StackResult<()> {
    Err(StackError::new(stack_msg!("failed to process data")))
}
```

You can include optional error handling information:

```rust
use stackerror::prelude::*;

fn process_data() -> StackResult<()> {
    Err(
        StackError::new(stack_msg!("failed to process data"))
        .with_err_code(ErrorCode::HttpImATeapot)
        .with_err_uri("https://example.invalid/teapot")
    )
}
```

You can chain errors together to provide context in the error message:

```rust
use stackerror::prelude::*;

pub read_data() -> StackResult<String> {
    Err(StackError::new(stack_msg!("failed to read data")))
}

pub process_data() -> StackResult<()> {
    // NOTE: stack_err can be called directly on the Result
    read_data().stack_err(stack_msg!("failed to process data"))
}
```

This would result in an error message like:

```
src/main:8 failed to process data
src/main:4 failed to read data
```

The [`stack_err!`] macro offers a shorthand for the common pattern `Err(Error::new(stack_msg!(...)))`:

```rust
use stackerror::prelude::*;

pub read_data() -> StackResult<String> {
    stack_err!("failed_to_read_data")
}
```

You can wrap an existing error:

```rust
use stackerror::prelude::*;

pub fn process_data(data: &str) -> Vec<String> {
    serde_json::from_str(data)
        .map_err(StackError::new)
        .stack_err(stack_msg!("data is not a list of strings"))
}
```

The [`stack_map!`] (and similarly [`stack_else!`]) macro offers a shorthand for this common pattern. They accept the error type to wrap the original error with, and the error message to stack onto it. The error type must be [`ErrorStacks`].

```rust
use stackerror::prelude::*;

pub fn process_data(data: &str) -> Vec<String> {
    serde_json::from_str(data)
        .map_err(stack_map!(StackError, "data is not a list of strings"))
}
```

You can use your own error codes by defining an [`ErrorCode`] type in the scope where [`derive_stack_error`] is used:

```rust
use stackerror::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ErrorCode {
    SomethingWentWrong,
    SomethingElseWentWrong,
}

#[derive_stack_error]
struct ErrorWithCustomCodes(StackError);
```