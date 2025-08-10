#![doc = include_str!("../README.md")]

pub mod codes;
pub mod error;
#[cfg(feature = "http")]
mod from_http;
#[cfg(feature = "reqwest")]
mod from_reqwest;
mod from_std_io;
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

    #[test]
    fn test_from_std_io_for_stackerror() {
        let io_err = std::io::Error::from(std::io::ErrorKind::NotFound);
        let err: StackError = io_err.into();
        assert_eq!(err.err_code(), Some(&ErrorCode::IoNotFound));
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_from_http_status_for_stackerror() {
        let status = http::StatusCode::NOT_FOUND;
        let err: StackError = status.into();
        assert_eq!(err.err_code(), Some(&ErrorCode::HttpNotFound));
    }

    #[cfg(feature = "reqwest")]
    #[test]
    fn test_from_reqwest_error_for_stackerror() {
        // Build-time request error (invalid header) -> no HTTP status
        let client = reqwest::Client::builder().build().unwrap();
        let req_err = client
            .get("http://example.com")
            .header("\n", "value")
            .build()
            .unwrap_err();
        let err: StackError = req_err.into();
        assert_eq!(err.err_code(), None);
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

    #[test]
    fn test_from_std_io_for_custom_error() {
        let io_err = std::io::Error::from(std::io::ErrorKind::PermissionDenied);
        let err: LibError = io_err.into();
        assert_eq!(err.err_code(), Some(&ErrorCode::IoPermissionDenied));
    }

    // NOTE: don't need to test other from impls in custom error since they
    // are handled by a generic impl block
}
