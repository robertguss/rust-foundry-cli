//! Load Project Spec from path, stdin, or in-memory text (read-only I/O).

use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::spec::error::SpecError;
use crate::spec::model::ProjectSpec;
use crate::spec::validate::validate_raw;

/// Sentinel path for stdin (`--spec -`) per REQ-031.
pub const STDIN_SPEC: &str = "-";

/// Parse and validate a Project Spec from a UTF-8 string.
pub fn parse_spec_str(text: &str, source: impl Into<String>) -> Result<ProjectSpec, SpecError> {
    let value: toml::Value = toml::from_str(text)
        .map_err(|e| SpecError::parse("spec.toml", format!("invalid TOML in Project Spec: {e}")))?;
    let table = value.as_table().ok_or_else(|| {
        SpecError::parse(
            "spec.root_type",
            format!(
                "Project Spec root must be a table, got {}",
                value_type_name(&value)
            ),
        )
    })?;
    validate_raw(table, source.into())
}

/// Load a Project Spec from a filesystem path or stdin.
///
/// Pass `path = "-"` ([`STDIN_SPEC`]) to read the entire stdin stream (REQ-031).
/// Only reads; never writes.
pub fn load_spec(path: impl AsRef<Path>) -> Result<ProjectSpec, SpecError> {
    let path = path.as_ref();
    let path_str = path.to_string_lossy();
    if path_str == STDIN_SPEC {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).map_err(|e| {
            SpecError::parse(
                "spec.read",
                format!("cannot read Project Spec from stdin: {e}"),
            )
        })?;
        return parse_spec_str(&buf, "<stdin>");
    }

    let text = fs::read_to_string(path).map_err(|e| {
        SpecError::parse(
            "spec.read",
            format!("cannot read Project Spec {}: {e}", path.display()),
        )
    })?;
    parse_spec_str(&text, path.display().to_string())
}

fn value_type_name(value: &toml::Value) -> &'static str {
    match value {
        toml::Value::String(_) => "string",
        toml::Value::Integer(_) => "integer",
        toml::Value::Float(_) => "float",
        toml::Value::Boolean(_) => "boolean",
        toml::Value::Datetime(_) => "datetime",
        toml::Value::Array(_) => "array",
        toml::Value::Table(_) => "table",
    }
}
