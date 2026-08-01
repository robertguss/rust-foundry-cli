//! Parse and validate Project Spec TOML (pure).
//!
//! Implements REQ-030..033 (schema 1, explicit load, archetype `cli` only,
//! secret field-name denylist). CLI overrides (REQ-034) and normalize
//! (effective Construct inputs) apply after parse.

mod denylist;
mod error;
mod model;
mod normalize;
mod parse;
mod validate;

pub use denylist::{SECRET_FIELD_DENYLIST, field_name_is_denied};
pub use error::{SpecError, SpecErrorKind};
pub use model::{
    ALLOWED_KEYS, ARCHETYPES, PROFILES, ProjectSpec, REQUIRED_KEYS, SUPPORTED_SCHEMA, VERIFY_MODES,
    VerifyMode,
};
pub use normalize::{
    CliOverrides, DEFAULT_VERIFY_MODE, EffectiveInputs, normalize_effective_inputs,
};
pub use parse::{STDIN_SPEC, load_spec, parse_spec_str};
pub use validate::{apply_overrides, validate_raw};
