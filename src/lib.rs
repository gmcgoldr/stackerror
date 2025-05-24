#![doc = include_str!("../README.md")]

pub mod codes;
pub mod error;
pub mod macros;
pub mod prelude;

pub use prelude::*;
pub use stackerror_impl::derive_stack_error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_builds_empty() {
        let error = StackError::new();
        assert_eq!(format!("{:?}", error), "");
    }

    #[test]
    fn test_error_builds_from_msg() {
        let error = StackError::from_msg("Test error");
        assert_eq!(format!("{:?}", error), "Test error");
    }

    #[test]
    fn test_error_has_err() {
        let error = StackError::new().with_err_msg("Test error");
        assert_eq!(format!("{:?}", error), "Test error");
    }

    #[test]
    fn test_error_has_code() {
        let error = StackError::new().with_err_code(ErrorCode::RuntimeInvalidValue);
        assert_eq!(error.err_code(), Some(&ErrorCode::RuntimeInvalidValue));
    }

    #[test]
    fn test_error_has_uri() {
        let error = StackError::new().with_err_uri("https://example.com/error".to_string());
        assert_eq!(error.err_uri(), Some("https://example.com/error"));
    }

    #[test]
    fn test_error_stacks() {
        let base_error = StackError::from_msg("Base error")
            .with_err_code(ErrorCode::RuntimeInvalidValue)
            .with_err_uri("https://example.com/base".to_string());
        let stacked_error = base_error.stack_err_msg("Stacked error");
        assert_eq!(format!("{:?}", stacked_error), "Base error\nStacked error");
        assert_eq!(
            stacked_error.err_code(),
            Some(&ErrorCode::RuntimeInvalidValue)
        );
        assert_eq!(stacked_error.err_uri(), Some("https://example.com/base"));
    }

    // Add this custom error struct
    #[derive_stack_error]
    struct LibError(StackError);

    #[test]
    fn test_custom_builds_empty() {
        let error = LibError::new();
        assert_eq!(format!("{:?}", error), "");
    }

    #[test]
    fn test_custom_builds_from_msg() {
        let error = LibError::from_msg("Test error");
        assert_eq!(format!("{:?}", error), "Test error");
    }

    #[test]
    fn test_custom_has_err() {
        let error = LibError::new().with_err_msg("Test error");
        assert_eq!(format!("{:?}", error), "Test error");
    }

    #[test]
    fn test_custom_has_code() {
        let error = LibError::new().with_err_code(ErrorCode::RuntimeInvalidValue);
        assert_eq!(error.err_code(), Some(&ErrorCode::RuntimeInvalidValue));
    }

    #[test]
    fn test_custom_has_uri() {
        let error = LibError::new().with_err_uri("https://example.com/error".to_string());
        assert_eq!(error.err_uri(), Some("https://example.com/error"));
    }

    #[test]
    fn test_custom_stacks() {
        let base_error = LibError::from_msg("Base error")
            .with_err_code(ErrorCode::RuntimeInvalidValue)
            .with_err_uri("https://example.com/base".to_string());
        let stacked_error = base_error.stack_err_msg("Stacked error");
        assert_eq!(format!("{:?}", stacked_error), "Base error\nStacked error");
        assert_eq!(
            stacked_error.err_code(),
            Some(&ErrorCode::RuntimeInvalidValue)
        );
        assert_eq!(stacked_error.err_uri(), Some("https://example.com/base"));
    }
}
