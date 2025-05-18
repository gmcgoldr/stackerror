//! This module provides the [`StackError`] struct which implements the
//! [`ErrorStacks`] trait.

use crate::codes::ErrorCode;

/// Trait for stacking errors: errors that stack and provide an optional error
/// code and resource URI for runtime error handling.
pub trait ErrorStacks<C>
where
    C: Send + Sync + 'static + Eq + PartialEq + Clone,
{
    /// Get the error code if one is set.
    fn err_code(&self) -> Option<&C>;
    /// Set the error code.
    fn with_err_code(self, code: Option<C>) -> Self;
    /// Get the error URI if one is set.
    fn err_uri(&self) -> Option<&str>;
    /// Set the error URI.
    fn with_err_uri(self, uri: Option<String>) -> Self;
    /// Stack a new error on the current one.
    fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
}

/// Implementation for [`Result`] allows adding error codes on results.
impl<T, E, C> ErrorStacks<C> for Result<T, E>
where
    C: Send + Sync + 'static + Eq + PartialEq + Clone,
    E: ErrorStacks<C>,
{
    fn err_code(&self) -> Option<&C> {
        self.as_ref().err().and_then(|e| e.err_code())
    }

    fn with_err_code(self, code: Option<C>) -> Self {
        self.map_err(|e| e.with_err_code(code))
    }

    fn err_uri(&self) -> Option<&str> {
        self.as_ref().err().and_then(|e| e.err_uri())
    }

    fn with_err_uri(self, uri: Option<String>) -> Self {
        self.map_err(|e| e.with_err_uri(uri))
    }

    fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.stack_err(error))
    }
}

/// A simple error type that implements the [`ErrorStacks`] [`ErrorWithCode`]
/// and [`ErrorWithUri`] traits.
pub struct StackError {
    error: Box<dyn std::fmt::Display + Send + Sync + 'static>,
    source: Option<Box<StackError>>,
    code: Option<ErrorCode>,
    uri: Option<String>,
    level: usize,
}

impl StackError {
    /// Creates a new StackError from any type that implements Display + Send + Sync.
    pub fn new(error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        Self {
            error: Box::new(error),
            source: None,
            code: None,
            uri: None,
            level: 0,
        }
    }
}

impl ErrorStacks<ErrorCode> for StackError {
    fn err_code(&self) -> Option<&ErrorCode> {
        self.code.as_ref()
    }

    fn with_err_code(self, code: Option<ErrorCode>) -> Self {
        Self { code, ..self }
    }

    fn err_uri(&self) -> Option<&str> {
        self.uri.as_deref()
    }

    fn with_err_uri(self, uri: Option<String>) -> Self {
        Self { uri, ..self }
    }

    fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        let code = self.code;
        let uri = self.uri.clone();
        let level = self.level + 1;
        Self {
            error: Box::new(error),
            source: Some(Box::new(self)),
            code,
            uri,
            level,
        }
    }
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.source {
            Some(source) => write!(f, "{}\n{}", source, self.error),
            None => write!(f, "{}", self.error),
        }
    }
}

impl std::fmt::Debug for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for StackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(source) => Some(source.as_ref()),
            None => None,
        }
    }
}
