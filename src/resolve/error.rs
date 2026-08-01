//! Resolve errors (pure; stable codes).

use std::fmt;

/// Failure while resolving archetype / profile composition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveError {
    /// Machine-oriented error code.
    pub code: &'static str,
    /// Human-readable message.
    pub message: String,
}

impl ResolveError {
    /// Construct a resolve validation error.
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ResolveError {}
