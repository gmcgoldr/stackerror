//! Prelude module that exports commonly used types and traits.

pub use crate::error::{ErrorCode, StackError};
pub use crate::stack_msg;
pub use crate::traits::{
    ErrorStacks, ErrorStacksWithCode, ErrorStacksWithCodeUri, ErrorStacksWithUri, ErrorWithCode,
    ErrorWithUri,
};
