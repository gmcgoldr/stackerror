use crate::traits::{ErrorCode, ErrorUri, StackError};

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

/// A simple error type that implements the [`StackError`][crate::StackError],
/// [`ErrorCode`][crate::ErrorCode] and [`ErrorUri`][crate::ErrorUri] traits.
pub struct Error {
    error: Box<dyn std::fmt::Display + Send + Sync + 'static>,
    source: Option<Box<Error>>,
    code: Option<ErrorHandling>,
    uri: Option<String>,
}

impl Error {
    /// Creates a new Error from any type that implements Display + Send + Sync.
    pub fn from_error(error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        Self {
            error: Box::new(error),
            source: None,
            code: None,
            uri: None,
        }
    }
}

impl StackError for Error {
    fn stack_error(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        Self {
            error: Box::new(error),
            source: Some(Box::new(self)),
            code: None,
            uri: None,
        }
    }
}

impl ErrorCode<ErrorHandling> for Error {
    fn code(&self) -> Option<&ErrorHandling> {
        self.code.as_ref()
    }

    fn with_code(self, code: Option<ErrorHandling>) -> Self {
        Self { code, ..self }
    }
}

impl ErrorUri for Error {
    fn uri(&self) -> Option<&str> {
        self.uri.as_deref()
    }

    fn with_uri(self, uri: Option<String>) -> Self {
        Self { uri, ..self }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.source {
            Some(source) => write!(f, "{}\n{}", self.error, source),
            None => write!(f, "{}", self.error),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(source) => Some(source.as_ref()),
            None => None,
        }
    }
}
