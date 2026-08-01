//! Archetype and profile resolution (pure).
//!
//! MS-003.2: composition identity for Construct (canonical profile order).
//! Full catalog unit load lands in PHASE-02.

mod error;

pub use error::ResolveError;

use crate::plan::ResolvedComposition;
use crate::spec::{ARCHETYPES, EffectiveInputs, PROFILES};

/// Canonical profile order independent of Project Spec input order (REQ-063).
///
/// Only selected profiles participate; unlisted ids are already rejected at parse.
pub const CANONICAL_PROFILE_ORDER: &[&str] = &["tui", "hooks", "secrets", "distribution"];

/// Always-on catalog units for v1 pure-CLI composition (before PHASE-02 emit matrix).
const CORE_UNIT: &str = "core";

/// Pure resolve: validate archetype/profile membership and produce composition.
///
/// - Archetype must be `cli` (v1 closed set).
/// - Each profile must be ∈ closed set (defense in depth; parse already checks).
/// - Ordered profiles follow [`CANONICAL_PROFILE_ORDER`] for the selected set.
/// - `unit_ids` = `core` + archetype + ordered profiles.
///
/// No filesystem I/O.
pub fn resolve_composition(inputs: &EffectiveInputs) -> Result<ResolvedComposition, ResolveError> {
    if !ARCHETYPES.contains(&inputs.archetype.as_str()) {
        let allowed = ARCHETYPES.join(", ");
        return Err(ResolveError::new(
            "resolve.unknown_archetype",
            format!(
                "unknown archetype {:?}; must be one of: {allowed}",
                inputs.archetype
            ),
        ));
    }

    // Defense in depth: profiles re-validated even though parse enforces the set.
    for profile_id in &inputs.profiles {
        if !PROFILES.contains(&profile_id.as_str()) {
            let allowed = PROFILES.join(", ");
            return Err(ResolveError::new(
                "resolve.unknown_profile",
                format!("unknown profile {profile_id:?}; must be one of: {allowed}"),
            ));
        }
    }

    let selected: std::collections::BTreeSet<&str> =
        inputs.profiles.iter().map(String::as_str).collect();

    let ordered_profiles: Vec<String> = CANONICAL_PROFILE_ORDER
        .iter()
        .filter(|id| selected.contains(**id))
        .map(|s| (*s).to_string())
        .collect();

    let mut unit_ids = Vec::with_capacity(2 + ordered_profiles.len());
    unit_ids.push(CORE_UNIT.to_string());
    unit_ids.push(inputs.archetype.clone());
    unit_ids.extend(ordered_profiles.iter().cloned());

    Ok(ResolvedComposition {
        archetype: inputs.archetype.clone(),
        ordered_profiles,
        unit_ids,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};

    fn inputs_with_profiles(profiles_toml: &str) -> EffectiveInputs {
        let body = format!(
            r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = {profiles_toml}
"#
        );
        let spec = parse_spec_str(&body, "<t>").unwrap();
        normalize_effective_inputs(spec, CliOverrides::default()).unwrap()
    }

    #[test]
    fn empty_profiles_ok() {
        let c = resolve_composition(&inputs_with_profiles("[]")).unwrap();
        assert_eq!(c.archetype, "cli");
        assert!(c.ordered_profiles.is_empty());
        assert_eq!(c.unit_ids, vec!["core", "cli"]);
    }

    #[test]
    fn permutation_yields_canonical_order() {
        let a = resolve_composition(&inputs_with_profiles(
            r#"["distribution", "tui", "secrets", "hooks"]"#,
        ))
        .unwrap();
        let b = resolve_composition(&inputs_with_profiles(
            r#"["hooks", "secrets", "distribution", "tui"]"#,
        ))
        .unwrap();
        assert_eq!(a.ordered_profiles, b.ordered_profiles);
        assert_eq!(
            a.ordered_profiles,
            vec!["tui", "hooks", "secrets", "distribution"]
        );
        assert_eq!(a.unit_ids, b.unit_ids);
        assert_eq!(
            a.unit_ids,
            vec!["core", "cli", "tui", "hooks", "secrets", "distribution"]
        );
    }

    #[test]
    fn subset_preserves_canonical_relative_order() {
        let c = resolve_composition(&inputs_with_profiles(r#"["secrets", "tui"]"#)).unwrap();
        assert_eq!(c.ordered_profiles, vec!["tui", "secrets"]);
        assert_eq!(c.unit_ids, vec!["core", "cli", "tui", "secrets"]);
    }
}
