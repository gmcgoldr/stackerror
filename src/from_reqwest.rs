//! Conversions from `reqwest` types into `StackError`.

use crate::error::{ErrorStacks, StackError};

impl From<reqwest::Error> for StackError {
    fn from(error: reqwest::Error) -> Self {
        // If there's an HTTP status, use the From<http::StatusCode> impl to set the code,
        // then stack the reqwest error message on top to preserve context.
        if let Some(status) = error.status() {
            let base: StackError = status.into();
            return base.stack_err_msg(error);
        }
        StackError::from_msg(error)
    }
}
