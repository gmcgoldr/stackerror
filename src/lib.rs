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
    use crate::error::ErrorStacks; // Already have StackError in prelude
    use std::io;

    #[cfg(feature = "feat_reqwest")]
    use reqwest;

    #[cfg(feature = "feat_axum")]
    use axum::BoxError;

    #[cfg(feature = "feat_actix")]
    use actix_web;

    #[cfg(feature = "feat_http")]
    use http;

    #[test]
    fn test_error_builds() {
        let error = StackError::new("Test error");
        assert_eq!(error.to_string(), "Test error");
    }

    #[test]
    fn test_error_has_code() {
        let error =
            StackError::new("Test error").with_err_code(Some(ErrorCode::RuntimeInvalidValue));
        assert_eq!(error.err_code(), Some(&ErrorCode::RuntimeInvalidValue));
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
            .with_err_code(Some(ErrorCode::RuntimeInvalidValue))
            .with_err_uri(Some("https://example.com/base".to_string()));
        let stacked_error = base_error.stack_err("Stacked error");
        assert_eq!(stacked_error.to_string(), "Base error\nStacked error");
        assert_eq!(
            stacked_error.err_code(),
            Some(&ErrorCode::RuntimeInvalidValue)
        );
        assert_eq!(stacked_error.err_uri(), Some("https://example.com/base"));
    }

    #[test]
    fn test_stack_map_macro_maps() {
        let error: Result<(), StackError> =
            Err(StackError::new("Base error")).map_err(stack_map!(StackError, "Stacked error"));
        let error = error.unwrap_err();
        assert!(error.to_string().contains("Base error"));
        assert!(error.to_string().contains("Stacked error"));
    }

    #[test]
    fn test_stack_else_macro_builds() {
        let error: Result<(), StackError> =
            Option::None.ok_or_else(stack_else!(StackError, "Base error"));
        let error = error.unwrap_err();
        assert!(error.to_string().contains("Base error"));
    }

    #[test]
    fn test_stack_err_macro_builds() {
        let error: Result<(), StackError> = stack_err!(StackError, "Test error");
        let error = error.unwrap_err();
        assert!(error.to_string().contains("Test error"));
    }

    // Add this custom error struct
    #[derive_stack_error]
    struct LibError(StackError);

    #[test]
    fn test_derived_builds() {
        let custom_error = LibError::new("Custom error");
        assert_eq!(custom_error.to_string(), "Custom error");
    }

    #[test]
    fn test_derived_has_code() {
        let coded_error =
            LibError::new("Coded error").with_err_code(Some(ErrorCode::RuntimeInvalidValue));
        assert_eq!(
            coded_error.err_code(),
            Some(&ErrorCode::RuntimeInvalidValue)
        );
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
            .with_err_code(Some(ErrorCode::RuntimeInvalidValue))
            .with_err_uri(Some("https://example.com/base_custom".to_string()));
        let stacked_error = base_error.stack_err("Stacked error");
        assert_eq!(stacked_error.to_string(), "Base error\nStacked error");
        assert_eq!(
            stacked_error.err_code(),
            Some(&ErrorCode::RuntimeInvalidValue)
        );
        assert_eq!(
            stacked_error.err_uri(),
            Some("https://example.com/base_custom")
        );
    }

    #[test]
    fn test_derived_creation_map() {
        let error: Result<(), LibError> =
            Err(LibError::new("Base error")).map_err(stack_map!(LibError, "Stacked error"));
        assert!(error.unwrap_err().to_string().ends_with("Stacked error"));
    }

    #[test]
    fn test_derived_creation_fn() {
        let error: Result<(), LibError> =
            Option::None.ok_or_else(stack_else!(LibError, "Base error"));
        assert!(error.unwrap_err().to_string().ends_with("Base error"));
    }

    #[test]
    fn test_from_std_io_error() {
        let original_error_msg = "test io error";
        let io_error = io::Error::new(io::ErrorKind::Other, original_error_msg);
        let stack_error = StackError::from(io_error);
        assert!(stack_error.to_string().contains(original_error_msg));
    }

    #[cfg(feature = "feat_reqwest")]
    #[test]
    fn test_from_reqwest_error() {
        // Create a reqwest error. This attempts a connection to a port that is likely not listened on.
        // This requires the `blocking` feature of `reqwest` to be available by default or enabled.
        // The `reqwest` dependency in Cargo.toml is just `reqwest = { version = "0.11", optional = true }`
        // which means default features (including blocking) should be enabled.
        let original_error = reqwest::blocking::Client::new().get("http://127.0.0.1:1").send().unwrap_err();
        let original_error_msg = original_error.to_string();
        let stack_error = StackError::from(original_error);
        assert!(stack_error.to_string().contains(&original_error_msg));
    }

    #[cfg(feature = "feat_axum")]
    #[test]
    fn test_from_axum_box_error() {
        #[derive(Debug)]
        struct MyAxumError(&'static str);
        impl std::fmt::Display for MyAxumError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::error::Error for MyAxumError {}

        let original_error_msg = "test axum error";
        let axum_error: BoxError = Box::new(MyAxumError(original_error_msg));
        let stack_error = StackError::from(axum_error);
        assert!(stack_error.to_string().contains(original_error_msg));
    }

    #[cfg(feature = "feat_actix")]
    #[test]
    fn test_from_actix_web_error() {
        let original_error_msg = "test actix error";
        let actix_error = actix_web::error::ErrorInternalServerError(original_error_msg);
        // actix_web::Error::ErrorInternalServerError creates an error that includes the status code message,
        // so we check if the supplied message is part of the resulting StackError string.
        let stack_error = StackError::from(actix_error);
        assert!(stack_error.to_string().contains(original_error_msg));
    }

    #[cfg(feature = "feat_http")]
    #[test]
    fn test_from_http_status_code() {
        let status_code = http::StatusCode::NOT_FOUND;
        let stack_error = StackError::from(status_code);
        assert!(stack_error.to_string().contains("404 Not Found"));
    }
}
