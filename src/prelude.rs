//! Prelude module that exports commonly used types and traits.

pub use crate::codes::ErrorCode;
pub use crate::error::{ErrorStacks, StackError};
pub use crate::{stack_else, stack_err, stack_map, stack_msg};
pub type StackResult<T> = std::result::Result<T, StackError>;
