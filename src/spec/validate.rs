//! Validate a decoded Project Spec table (pure; no I/O).

use std::collections::BTreeSet;

use toml::Value;
use toml::map::Map;

use crate::spec::denylist::field_name_is_denied;
use crate::spec::error::SpecError;
use crate::spec::model::{
    ALLOWED_KEYS, ARCHETYPES, PROFILES, ProjectSpec, REQUIRED_KEYS, SUPPORTED_SCHEMA, VERIFY_MODES,
    VerifyMode,
};

/// Validate a top-level TOML table and return an immutable [`ProjectSpec`].
pub fn validate_raw(table: &Map<String, Value>, source: String) -> Result<ProjectSpec, SpecError> {
    reject_denied_field_names(table, "")?;

    let keys: BTreeSet<&str> = table.keys().map(String::as_str).collect();
    let allowed: BTreeSet<&str> = ALLOWED_KEYS.iter().copied().collect();
    let unknown: Vec<&str> = keys.difference(&allowed).copied().collect();
    if !unknown.is_empty() {
        let list = unknown
            .iter()
            .map(|k| format!("{k:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(SpecError::validation(
            "spec.unknown_key",
            format!("unknown top-level key(s): {list}"),
        ));
    }

    let required: BTreeSet<&str> = REQUIRED_KEYS.iter().copied().collect();
    let missing: Vec<&str> = required.difference(&keys).copied().collect();
    if !missing.is_empty() {
        let list = missing
            .iter()
            .map(|k| format!("{k:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(SpecError::validation(
            "spec.missing_field",
            format!("missing required field(s): {list}"),
        ));
    }

    let schema = require_schema(table.get("schema").expect("schema present"))?;
    let name = require_nonempty_str(table.get("name").expect("name present"), "name")?;
    let archetype = require_archetype(table.get("archetype").expect("archetype present"))?;
    let destination = require_nonempty_str(
        table.get("destination").expect("destination present"),
        "destination",
    )?;
    let profiles = require_profiles(table.get("profiles").expect("profiles present"))?;

    let description = match table.get("description") {
        Some(v) => Some(require_str(v, "description")?),
        None => None,
    };

    let verify = match table.get("verify") {
        Some(v) => Some(require_verify(v)?),
        None => None,
    };

    Ok(ProjectSpec {
        schema,
        name,
        description,
        archetype,
        destination,
        profiles,
        verify,
        source,
    })
}

/// Apply optional CLI overrides (REQ-034) to produce effective inputs.
///
/// Present flags win over TOML fields. Profiles have no CLI override in v1.
pub fn apply_overrides(
    mut spec: ProjectSpec,
    name: Option<String>,
    dest: Option<String>,
    verify: Option<VerifyMode>,
) -> Result<ProjectSpec, SpecError> {
    if let Some(n) = name {
        let n = n.trim();
        if n.is_empty() {
            return Err(SpecError::validation(
                "spec.empty_field",
                "CLI --name must be a non-empty string",
            ));
        }
        spec.name = n.to_string();
    }
    if let Some(d) = dest {
        let d = d.trim();
        if d.is_empty() {
            return Err(SpecError::validation(
                "spec.empty_field",
                "CLI --dest must be a non-empty string",
            ));
        }
        spec.destination = d.to_string();
    }
    if let Some(v) = verify {
        spec.verify = Some(v);
    }
    Ok(spec)
}

fn require_schema(value: &Value) -> Result<i64, SpecError> {
    let Some(n) = value.as_integer() else {
        return Err(SpecError::validation(
            "spec.schema_type",
            format!("schema must be integer {SUPPORTED_SCHEMA}, got {value}"),
        ));
    };
    if n != SUPPORTED_SCHEMA {
        return Err(SpecError::validation(
            "spec.unsupported_schema",
            format!("unsupported schema = {n}; supported set is {{{SUPPORTED_SCHEMA}}}"),
        ));
    }
    Ok(n)
}

