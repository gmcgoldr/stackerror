# Stack Error

This library provides [`Error`][Error], 
an error type for convenient error handling in Rust libraries and applications.

The library also includes the following traits which are implemented by [`Error`][Error]: 
[`StackError`][StackError], 
[`ErrorCode`][ErrorCode], 
[`ErrorUri`][ErrorUri]. 
And a convenience macro to build your own Error type [`derive_stack_error`][derive_stack_error].

## Motivation

The premise behind this library is that errors have two purposes:

1. Provide information about what went wrong at debug time
2. Allow for handling errors at runtime

A typical solution to the first problem is to provide informative error messages, 
or a backtrace. 
This library provides the [`StackError`][StackError] trait to make it easy to provide informative error messages.

A typical solution to the second problem is to provide an error code.
This library provides the [`ErrorCode`][ErrorCode] and [`ErrorUri`][ErrorUri] traits to make it easy to provide information about how the error can be handled.

## StackError trait

The [`StackError`][StackError] trait allows you to stack error messages. 
Calling `stack_error` on a result will create a new error with the original message and the new message.

```rust
let result: Result<i32, Error> = Err(Error::from_error("something went wrong"));
let stacked_error = result.stack_error("unable to proceed");
assert_eq!(stacked_error.to_string(), "unable to proceed\nsomething went wrong");
```

In this case, 
the developer can understand what parts of the program were involved in the error.

## ErrorCode and ErrorUri traits

The [`ErrorCode`][ErrorCode] and [`ErrorUri`][ErrorUri] traits 
allow you to provide the caller with information about how to handle the error.

Typically, runtime failures are caused by IO.
This library proposes a minimal set of error handling codes that can be used to handle (work around) typical IO errors.

```rust
let result: Result<i32, Error> = Err(Error::from_error("something went wrong"));
let error = result.with_code(ErrorHandling::RetryResource).with_uri("https://example.com/busyresource");
assert_eq!(error.code(), Some(&ErrorHandling::RetryResource));
assert_eq!(error.uri(), Some("https://example.com/busyresource"));
```

In this case,
the caller could retry the operation later, 
or try a different resource.

## derive_stack_error macro

The [`derive_stack_error`][derive_stack_error] macro allows you to create your own error type that behaves like [`Error`][Error].
The benefit is that you can implement your own methods since you own the type.

```rust
#[derive_stack_error]
pub struct MyError(Error);
```
