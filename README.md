# Stack Error

A pragmatic error handling library for Rust that provides helpful strings for debugging, and structured data for runtime error handling.

## Overview

Stack Error provides an error type that is appropriate for library development while providing ergonomics similar to [anyhow](https://docs.rs/anyhow/latest/anyhow/).

- Provides error types that implement [`std::error::Error`]. Errors are compatible with the broader Rust ecosystem.
- Provides rich error context by chaining errors, and using the [`stack_msg!`] macro to include file and line information in error messages.
- Facilitates runtime error handling by providing a structured error data. The caller can match on the error code and inspect an optional resource URI to handle errors programmatically.
- Supports custom error types using a derive macros. Define your own error types, allowing you to create custom methods such as [`std::convert::From`] implementations.

## Usage

Import the prelude to get started:

```rust
use stackerror::prelude::*;
```

This will import the [`StackError`] type, the [`ErrorCode`] enum, the [`stack_msg!`] macro, and the traits used to build and stack errors.

You can build a new [`StackError`] from anything that is [`std::fmt::Display`]:

```rust
use stackerror::prelude::*;

fn process_data() -> Result<(), StackError> {
    Err(StackError::new("failed to process data"))
}
```

You can include file and line information in error messages using the [`stack_msg!`] macro:

```rust
use stackerror::prelude::*;

fn process_data() -> Result<(), StackError> {
    Err(StackError::new(stack_msg!("failed to process data")))
}
```

You can include optional error handling information:

```rust
use stackerror::prelude::*;

fn process_data() -> Result<(), StackError> {
    Err(
        StackError::new(stack_msg!("failed to process data"))
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
use stackerror::prelude::*;

pub read_data() -> Result<String, StackError> {
    Err(StackError::new(stack_msg!("failed to read data")))
}

pub process_data() -> Result<(), StackError> {
    // NOTE: stack_err can be called directly on the Result
    read_data().stack_err(stack_msg!("failed to process data"))
}
```

This would result in an error message like:

```
src/main:8 failed to process data
src/main:4 failed to read data
```

You can define your own error type. This allows you to implement your own methods, such as [`std::convert::From`] implementations for upstream error types frequently used in your library.

```rust
use stackerror::prelude::*;

#[derive_stack_error]
struct AppError(StackError);

impl std::convert::From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::new(err)
    }
}
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
struct AppError(StackError);
```

## Rationale

There are two distinct situation in which errors are used: during debugging and at runtime. During debugging, an error should provide an actionable and human-readable message that conveys _what_ went wrong and _how_ it happened. Whereas at runtime, an error should provide typed data allowing calling code to take appropriate error handling actions.

Stack Error is addresses those needs separately. First by offering an ergonomic interface for writing good error messages explaining _what_ went wrong, second by building a pseudo-trace that is focused on providing the relevant context to understand _how_ an error ocurred, and third by offering a generic interface to for a caller to get information about what resource caused and error and how to recover from it.

Stack Error is an experiment, and the hypothesis being tested is: does an error type that sits somewhere between `anyhow` and `thiserror` in terms of ergonomics and flexibility provide a good development experience. `anyhow` is the faster, more ergonomic way to write application code. And `thiserror` is the more flexible way to write bespoke library error types. If it succeeds, `stackerror` could be a pragmatic choice that's good enough for most cases, reducing the mental overhead of choosing and designing an error handling strategy for each project.

### Anyhow comparison

Stack Error is inspired by the [anyhow](https://docs.rs/anyhow/latest/anyhow/) library, and aims to borrow from its ergonomics while being suitable for library development. Using `anyhow` makes development quick as error handling is nearly always just a matter of adding the `?` operator. But this can slow down the debugging experience. Consider this example:

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

The resulting error message is: ``Error: expected `:` at line 1 column 27``. The message clearly states what went wrong, but not _how_ it went wrong (i.e. deserializing a config for printing). Running with with  the `RUST_BACKTRACE=1` prints a backtrace with this information, though it contains nearly 20 unrelated frames. Debugging this example is feasible, but a bit cumbersome.

And as a an application project grows, the distinction between application and library can become blurred as modules are introduced to support the application code. You might eventually find you want to handle some errors, but the `anyhow::Error` type is opaque. You can use `anyhow::Error::downcast`, but this is cumbersome as you need to try to downcast to every possible error type.

```rust
use anyhow::Result;
use reqwest;

fn fetch_data(resource: &str) -> Result<String> {
    let url = reqwest::Url::parse("https://busy.example")?.join(resource)?;
    let response = reqwest::blocking::get(url)?;
    let data = response.text()?;
    Ok(data)
}

fn print_data() -> Result<()> {
    match fetch_data("resource") {
        Ok(data) => println!("{}", data),
        Err(_) => {
            // should I retry the request, or will it fail because I sent an
            // invalid resource name?
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    print_data()?;
    Ok(())
}
```

## Stack traces

The [`ErrorStacks`] used with the [`stack_msg!`] macro allow for the construction of pseudo-traces which can be clearer  than a full stack trace. However, stack traces can still be useful to get a more complete picture of the state of the program when an error occurred.

It is currently not simple to get stack traces by relying on the [`std::error::Error`] trait. There are some proposals and nightly features to enable this. If that work makes its way into Rust stable, Stack Error could be updated to provide stack traces.