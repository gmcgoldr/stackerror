#![doc = include_str!("../README.md")]

mod error;
mod traits;

pub use error::{Error, ErrorHandling};
pub use stackerror_impl::derive_stack_error;
pub use traits::{ErrorCode, ErrorStack, ErrorUri};

/// This module contains unit tests for the stackerror library.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = Error::from_error("Test error");
        assert_eq!(error.to_string(), "Test error");
    }

    #[test]
    fn test_error_stacking() {
        let base_error = Error::from_error("Base error");
        let stacked_error = base_error.stack_error("Stacked error");
        assert_eq!(stacked_error.to_string(), "Stacked error\nBase error");
    }

    #[test]
    fn test_error_code() {
        let error = Error::from_error("Test error").with_code(Some(ErrorHandling::ValidateInputs));
        assert_eq!(error.code(), Some(&ErrorHandling::ValidateInputs));
    }

    #[test]
    fn test_error_uri() {
        let error =
            Error::from_error("Test error").with_uri(Some("https://example.com/error".to_string()));
        assert_eq!(error.uri(), Some("https://example.com/error"));
    }

    // Add this custom error struct
    #[derive_stack_error]
    struct CustomError(Error);

    #[test]
    fn test_custom_error_builds() {
        let custom_error = CustomError::from_error("Custom error");
        assert_eq!(custom_error.to_string(), "Custom error");
    }

    #[test]
    fn test_custom_error_stacks() {
        let custom_error = CustomError::from_error("Custom error").stack_error("Stacked on custom");
        assert_eq!(custom_error.to_string(), "Stacked on custom\nCustom error");
    }

    #[test]
    fn test_custom_returns_code() {
        let coded_error =
            CustomError::from_error("Coded error").with_code(Some(ErrorHandling::ValidateInputs));
        assert_eq!(coded_error.code(), Some(&ErrorHandling::ValidateInputs));
    }

    #[test]
    fn test_custom_returns_uri() {
        let uri_error = CustomError::from_error("URI error")
            .with_uri(Some("https://example.com/custom".to_string()));
        assert_eq!(uri_error.uri(), Some("https://example.com/custom"));
    }
}
