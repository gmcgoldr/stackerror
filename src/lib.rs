#![doc = include_str!("../README.md")]

mod error;
mod message;
pub mod prelude;
mod traits;

pub use prelude::*;
pub use stackerror_impl::derive_stack_error;

/// This module contains unit tests for the stackerror library.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = StackError::new("Test error");
        assert_eq!(error.to_string(), "Test error");
    }

    #[test]
    fn test_error_stacking() {
        let base_error = StackError::new("Base error");
        let stacked_error = base_error.stack_err("Stacked error");
        assert_eq!(stacked_error.to_string(), "Stacked error\nBase error");
    }

    #[test]
    fn test_error_code() {
        let error = StackError::new("Test error").with_err_code(Some(ErrorCode::ValidateInputs));
        assert_eq!(error.err_code(), Some(&ErrorCode::ValidateInputs));
    }

    #[test]
    fn test_error_uri() {
        let error = StackError::new("Test error")
            .with_err_uri(Some("https://example.com/error".to_string()));
        assert_eq!(error.err_uri(), Some("https://example.com/error"));
    }

    #[test]
    fn test_error_stacking_with_code() {
        let base_error =
            StackError::new("Base error").with_err_code(Some(ErrorCode::ValidateInputs));
        let stacked_error = base_error.stack_err_code("Stacked error");
        assert_eq!(stacked_error.to_string(), "Stacked error\nBase error");
        assert_eq!(stacked_error.err_code(), Some(&ErrorCode::ValidateInputs));
    }

    #[test]
    fn test_error_stacking_with_uri() {
        let base_error = StackError::new("Base error")
            .with_err_uri(Some("https://example.com/base".to_string()));
        let stacked_error = base_error.stack_err_uri("Stacked error");
        assert_eq!(stacked_error.to_string(), "Stacked error\nBase error");
        assert_eq!(stacked_error.err_uri(), Some("https://example.com/base"));
    }

    #[test]
    fn test_error_stacking_with_code_and_uri() {
        let base_error = StackError::new("Base error")
            .with_err_code(Some(ErrorCode::ValidateInputs))
            .with_err_uri(Some("https://example.com/base".to_string()));
        let stacked_error = base_error.stack_err_code_uri("Stacked error");
        assert_eq!(stacked_error.to_string(), "Stacked error\nBase error");
        assert_eq!(stacked_error.err_code(), Some(&ErrorCode::ValidateInputs));
        assert_eq!(stacked_error.err_uri(), Some("https://example.com/base"));
    }

    // Add this custom error struct
    #[derive_stack_error]
    struct CustomError(StackError);

    #[test]
    fn test_custom_error_builds() {
        let custom_error = CustomError::new("Custom error");
        assert_eq!(custom_error.to_string(), "Custom error");
    }

    #[test]
    fn test_custom_error_stacks() {
        let custom_error = CustomError::new("Custom error").stack_err("Stacked on custom");
        assert_eq!(custom_error.to_string(), "Stacked on custom\nCustom error");
    }

    #[test]
    fn test_custom_returns_code() {
        let coded_error =
            CustomError::new("Coded error").with_err_code(Some(ErrorCode::ValidateInputs));
        assert_eq!(coded_error.err_code(), Some(&ErrorCode::ValidateInputs));
    }

    #[test]
    fn test_custom_returns_uri() {
        let uri_error = CustomError::new("URI error")
            .with_err_uri(Some("https://example.com/custom".to_string()));
        assert_eq!(uri_error.err_uri(), Some("https://example.com/custom"));
    }

    #[test]
    fn test_custom_error_stacking_with_code() {
        let base_error =
            CustomError::new("Base custom error").with_err_code(Some(ErrorCode::ValidateInputs));
        let stacked_error = base_error.stack_err_code("Stacked custom error");
        assert_eq!(
            stacked_error.to_string(),
            "Stacked custom error\nBase custom error"
        );
        assert_eq!(stacked_error.err_code(), Some(&ErrorCode::ValidateInputs));
    }

    #[test]
    fn test_custom_error_stacking_with_uri() {
        let base_error = CustomError::new("Base custom error")
            .with_err_uri(Some("https://example.com/base_custom".to_string()));
        let stacked_error = base_error.stack_err_uri("Stacked custom error");
        assert_eq!(
            stacked_error.to_string(),
            "Stacked custom error\nBase custom error"
        );
        assert_eq!(
            stacked_error.err_uri(),
            Some("https://example.com/base_custom")
        );
    }

    #[test]
    fn test_custom_error_stacking_with_code_and_uri() {
        let base_error = CustomError::new("Base custom error")
            .with_err_code(Some(ErrorCode::ValidateInputs))
            .with_err_uri(Some("https://example.com/base_custom".to_string()));
        let stacked_error = base_error.stack_err_code_uri("Stacked custom error");
        assert_eq!(
            stacked_error.to_string(),
            "Stacked custom error\nBase custom error"
        );
        assert_eq!(stacked_error.err_code(), Some(&ErrorCode::ValidateInputs));
        assert_eq!(
            stacked_error.err_uri(),
            Some("https://example.com/base_custom")
        );
    }
}