fn require_str(value: &Value, field: &str) -> Result<String, SpecError> {
    value.as_str().map(str::to_string).ok_or_else(|| {
        SpecError::validation(
            "spec.field_type",
            format!("{field} must be a string, got {}", type_name(value)),
        )
    })
}

fn require_nonempty_str(value: &Value, field: &str) -> Result<String, SpecError> {
    let text = require_str(value, field)?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(SpecError::validation(
            "spec.empty_field",
            format!("{field} must be a non-empty string"),
        ));
    }
    Ok(trimmed.to_string())
}

fn require_archetype(value: &Value) -> Result<String, SpecError> {
    let text = require_nonempty_str(value, "archetype")?;
    if !ARCHETYPES.contains(&text.as_str()) {
        let allowed = ARCHETYPES.join(", ");
        return Err(SpecError::validation(
            "spec.unknown_archetype",
            format!("unknown archetype {text:?}; must be one of: {allowed}"),
        ));
    }
    Ok(text)
}

fn require_profiles(value: &Value) -> Result<Vec<String>, SpecError> {
    let Some(arr) = value.as_array() else {
        return Err(SpecError::validation(
            "spec.profiles_type",
            format!("profiles must be an array, got {}", type_name(value)),
        ));
    };

    let mut profiles = Vec::with_capacity(arr.len());
    let mut seen = BTreeSet::new();
    for (index, item) in arr.iter().enumerate() {
        let Some(raw) = item.as_str() else {
            return Err(SpecError::validation(
                "spec.profile_type",
                format!("profiles[{index}] must be a non-empty string"),
            ));
        };
        let profile_id = raw.trim();
        if profile_id.is_empty() {
            return Err(SpecError::validation(
                "spec.profile_type",
                format!("profiles[{index}] must be a non-empty string"),
            ));
        }
        if !seen.insert(profile_id.to_string()) {
            return Err(SpecError::validation(
                "spec.duplicate_profile",
                format!("profiles lists duplicate ID {profile_id:?}"),
            ));
        }
        if !PROFILES.contains(&profile_id) {
            let allowed = PROFILES.join(", ");
            return Err(SpecError::validation(
                "spec.unknown_profile",
                format!("unknown profile {profile_id:?}; must be one of: {allowed}"),
            ));
        }
        profiles.push(profile_id.to_string());
    }
    Ok(profiles)
}

fn require_verify(value: &Value) -> Result<VerifyMode, SpecError> {
    let text = require_nonempty_str(value, "verify")?;
    VerifyMode::parse(&text).ok_or_else(|| {
        let allowed = VERIFY_MODES.join(", ");
        SpecError::validation(
            "spec.verify_mode",
            format!("verify must be one of: {allowed} (got {text:?})"),
        )
    })
}

/// Walk all tables and reject denied field names (case-insensitive).
fn reject_denied_field_names(table: &Map<String, Value>, path: &str) -> Result<(), SpecError> {
    for (key, value) in table {
        if field_name_is_denied(key) {
            let where_ = if path.is_empty() {
                format!("top-level field {key:?}")
            } else {
                format!("field {path}.{key}")
            };
            return Err(SpecError::validation(
                "spec.secret_field",
                format!(
                    "{where_} matches secret field denylist (REQ-033); \
                     forbidden names: {}",
                    crate::spec::SECRET_FIELD_DENYLIST.join(", ")
                ),
            ));
        }
        match value {
            Value::Table(nested) => {
                let child = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                reject_denied_field_names(nested, &child)?;
            }
            Value::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    if let Value::Table(nested) = item {
                        let child = if path.is_empty() {
                            format!("{key}[{i}]")
                        } else {
                            format!("{path}.{key}[{i}]")
                        };
                        reject_denied_field_names(nested, &child)?;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn type_name(value: &Value) -> &'static str {
    match value {
        Value::String(_) => "string",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::Boolean(_) => "boolean",
        Value::Datetime(_) => "datetime",
        Value::Array(_) => "array",
        Value::Table(_) => "table",
    }
}
