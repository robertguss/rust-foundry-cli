//! Normalize Project Spec + CLI overrides into Construct effective inputs.
//!
//! REQ-034: CLI flags win over TOML; applied before pure Construct.
//! REQ-040: validate/plan/generate share one effective-input shape.

use crate::spec::error::SpecError;
use crate::spec::model::{ProjectSpec, VerifyMode};

/// Documented default verify mode when neither TOML nor CLI sets `verify`.
///
/// Generate records this as the effective post-stage policy (REQ-120: default
/// primary gate). Plan and validate use the same effective mode so
/// plan-as-contract (REQ-040) stays enforceable across commands.
pub const DEFAULT_VERIFY_MODE: VerifyMode = VerifyMode::Default;

/// Optional CLI field overrides (REQ-034). Present flags win over TOML.
///
/// Profile membership has no CLI override in v1.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CliOverrides {
    /// Override TOML `name` when `Some`.
    pub name: Option<String>,
    /// Override TOML `destination` when `Some`.
    pub dest: Option<String>,
    /// Override TOML `verify` when `Some`.
    pub verify: Option<VerifyMode>,
}

/// Normalized effective inputs — the single Construct input for
/// validate / plan / generate after parse + overrides + verify defaulting.
///
/// Pure value type: no FS paths are touched here; `destination` remains a
/// string for later resolve/construct/fsx stages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveInputs {
    /// Schema version (always 1 when valid).
    pub schema: i64,
    /// Effective project / crate name (CLI or TOML).
    pub name: String,
    /// Optional free-text description (TOML only in v1).
    pub description: Option<String>,
    /// Archetype id (`cli` only in v1).
    pub archetype: String,
    /// Effective destination path string (CLI or TOML).
    pub destination: String,
    /// Selected profile ids (input order; canonical order is resolve-time).
    pub profiles: Vec<String>,
    /// Effective verify mode — always set after normalization.
    pub verify: VerifyMode,
    /// Provenance label for diagnostics (path or `<stdin>` / `<string>`).
    pub source: String,
}

impl EffectiveInputs {
    /// Build from a validated [`ProjectSpec`] after overrides have been applied
    /// onto that value (name/dest/verify fields already effective).
    fn from_spec_with_verify(spec: ProjectSpec, verify: VerifyMode) -> Self {
        Self {
            schema: spec.schema,
            name: spec.name,
            description: spec.description,
            archetype: spec.archetype,
            destination: spec.destination,
            profiles: spec.profiles,
            verify,
            source: spec.source,
        }
    }
}

/// Apply CLI overrides and produce normalized [`EffectiveInputs`] for Construct.
///
/// Pure: no filesystem reads or writes.
///
/// Rules:
/// - Present `--name` / `--dest` / `--verify` win over the corresponding TOML field.
/// - Empty name or dest override is rejected (`spec.empty_field`).
/// - When verify is omitted from both TOML and CLI, uses [`DEFAULT_VERIFY_MODE`].
pub fn normalize_effective_inputs(
    mut spec: ProjectSpec,
    overrides: CliOverrides,
) -> Result<EffectiveInputs, SpecError> {
    if let Some(n) = overrides.name {
        let n = n.trim();
        if n.is_empty() {
            return Err(SpecError::validation(
                "spec.empty_field",
                "CLI --name must be a non-empty string",
            ));
        }
        spec.name = n.to_string();
    }
    if let Some(d) = overrides.dest {
        let d = d.trim();
        if d.is_empty() {
            return Err(SpecError::validation(
                "spec.empty_field",
                "CLI --dest must be a non-empty string",
            ));
        }
        spec.destination = d.to_string();
    }

    let verify = match overrides.verify {
        Some(v) => v,
        None => spec.verify.unwrap_or(DEFAULT_VERIFY_MODE),
    };

    Ok(EffectiveInputs::from_spec_with_verify(spec, verify))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::parse::parse_spec_str;

    fn minimal_toml() -> &'static str {
        r#"
schema = 1
name = "from-toml"
archetype = "cli"
destination = "./from-toml"
profiles = []
"#
    }

    fn with_verify_toml() -> &'static str {
        r#"
schema = 1
name = "from-toml"
archetype = "cli"
destination = "./from-toml"
profiles = []
verify = "none"
"#
    }

    #[test]
    fn name_dest_verify_flags_win_over_toml() {
        let spec = parse_spec_str(with_verify_toml(), "<t>").unwrap();
        let effective = normalize_effective_inputs(
            spec,
            CliOverrides {
                name: Some("renamed".into()),
                dest: Some("./elsewhere".into()),
                verify: Some(VerifyMode::Strict),
            },
        )
        .unwrap();
        assert_eq!(effective.name, "renamed");
        assert_eq!(effective.destination, "./elsewhere");
        assert_eq!(effective.verify, VerifyMode::Strict);
    }

    #[test]
    fn missing_optional_verify_defaults_to_default_mode() {
        let spec = parse_spec_str(minimal_toml(), "<t>").unwrap();
        assert!(spec.verify.is_none());
        let effective = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
        assert_eq!(effective.verify, DEFAULT_VERIFY_MODE);
        assert_eq!(effective.verify, VerifyMode::Default);
    }

    #[test]
    fn toml_verify_preserved_when_no_cli_override() {
        let spec = parse_spec_str(with_verify_toml(), "<t>").unwrap();
        let effective = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
        assert_eq!(effective.verify, VerifyMode::None);
    }

    #[test]
    fn empty_name_override_rejected() {
        let spec = parse_spec_str(minimal_toml(), "<t>").unwrap();
        let err = normalize_effective_inputs(
            spec,
            CliOverrides {
                name: Some("   ".into()),
                ..Default::default()
            },
        )
        .unwrap_err();
        assert_eq!(err.code, "spec.empty_field");
    }

    #[test]
    fn empty_dest_override_rejected() {
        let spec = parse_spec_str(minimal_toml(), "<t>").unwrap();
        let err = normalize_effective_inputs(
            spec,
            CliOverrides {
                dest: Some(String::new()),
                ..Default::default()
            },
        )
        .unwrap_err();
        assert_eq!(err.code, "spec.empty_field");
    }

    #[test]
    fn pure_no_side_effects_on_profiles() {
        let body = r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = ["hooks", "tui"]
"#;
        let spec = parse_spec_str(body, "<t>").unwrap();
        let effective = normalize_effective_inputs(
            spec,
            CliOverrides {
                name: Some("y".into()),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            effective.profiles,
            vec!["hooks".to_string(), "tui".to_string()]
        );
        assert_eq!(effective.archetype, "cli");
        assert_eq!(effective.schema, 1);
    }
}
