# Stack Error

A pragmatic error handling library that provides helpful strings for debugging, and structured data for runtime error handling.

## Overview

Stack Error provides an error type that is appropriate for library development while providing ergonomics similar to [anyhow](https://docs.rs/anyhow/latest/anyhow/).

- Provides error types that implement [`std::error::Error`]. Errors are compatible with the broader Rust ecosystem.
- Facilitates runtime error handling by providing a structured error data. The caller can match on the error code and inspect an optional resource URI to handle errors programmatically.
- Provides rich error context by chaining errors, creating a pseudo-stack, and using the [`stack_msg!`] macro to include file and line information in error messages.
- Supports custom error types using a derive macros. Define your own error types, allowing you to create custom methods such as [`std::convert::From`] implementations.

## Usage

Import the prelude to get started:

```rust
use stackerror::prelude::*;
```

This will import the [`StackError`] type, the [`ErrorHandling`] enum, the [`stack_msg!`] macro, and the [`ErrorStacks`], [`ErrorWithCode`], and [`ErrorWithUri`] traits.

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
        .with_err_code(ErrorHandling::RetryResource)
        .with_err_uri("https://example.com/busy-resource")
    )
}

fn main() {
    let result = process_data();
    if let Err(err) = result {
        match err.err_code() {
            ErrorHandling::RetryResource => {
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

You can define your own error type that you can implement custom methods on. This allows you to implement your own methods, such as [`std::convert::From`] implementations for upstream error types frequently used in your library.

```rust
use stackerror::prelude::*;

#[derive_stack_error]
struct AppError(StackError);
```

## Rationale

There are two distinct consumers of errors: programmers at debug-time, and code at runtime.

During debugging, programmers need human-readable error messages to explain what went wrong and why. To answer the why, the programmer typically needs to know the state of the program when the error-returning function was called. To serve this user, an error type facilitate providing clear and detailed error message.

At runtime, some errors can be handled programmatically. For example, a networked resource could respond with an HTTP busy status code, in which case the program could wait and retry the request. In order to handle errors programmatically, the program needs access to structured error data that is part of the library's public API. Trying to use error messages to provide clear human-readable information at debug-time, and structured data at runtime leads to poor results for both.

### Stack traces

The [`ErrorStacks`] used with the [`stack_msg!`] macro allow for the construction of pseudo-traces which can be clearer  than a full stack trace. However, stack traces can still be useful to get a more complete picture of the state of the program when an error occurred.

It is currently not easy to get stack traces by relying on the [`std::error::Error`] trait. There are some proposals and nightly features to enable this. If that work makes its way into Rust stable, Stack Error could be updated to provide stack traces.