//! Archetype and profile resolution (pure).
//!
//! Canonical profile order (REQ-063) + catalog `requires` edges (MS-009).

mod error;

pub use error::ResolveError;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::catalog::{EmbeddedCatalog, load_embedded_catalog};
use crate::plan::ResolvedComposition;
use crate::spec::{ARCHETYPES, EffectiveInputs, PROFILES};

/// Canonical profile order independent of Project Spec input order (REQ-063).
pub const CANONICAL_PROFILE_ORDER: &[&str] = &["tui", "hooks", "secrets", "distribution"];

const CORE_UNIT: &str = "core";

/// Pure resolve: validate membership + requires, produce ordered composition.
pub fn resolve_composition(inputs: &EffectiveInputs) -> Result<ResolvedComposition, ResolveError> {
    let catalog = load_embedded_catalog().ok();
    resolve_composition_with_catalog(inputs, catalog.as_ref())
}

/// Resolve with an optional catalog for requires edges (tests inject custom graphs).
pub fn resolve_composition_with_catalog(
    inputs: &EffectiveInputs,
    catalog: Option<&EmbeddedCatalog>,
) -> Result<ResolvedComposition, ResolveError> {
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

    for profile_id in &inputs.profiles {
        if !PROFILES.contains(&profile_id.as_str()) {
            let allowed = PROFILES.join(", ");
            return Err(ResolveError::new(
                "resolve.unknown_profile",
                format!("unknown profile {profile_id:?}; must be one of: {allowed}"),
            ));
        }
    }

    let selected: BTreeSet<&str> = inputs.profiles.iter().map(String::as_str).collect();

    let ordered_profiles: Vec<String> = CANONICAL_PROFILE_ORDER
        .iter()
        .filter(|id| selected.contains(**id))
        .map(|s| (*s).to_string())
        .collect();

    // Seed unit set: core + archetype + profiles.
    let mut needed: BTreeSet<String> = BTreeSet::new();
    needed.insert(CORE_UNIT.to_string());
    needed.insert(inputs.archetype.clone());
    for p in &ordered_profiles {
        needed.insert(p.clone());
    }

    // Expand requires from catalog (if available).
    if let Some(cat) = catalog {
        let mut queue: VecDeque<String> = needed.iter().cloned().collect();
        while let Some(id) = queue.pop_front() {
            let Some(unit) = cat.units.get(&id) else {
                return Err(ResolveError::new(
                    "resolve.unknown_unit",
                    format!("catalog missing unit {id:?}"),
                ));
            };
            for req in &unit.requires {
                if needed.insert(req.clone()) {
                    queue.push_back(req.clone());
                }
            }
        }

        // Topological order of units with cycle detection.
        let unit_ids = topo_sort_units(cat, &needed)?;
        return Ok(ResolvedComposition {
            archetype: inputs.archetype.clone(),
            ordered_profiles,
            unit_ids,
        });
    }

    // Fallback without catalog: fixed order core + archetype + profiles.
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

fn topo_sort_units(
    catalog: &EmbeddedCatalog,
    needed: &BTreeSet<String>,
) -> Result<Vec<String>, ResolveError> {
    let mut indegree: BTreeMap<String, usize> = BTreeMap::new();
    let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for id in needed {
        indegree.entry(id.clone()).or_insert(0);
        adj.entry(id.clone()).or_default();
    }
    for id in needed {
        let unit = catalog.units.get(id).ok_or_else(|| {
            ResolveError::new(
                "resolve.unknown_unit",
                format!("catalog missing unit {id:?}"),
            )
        })?;
        for req in &unit.requires {
            if !needed.contains(req) {
                return Err(ResolveError::new(
                    "resolve.missing_requires",
                    format!("unit {id:?} requires {req:?} which is not selected/expanded"),
                ));
            }
            // Edge req → id (req before id)
            adj.entry(req.clone()).or_default().push(id.clone());
            *indegree.entry(id.clone()).or_insert(0) += 1;
        }
    }

    // Prefer stable order: prefer core, then archetype order, then CANONICAL profiles.
    let mut ready: Vec<String> = indegree
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(k, _)| k.clone())
        .collect();
    ready.sort_by_key(|id| unit_rank(id));

    let mut out = Vec::new();
    while let Some(n) = {
        if ready.is_empty() {
            None
        } else {
            Some(ready.remove(0))
        }
    } {
        out.push(n.clone());
        let children = adj.get(&n).cloned().unwrap_or_default();
        for c in children {
            if let Some(d) = indegree.get_mut(&c) {
                *d -= 1;
                if *d == 0 {
                    ready.push(c);
                    ready.sort_by_key(|id| unit_rank(id));
                }
            }
        }
    }

    if out.len() != needed.len() {
        return Err(ResolveError::new(
            "resolve.cycle",
            "catalog requires graph has a cycle among selected units",
        ));
    }
    Ok(out)
}

fn unit_rank(id: &str) -> (u8, String) {
    if id == "core" {
        return (0, id.to_string());
    }
    if ARCHETYPES.contains(&id) {
        return (1, id.to_string());
    }
    if let Some(i) = CANONICAL_PROFILE_ORDER.iter().position(|p| *p == id) {
        return (2 + i as u8, id.to_string());
    }
    (9, id.to_string())
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
        assert!(c.unit_ids.contains(&"core".to_string()));
        assert!(c.unit_ids.contains(&"cli".to_string()));
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
    }

    #[test]
    fn subset_preserves_canonical_relative_order() {
        let c = resolve_composition(&inputs_with_profiles(r#"["secrets", "tui"]"#)).unwrap();
        assert_eq!(c.ordered_profiles, vec!["tui", "secrets"]);
    }

    #[test]
    fn requires_expanded_for_tui() {
        let c = resolve_composition(&inputs_with_profiles(r#"["tui"]"#)).unwrap();
        // tui requires core, cli — already present
        assert!(c.unit_ids.contains(&"tui".to_string()));
        assert!(c.unit_ids.contains(&"core".to_string()));
        assert!(c.unit_ids.contains(&"cli".to_string()));
        // core before cli before tui
        let core_i = c.unit_ids.iter().position(|u| u == "core").unwrap();
        let cli_i = c.unit_ids.iter().position(|u| u == "cli").unwrap();
        let tui_i = c.unit_ids.iter().position(|u| u == "tui").unwrap();
        assert!(core_i < cli_i && cli_i < tui_i);
    }
}
