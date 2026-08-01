//! Spec parse/validate errors.

use std::fmt;

/// Failure while loading or validating a Project Spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecError {
    /// Machine-oriented error code (stable for tests/agents).
    pub code: &'static str,
    /// Human-readable message.
    pub message: String,
    /// Broad class for CLI reporting.
    pub kind: SpecErrorKind,
}

/// High-level error class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecErrorKind {
    /// TOML decode, encoding, or I/O read failure.
    Parse,
    /// Schema / field / denylist / policy failure.
    Validation,
}

impl SpecError {
    /// Construct a parse error.
    pub fn parse(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            kind: SpecErrorKind::Parse,
        }
    }

    /// Construct a validation error.
    pub fn validation(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            kind: SpecErrorKind::Validation,
        }
    }

    /// Error class string for reports (`validation` for both parse/validate).
    pub fn error_class(&self) -> &'static str {
        "validation"
    }
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for SpecError {}
