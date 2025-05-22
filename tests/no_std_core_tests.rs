#![no_std]

// We can't use the main error type `StackError` without `alloc` because it uses `Box`.
// So, for a true `no_std` (core only) test, we'd need a very simple error type
// or focus on parts of the crate that don't require allocation.
// The current `ErrorStacks` trait itself is generic and could potentially be used
// with non-allocating error types if they existed.

// For now, let's create a placeholder test that ensures the crate can be imported.
// We will also test the `codes::ErrorCode` as it should be `no_std` compatible.

extern crate stackerror; // Use `extern crate` as we are in a separate test file.
use stackerror::codes::ErrorCode;

#[test]
fn test_error_code_can_be_used() {
    let code = ErrorCode::RuntimeInvalidValue;
    assert_eq!(code as u32, 1001); // Example assertion
}

// We can't easily test `StackError` or the derive macro here without `alloc`.
// If there were parts of `ErrorStacks` that could be implemented for a simple,
// non-allocating error struct, we could test that here.
