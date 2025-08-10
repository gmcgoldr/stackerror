//! Conversions from `std::io` types into `StackError`.

use crate::codes::ErrorCode;
use crate::error::{ErrorStacks, StackError};

impl From<std::io::Error> for StackError {
    fn from(error: std::io::Error) -> Self {
        // Capture the kind for mapping before moving the error into the message box
        let kind = error.kind();
        let err = StackError::from_msg(error);
        match ErrorCode::from_io_kind(kind) {
            Some(code) => err.with_err_code(code),
            None => err,
        }
    }
}
