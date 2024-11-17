//! This module contains the traits that are implemented by
//! [`StackError`][crate::StackError].

/// Trait for stacking errors, allowing creation of error chains.
pub trait ErrorStacks {
    fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
}

/// Trait for associating error codes that can be used for runtime error
/// handling.
pub trait ErrorWithCode<T>
where
    T: Send + Sync + 'static + Eq + PartialEq,
{
    fn err_code(&self) -> Option<&T>;
    fn with_err_code(self, code: Option<T>) -> Self;
}

/// Trait for associating URIs with errors for runtime error handling.
pub trait ErrorWithUri {
    fn err_uri(&self) -> Option<&str>;
    fn with_err_uri(self, uri: Option<String>) -> Self;
}

/// This implementation of the [`ErrorStacks`][ErrorStacks] trait for
/// [`Result`][Result] allows you to stack errors on a result.
impl<T, E> ErrorStacks for Result<T, E>
where
    E: ErrorStacks,
{
    fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.stack_err(error))
    }
}
