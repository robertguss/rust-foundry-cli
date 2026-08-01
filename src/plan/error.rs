//! Construct / plan errors (pure; stable codes).

use std::fmt;

/// Failure while building a Generation Plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructError {
    /// Machine-oriented error code.
    pub code: &'static str,
    /// Human-readable message.
    pub message: String,
}

impl ConstructError {
    /// Construct error with stable code.
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for ConstructError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ConstructError {}

impl From<crate::resolve::ResolveError> for ConstructError {
    fn from(err: crate::resolve::ResolveError) -> Self {
        Self {
            code: err.code,
            message: err.message,
        }
    }
}
