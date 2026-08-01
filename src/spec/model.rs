//! Validated Project Spec model (schema = 1).

/// Supported Project Spec schema version (v1).
pub const SUPPORTED_SCHEMA: i64 = 1;

/// Closed archetype set for v1 (REQ-032): exactly `cli`.
pub const ARCHETYPES: &[&str] = &["cli"];

/// Closed profile ids (revised-spec §11.3).
pub const PROFILES: &[&str] = &["tui", "hooks", "secrets", "distribution"];

/// Allowed verify modes (§11.1).
pub const VERIFY_MODES: &[&str] = &["none", "default", "strict"];

/// Required top-level keys.
pub const REQUIRED_KEYS: &[&str] = &["schema", "name", "archetype", "destination", "profiles"];

/// Allowed top-level keys (required + optional).
pub const ALLOWED_KEYS: &[&str] = &[
    "schema",
    "name",
    "description",
    "archetype",
    "destination",
    "profiles",
    "verify",
];

/// Verify mode after optional defaulting for Construct (TOML may omit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyMode {
    /// Skip verify.
    None,
    /// Default primary gate (PHASE-03 semantics).
    Default,
    /// Strict tier.
    Strict,
}

impl VerifyMode {
    /// Parse a verify mode string.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "default" => Some(Self::Default),
            "strict" => Some(Self::Strict),
            _ => None,
        }
    }

    /// Canonical string form.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Default => "default",
            Self::Strict => "strict",
        }
    }
}

/// Immutable validated Project Spec (schema = 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectSpec {
    /// Schema version (always 1 when valid).
    pub schema: i64,
    /// Project / crate name.
    pub name: String,
    /// Optional free-text description.
    pub description: Option<String>,
    /// Archetype id (`cli` only in v1).
    pub archetype: String,
    /// Destination path string from the spec (or override).
    pub destination: String,
    /// Selected profile ids (input order preserved; canonical order is resolve-time).
    pub profiles: Vec<String>,
    /// Optional verify mode from TOML (or override).
    pub verify: Option<VerifyMode>,
    /// Provenance label for diagnostics (path or `<stdin>` / `<string>`).
    pub source: String,
}
