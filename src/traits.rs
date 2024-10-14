//! This module contains the traits that are implemented by [`stackerror::Error`][crate::Error].

/// Trait for stacking errors, allowing creation of error chains.
pub trait StackError {
    fn stack_error(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
}

/// Trait for associating error codes that can be used for runtime error handling.
pub trait ErrorCode<T>
where
    T: Send + Sync + 'static + Eq + PartialEq,
{
    fn code(&self) -> Option<&T>;
    fn with_code(self, code: Option<T>) -> Self;
}

/// Trait for associating URIs with errors for runtime error handling.
pub trait ErrorUri {
    fn uri(&self) -> Option<&str>;
    fn with_uri(self, uri: Option<String>) -> Self;
}

/// This implementation of the [`StackError`][StackError] trait for [`std::result::Result`][Result] allows you to stack errors on a result.
impl<T, E> StackError for Result<T, E>
where
    E: StackError,
{
    fn stack_error(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.stack_error(error))
    }
}

/// This implementation of the [`ErrorCode`][ErrorCode] trait for [`std::result::Result`][Result] allows you to associate an error code with a result.
impl<T, E, C> ErrorCode<C> for Result<T, E>
where
    C: Send + Sync + 'static + Eq + PartialEq,
    E: ErrorCode<C>,
{
    fn code(&self) -> Option<&C> {
        self.as_ref().err().and_then(|e| e.code())
    }

    fn with_code(self, code: Option<C>) -> Self {
        self.map_err(|e| e.with_code(code))
    }
}

/// This implementation of the [`ErrorUri`][ErrorUri] trait for [`std::result::Result`][Result] allows you to associate a URI with a result.
impl<T, E> ErrorUri for Result<T, E>
where
    E: ErrorUri,
{
    fn uri(&self) -> Option<&str> {
        self.as_ref().err().and_then(|e| e.uri())
    }

    fn with_uri(self, uri: Option<String>) -> Self {
        self.map_err(|e| e.with_uri(uri))
    }
}
