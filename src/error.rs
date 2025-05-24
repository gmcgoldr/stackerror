//! Provides the [`StackError`] struct which implements the [`ErrorStacks`]
//! trait.

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
    fn with_err_code(self, code: C) -> Self;
    /// Remove the error code.
    fn with_no_err_code(self) -> Self;
    /// Get the error URI if one is set.
    fn err_uri(&self) -> Option<&str>;
    /// Set the error URI.
    fn with_err_uri(self, uri: String) -> Self;
    /// Remove the error URI.
    fn with_no_err_uri(self) -> Self;
    /// Set the error message.
    fn with_err_msg(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
    /// Remove the error message.
    fn with_no_err_msg(self) -> Self;
    /// Stack a new error on the current one.
    fn stack_err(self) -> Self;
    /// Stack a new error on the current one with a given message.
    fn stack_err_msg(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self;
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

    fn with_err_code(self, code: C) -> Self {
        self.map_err(|e| e.with_err_code(code))
    }

    fn with_no_err_code(self) -> Self {
        self.map_err(|e| e.with_no_err_code())
    }

    fn err_uri(&self) -> Option<&str> {
        self.as_ref().err().and_then(|e| e.err_uri())
    }

    fn with_err_uri(self, uri: String) -> Self {
        self.map_err(|e| e.with_err_uri(uri))
    }

    fn with_no_err_uri(self) -> Self {
        self.map_err(|e| e.with_no_err_uri())
    }

    fn with_err_msg(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.with_err_msg(error))
    }

    fn with_no_err_msg(self) -> Self {
        self.map_err(|e| e.with_no_err_msg())
    }

    fn stack_err(self) -> Self {
        self.map_err(|e| e.stack_err())
    }

    fn stack_err_msg(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        self.map_err(|e| e.stack_err_msg(error))
    }
}

/// A simple error type that implements the [`ErrorStacks`] trait.
#[derive(Default)]
pub struct StackError {
    message: Option<Box<dyn std::fmt::Display + Send + Sync + 'static>>,
    source: Option<Box<StackError>>,
    code: Option<ErrorCode>,
    uri: Option<String>,
}

impl StackError {
    /// Creates a new empty StackError.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new StackError from any error message that implements
    /// Display + Send + Sync.
    pub fn from_msg(error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        Self {
            message: Some(Box::new(error)),
            ..Default::default()
        }
    }
}

impl ErrorStacks<ErrorCode> for StackError {
    fn err_code(&self) -> Option<&ErrorCode> {
        self.code.as_ref()
    }

    fn with_err_code(self, code: ErrorCode) -> Self {
        Self {
            code: Some(code),
            ..self
        }
    }

    fn with_no_err_code(self) -> Self {
        Self { code: None, ..self }
    }

    fn err_uri(&self) -> Option<&str> {
        self.uri.as_deref()
    }

    fn with_err_uri(self, uri: String) -> Self {
        Self {
            uri: Some(uri),
            ..self
        }
    }

    fn with_no_err_uri(self) -> Self {
        Self { uri: None, ..self }
    }

    fn with_err_msg(self, message: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        Self {
            message: Some(Box::new(message)),
            ..self
        }
    }

    fn with_no_err_msg(self) -> Self {
        Self {
            message: None,
            ..self
        }
    }

    fn stack_err(self) -> Self {
        let code = self.code;
        let uri = self.uri.clone();
        Self {
            message: None,
            source: Some(Box::new(self)),
            code,
            uri,
        }
    }

    fn stack_err_msg(self, message: impl std::fmt::Display + Send + Sync + 'static) -> Self {
        let code = self.code;
        let uri = self.uri.clone();
        Self {
            message: Some(Box::new(message)),
            source: Some(Box::new(self)),
            code,
            uri,
        }
    }
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.message {
            Some(error) => {
                write!(f, "{}", error)
            }
            None => Ok(()),
        }
    }
}

impl std::fmt::Debug for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, err) in std::iter::successors(Some(self), |e| e.source.as_deref())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .enumerate()
        {
            if idx > 0 {
                writeln!(f)?;
            }
            write!(f, "{err}")?;
        }
        Ok(())
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
