# Stack Error

Stack Error reduces the up-front cost of designing an error handling solution for your project, 
so that you focus on writing great libraries and applications. Stack Error has three goals:

1. Provide ergonomics similar to [`anyhow`](https://docs.rs/anyhow/latest/anyhow/).
2. Create informative error messages that facilitate debugging.
3. Provide typed data that facilitates runtime error handling.

## Overview

- Build informative error messages for debugging with minimal effort. 
  The error message is co-located with the error source, which helps document your code.

  ```rust
  use stackerror::prelude::*;

  pub fn process_data(data: &str) -> StackResult<String> {
      let data: Vec<String> = serde_json::from_str(data)
          .map_err(StackError::from_msg)
          .stack_err_msg(fmt_loc!("data is not a list of strings"))?;
      data.first()
          .cloned()
          .ok_or_else(StackError::new)
          .with_err_msg(fmt_loc!("data is empty"))
  }
  ```

  In this example,
  when the data isn't a valid JSON vector, 
  [`StackError::from_msg`] is used to create a new error using serde's error message, 
  and the [`StackError::stack_err_msg`] method is used to stack a new error onto this with a custom message. 
  The [`fmt_loc`] marco prepends the file name and line number to the message. 
  The resulting error message is:

  ```
  Error: EOF while parsing a value at line 1 column 0
  src/main.rs:6 data is not a list of strings
  ```

  If the data is an empty JSON vector, 
  then [`StackError::new`] is used to create an empty error, 
  and [`StackError::with_err_msg`] is used to set the error's message.

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

- Easily define your own error type. This allows you to customize error codes,
  derive `From` implementations, 
  and provide a single opaque error type to downstream code.

- Provides error types that implement [`std::error::Error`]. 
  Errors are compatible with the broader Rust ecosystem.


## Rationale

The Rust error handling ecosystem is largely built on two libraries: 
[`anyhow`](https://docs.rs/anyhow/latest/anyhow/) and [`thiserror`](https://docs.rs/thiserror/latest/thiserror/). 
Stack Error aims to explore the space between these two libraries: 
providing ergonomic error handling and an error type that is suitable for library development.

### Anyhow comparison

Using [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) makes development quick as error handling is nearly always just a matter of adding the `?` operator. 
But this can slow down the debugging process. 
Consider this example:

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

The core of the library is the [`StackError`] struct and the [`ErrorStacks`]
trait. The [`ErrorCode`] enum can be used to add error codes to any 
[`ErrorStacks`].

The [`ErrorStacks`] methods are implemented on [`StackError`]  and 
`Result<_, StackError>`. So you can act on the `StackError` inside a `Result`. 
The methods have no impact on the result if it is an `Ok` variant.

Typically, you will access these using the [`prelude`] module which also defines [`StackResult`].

## Custom error type

Create your error type by using the [`derive_stack_error`] macro:

```rust
// src/errors.rs

pub use stackerror::prelude::*;

#[derive_stack_error]
struct LibError(StackError);

pub type LibResult<T> = std::result::Result<T, LibError>;
```

Then you can replace `use stackerror::prelude::*` with `use crate::errors::*`
in your code in your code, and use `LibError` and `LibResult`.

This has several benefits:

- Your code exposes a single opaque error type. Downstream error-handling code 
  doesn't need to know about `StackError`, and can write `From` implementations
  that handle any error returned by your code.
- Debug messages include only errors from your codebase since you can stack 
  only instances of `LibError`.
- You can customize the error codes.

## Examples

You can build a new error with an error message that is [`std::fmt::Display`]:

```rust
use stackerror::prelude::*;

fn process_data() -> StackResult<()> {
    Err(StackError::from_msg("failed to process data"))
}
```

You can include file and line information in error messages using the [`fmt_loc!`] macro:

```rust
use stackerror::prelude::*;

fn process_data() -> StackResult<()> {
    Err(StackError::from_msg(fmt_loc!("failed to process data")))
}
```

You can include optional error handling information:

```rust
use stackerror::prelude::*;

fn process_data() -> StackResult<()> {
    Err(
        StackError::from_msg(fmt_loc!("failed to process data"))
        .with_err_code(ErrorCode::HttpImATeapot)
        .with_err_uri("https://example.invalid/teapot")
    )
}
```

You can chain errors together to provide context in the error message:

```rust
use stackerror::prelude::*;

pub read_data() -> StackResult<String> {
    Err(StackError::from_msg(fmt_loc!("failed to read data")))
}

pub process_data() -> StackResult<()> {
    // NOTE: stack_err can be called directly on the Result
    read_data().stack_err_msg(fmt_loc!("failed to process data"))
}
```

This would result in an error message like:

```
src/main:9 failed to process data
src/main:4 failed to read data
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