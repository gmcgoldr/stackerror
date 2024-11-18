//! This module contains the traits that are implemented by
//! [`StackError`][crate::StackError].

/// Trait for associating error codes that can be used for runtime error
/// handling.
pub trait ErrorWithCode<C>
where
    C: Send + Sync + 'static + Eq + PartialEq,
{
    fn err_code(&self) -> Option<&C>;
    fn with_err_code(self, code: Option<C>) -> Self;
}

/// Trait for associating URIs with errors for runtime error handling.
pub trait ErrorWithUri {
    fn err_uri(&self) -> Option<&str>;
    fn with_err_uri(self, uri: Option<String>) -> Self;
}

/// Trait for stacking errors, allowing creation of error chains.
pub trait ErrorStacks {
    fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
}

/// Implementation for [`Result`] allows chaining directly on results.
impl<T, E> ErrorStacks for Result<T, E>
where
    E: ErrorStacks,
{
    fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.stack_err(error))
    }
}

/// Trait for stacking errors preserving the code.
pub trait ErrorStacksWithCode {
    fn stack_err_code(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
}

/// Implementation for [`Result`] allows chaining directly on results.
impl<T, E> ErrorStacksWithCode for Result<T, E>
where
    E: ErrorStacksWithCode,
{
    fn stack_err_code(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.stack_err_code(error))
    }
}

/// Trait for stacking errors preserving the URI.
pub trait ErrorStacksWithUri {
    fn stack_err_uri(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
}

/// Implementation for [`Result`] allows chaining directly on results.
impl<T, E> ErrorStacksWithUri for Result<T, E>
where
    E: ErrorStacksWithUri,
{
    fn stack_err_uri(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.stack_err_uri(error))
    }
}

/// Trait for stacking errors preserving the code and URI.
pub trait ErrorStacksWithCodeUri {
    fn stack_err_code_uri(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
}

/// Implementation for [`Result`] allows chaining directly on results.
impl<T, E> ErrorStacksWithCodeUri for Result<T, E>
where
    E: ErrorStacksWithCodeUri,
{
    fn stack_err_code_uri(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.stack_err_code_uri(error))
    }
}
