//! Conversions from `http` types into `StackError`.

use crate::codes::ErrorCode;
use crate::error::{ErrorStacks, StackError};

impl From<http::StatusCode> for StackError {
    fn from(status: http::StatusCode) -> Self {
        let code = ErrorCode::from_http_value(status.as_u16());
        let err = StackError::from_msg(status);
        match code {
            Some(mapped) => err.with_err_code(mapped),
            None => err,
        }
    }
}
