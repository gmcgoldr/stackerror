#![doc = include_str!("../README.md")]

mod error;
mod message;
pub mod prelude;

pub use prelude::*;
pub use stackerror_impl::derive_stack_error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_builds() {
        let error = StackError::new("Test error");
        assert_eq!(error.to_string(), "0: Test error");
    }

    #[test]
    fn test_error_has_code() {
        let error = StackError::new("Test error").with_err_code(Some(ErrorCode::InvalidInput));
        assert_eq!(error.err_code(), Some(&ErrorCode::InvalidInput));
    }

    #[test]
    fn test_error_has_uri() {
        let error = StackError::new("Test error")
            .with_err_uri(Some("https://example.com/error".to_string()));
        assert_eq!(error.err_uri(), Some("https://example.com/error"));
    }

    #[test]
    fn test_error_stacks() {
        let base_error = StackError::new("Base error")
            .with_err_code(Some(ErrorCode::InvalidInput))
            .with_err_uri(Some("https://example.com/base".to_string()));
        let stacked_error = base_error.stack_err("Stacked error");
        assert_eq!(stacked_error.to_string(), "0: Base error\n1: Stacked error");
        assert_eq!(stacked_error.err_code(), Some(&ErrorCode::InvalidInput));
        assert_eq!(stacked_error.err_uri(), Some("https://example.com/base"));
    }

    // Add this custom error struct
    #[derive_stack_error]
    struct LibError(StackError);

    #[test]
    fn test_derived_builds() {
        let custom_error = LibError::new("Custom error");
        assert_eq!(custom_error.to_string(), "0: Custom error");
    }

    #[test]
    fn test_derived_has_code() {
        let coded_error = LibError::new("Coded error").with_err_code(Some(ErrorCode::InvalidInput));
        assert_eq!(coded_error.err_code(), Some(&ErrorCode::InvalidInput));
    }

    #[test]
    fn test_derived_has_uri() {
        let uri_error =
            LibError::new("URI error").with_err_uri(Some("https://example.com/custom".to_string()));
        assert_eq!(uri_error.err_uri(), Some("https://example.com/custom"));
    }

    #[test]
    fn test_derived_error_stacks() {
        let base_error = LibError::new("Base error")
            .with_err_code(Some(ErrorCode::InvalidInput))
            .with_err_uri(Some("https://example.com/base_custom".to_string()));
        let stacked_error = base_error.stack_err("Stacked error");
        assert_eq!(stacked_error.to_string(), "0: Base error\n1: Stacked error");
        assert_eq!(stacked_error.err_code(), Some(&ErrorCode::InvalidInput));
        assert_eq!(
            stacked_error.err_uri(),
            Some("https://example.com/base_custom")
        );
    }

    #[test]
    fn test_derived_creation_map() {
        let error: Result<(), LibError> =
            Err(LibError::new("Base error")).map_err(stack_map!("Stacked error"));
        assert!(error.unwrap_err().to_string().ends_with("Stacked error"));
    }

    #[test]
    fn test_derived_creation_fn() {
        let error: Result<(), LibError> = Option::None.ok_or_else(stack_else!("Base error"));
        assert!(error.unwrap_err().to_string().ends_with("Base error"));
    }
}
