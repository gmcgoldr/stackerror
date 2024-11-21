# Stack Error

A pragmatic error handling library for Rust that provides helpful messages for debugging, and structured data for runtime error handling.

## Overview

- Build informative error messages for debugging with minimal effort. The error message is co-located with the error source, providing code clarity. The [`stack_msg!`] macro includes the file and line number in the error message.

  ```rust
  use stackerror::prelude::*;

  pub fn process_data(data: &str) -> Result<String> {
      let data: Vec<String> = serde_json::from_str(data)
          .map_err(StackError::new)
          .stack_err(stack_msg!("data is not a list of strings"))?;
      data
          .first()
          .cloned()
          .ok_or_else(|| StackError::new(stack_msg!("data is empty")))
  }
  ```

- Facilitates runtime error handling by providing an optional error code and URI. The caller can match on the code and inspect the URI to handle errors programmatically.

  ```rust
  use stackerror::prelude::*;
  
  fn fetch_data(url: &str) -> Result<String> {
      let response = reqwest::blocking::get(url)
          .map_err(StackError::new)
          .stack_err(stack_msg!("unable to get the data"))
          // the caller can handle this by trying to get the resource from 
          // another location
          .with_err_code(ErrorCode::ResourceUnavailable)
          .with_err_uri(url.to_string())?;
      let data = response
          .text()
          .map_err(StackError::new)
          .stack_err(stack_msg!("unable to prase the data"))
          // the caller can handle this by bypassing the resource
          .with_err_code(ErrorCode::InvalidResource)
          .with_err_uri(url.to_string())?;
      Ok(data)
  }
  ```

- Supports custom error types using a derive macros. Define your own error types, allowing you to create custom methods such as [`std::convert::From`] implementations.
- Provides error types that implement [`std::error::Error`]. Errors are compatible with the broader Rust ecosystem.

## Usage

You start simply by using the [`StackError`] type directly, but it's recommended to wrap it in your own error type using the [`derive_stack_error`] macro:

```rust
// src/errors.rs

pub use stackerror::prelude::*;

#[derive_stack_error]
struct LibError(StackError);

pub type Result<T> = std::result::Result<T, LibError>;
```

The prelude provides the [`ErrorStacks`] trait, the [`stack_msg!`] macro, and the [`ErrorCode`] enum. The [`ErrorStacks`] methods are implemented for your `LibError` and for any `Result<T, LibError>`.

You can build a new error from anything that is [`std::fmt::Display`]:

```rust
use crate::errors::prelude::*;

fn process_data() -> Result<()> {
    Err(LibError::new("failed to process data"))
}
```

You can include file and line information in error messages using the [`stack_msg!`] macro:

```rust
use crate::errors::prelude::*;

fn process_data() -> Result<()> {
    Err(LibError::new(stack_msg!("failed to process data")))
}
```

You can include optional error handling information:

```rust
use crate::errors::prelude::*;

fn process_data() -> Result<()> {
    Err(
        LibError::new(stack_msg!("failed to process data"))
        .with_err_code(ErrorCode::ResourceBusy)
        .with_err_uri("https://example.com/busy-resource")
    )
}

fn main() {
    let result = process_data();
    if let Err(err) = result {
        match err.err_code() {
            ErrorCode::ResourceBusy => {
                // retry the resource
            }
            _ => {
                // unhandled error
            }
        }
    }
}
```

You can chain errors together to provide context in the error message:

```rust
use crate::errors::prelude::*;

pub read_data() -> Result<String> {
    Err(LibError::new(stack_msg!("failed to read data")))
}

pub process_data() -> Result<()> {
    // NOTE: stack_err can be called directly on the Result
    read_data().stack_err(stack_msg!("failed to process data"))
}
```

This would result in an error message like:

```
src/main:8 failed to process data
src/main:4 failed to read data
```

You can use your own error codes by defining an `ErrorCode` type in the scope where `derive_stack_error` is used:

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

## Rationale

The Rust error handling ecosystem is largely built on two fantastic libraries: [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) and [`thiserror`](https://docs.rs/thiserror/latest/thiserror/). Stack Error aims to explore the space between these two libraries: providing ergonomic error handling and an error type that is suitable for library development.

### Anyhow comparison

Using `anyhow` makes development quick as error handling is nearly always just a matter of adding the `?` operator. But this can slow down the debugging process. Consider this example:

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

Handling all errors with the `?` operator speeds up the writing process, but can hinder the overall development process by making debugging more difficult. `anyhow` offers an alternative: the [`anyhow::Context`] trait.

Using the `context` method helps provide clear error messages that answer _what_ went wrong and _how_ it went wrong. It also helps document the code by co-locating error sources with their corresponding error messages. This is the inspiration for Stack Error's `stack_err` method.

As a an application grows, the distinction between application and library can become blurred as modules are introduced to support the application code. You might eventually find you want to handle some errors. You can use `anyhow::Error::downcast`, but this is cumbersome as you need to try to downcast to every possible error type.

### Thiserror comparison

The `thiserror` library provides flexible tools to facilitate the creation of custom error types. There is no one way to use it, so it's hard to make a direct comparison other than to say: Stack Error is opinionated and aims to be minimal but generally useful. This reduces the effort required to develop (and stick to) a good error handling strategy for your project. By contrast, when using `thiserror`, you have to make many decisions at the start of a project about where and how to define your errors. And if you don't put much thought into the design of your errors, you could end up:

- Constantly duplicating error messages in enum variant names and error message strings;
- Creating generic errors that result in poor debugging messages and insufficient runtime information for error handling;
- Creating too many errors exposing the internals of your library or application, making it hard to refactor;
- Having to move back-and-forth between writing code and defining error variants in separate files.

## Stack traces

The [`ErrorStacks`] used with the [`stack_msg!`] macro allow for the construction of pseudo-traces which can be clearer  than a full stack trace. However, stack traces can still be useful to get a more complete picture of the state of the program when an error occurred.

It is currently not simple to get stack traces by relying on the [`std::error::Error`] trait. There are some proposals and nightly features to enable this. If that work makes its way into Rust stable, Stack Error could be updated to provide stack traces.