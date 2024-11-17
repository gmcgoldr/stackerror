//! This module provides the [`StackError`] struct which implements the error
//! traits provided by this crate.

use crate::traits::{ErrorStacks, ErrorWithCode, ErrorWithUri};

/// Error handling codes.
///
/// Suggests to the caller how an error could be handled.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ErrorHandling {
    /// An input to the function is invalid.
    ValidateInputs,
    /// A resource required by the function is invalid.
    ValidateResource,
    /// A resource required by the function isn't currently available, but it
    /// could be in the future.
    RetryResource,
    /// A resource required by the function isn't available.
    BypassResource,
}

/// A simple error type that implements the [`ErrorStacks`] [`ErrorWithCode`]
/// and [`ErrorWithUri`] traits.
pub struct StackError {
    error: Box<dyn std::fmt::Display + Send + Sync + 'static>,
    source: Option<Box<StackError>>,
    code: Option<ErrorHandling>,
    uri: Option<String>,
}

impl StackError {
    /// Creates a new StackError from any type that implements Display + Send + Sync.
    pub fn new(error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        Self {
            error: Box::new(error),
            source: None,
            code: None,
            uri: None,
        }
    }
}

impl ErrorStacks for StackError {
    fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        Self {
            error: Box::new(error),
            source: Some(Box::new(self)),
            code: None,
            uri: None,
        }
    }
}

impl ErrorWithCode<ErrorHandling> for StackError {
    fn err_code(&self) -> Option<&ErrorHandling> {
        self.code.as_ref()
    }

    fn with_err_code(self, code: Option<ErrorHandling>) -> Self {
        Self { code, ..self }
    }
}

impl ErrorWithUri for StackError {
    fn err_uri(&self) -> Option<&str> {
        self.uri.as_deref()
    }

    fn with_err_uri(self, uri: Option<String>) -> Self {
        Self { uri, ..self }
    }
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.source {
            Some(source) => write!(f, "{}\n{}", self.error, source),
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
