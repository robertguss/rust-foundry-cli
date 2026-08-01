//! Parse and validate Project Spec TOML (pure).
//!
//! Implements REQ-030..033 (schema 1, explicit load, archetype `cli` only,
//! secret field-name denylist). CLI overrides (REQ-034) apply after parse.

mod denylist;
mod error;
mod model;
mod parse;
mod validate;

pub use denylist::{SECRET_FIELD_DENYLIST, field_name_is_denied};
pub use error::{SpecError, SpecErrorKind};
pub use model::{
    ALLOWED_KEYS, ARCHETYPES, PROFILES, ProjectSpec, REQUIRED_KEYS, SUPPORTED_SCHEMA, VERIFY_MODES,
    VerifyMode,
};
pub use parse::{STDIN_SPEC, load_spec, parse_spec_str};
pub use validate::{apply_overrides, validate_raw};
