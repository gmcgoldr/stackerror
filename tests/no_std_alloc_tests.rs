#![no_std]
#![feature(alloc_error_handler)] // Required for tests involving alloc

extern crate alloc;
extern crate stackerror; // Use `extern crate`

use stackerror::prelude::*; // Imports StackError, ErrorStacks, ErrorCode
use stackerror::derive_stack_error;

// Basic allocator and panic handler for testing in no_std + alloc environment
// This is a minimal setup. A real no_std application would have a proper allocator.
#[cfg(not(feature = "std"))]
mod no_std_setup {
    use alloc::alloc::{GlobalAlloc, Layout};
    use core::panic::PanicInfo;

    struct DummyAllocator;

    unsafe impl GlobalAlloc for DummyAllocator {
        unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
            core::ptr::null_mut() // A real allocator would do actual allocation
        }
        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
    }

    #[global_allocator]
    static ALLOCATOR: DummyAllocator = DummyAllocator;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }

    #[alloc_error_handler]
    fn alloc_error_handler(_layout: Layout) -> ! {
        loop {}
    }
}


#[test]
fn test_stack_error_no_std_alloc() {
    let error = StackError::new("Base error in no_std_alloc");
    // `to_string` on `Display` trait is fine.
    assert!(error.to_string().contains("Base error in no_std_alloc"));
}

#[test]
fn test_stack_error_with_code_no_std_alloc() {
    let error = StackError::new("Coded error").with_err_code(Some(ErrorCode::RuntimeInvalidValue));
    assert_eq!(error.err_code(), Some(&ErrorCode::RuntimeInvalidValue));
}

#[test]
fn test_stack_error_with_uri_no_std_alloc() {
    let error = StackError::new("URI error").with_err_uri(Some(alloc::string::String::from("test/uri")));
    assert_eq!(error.err_uri(), Some("test/uri"));
}

#[test]
fn test_error_stacking_no_std_alloc() {
    let base = StackError::new("Base").with_err_code(Some(ErrorCode::NotFound));
    let stacked = base.stack_err("Stacked");
    assert!(stacked.to_string().contains("Base"));
    assert!(stacked.to_string().contains("Stacked"));
    assert_eq!(stacked.err_code(), Some(&ErrorCode::NotFound));
}

#[derive_stack_error)]
#[allow(dead_code)] // To suppress warnings if LibError is not used beyond definition
struct LibError(StackError);

#[test]
fn test_derived_error_no_std_alloc() {
    let custom_error = LibError::new("Custom error");
    assert!(custom_error.to_string().contains("Custom error"));
    let coded_error = custom_error.with_err_code(Some(ErrorCode::PermissionDenied));
    assert_eq!(coded_error.err_code(), Some(&ErrorCode::PermissionDenied));
}
