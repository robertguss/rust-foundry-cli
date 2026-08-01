//! MS-003.2: resolve stub — archetype cli + profile membership + canonical order.

use foundry::resolve::{CANONICAL_PROFILE_ORDER, resolve_composition};
use foundry::spec::{
    CliOverrides, EffectiveInputs, PROFILES, VerifyMode, normalize_effective_inputs, parse_spec_str,
};

fn effective(profiles: &[&str]) -> EffectiveInputs {
    let list = profiles
        .iter()
        .map(|p| format!("\"{p}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let body = format!(
        r#"
schema = 1
name = "demo"
archetype = "cli"
destination = "./demo"
profiles = [{list}]
"#
    );
    let spec = parse_spec_str(&body, "<t>").unwrap();
    normalize_effective_inputs(spec, CliOverrides::default()).unwrap()
}

#[test]
fn empty_profiles_ok() {
    let c = resolve_composition(&effective(&[])).unwrap();
    assert!(c.ordered_profiles.is_empty());
    assert_eq!(c.unit_ids, ["core", "cli"]);
}

#[test]
fn unknown_profile_hard_fails_at_parse_before_resolve() {
    // Parse rejects unknown profiles; resolve never sees them.
    let body = r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = ["not-a-profile"]
"#;
    let err = parse_spec_str(body, "<t>").unwrap_err();
    assert_eq!(err.code, "spec.unknown_profile");
}

#[test]
fn resolve_rejects_unknown_profile_defense_in_depth() {
    // Bypass parse by constructing EffectiveInputs directly.
    let inputs = EffectiveInputs {
        schema: 1,
        name: "x".into(),
        description: None,
        archetype: "cli".into(),
        destination: "./x".into(),
        profiles: vec!["nope".into()],
        verify: VerifyMode::Default,
        source: "<t>".into(),
    };
    let err = resolve_composition(&inputs).unwrap_err();
    assert_eq!(err.code, "resolve.unknown_profile");
}

#[test]
fn non_cli_archetype_hard_fails() {
    let inputs = EffectiveInputs {
        schema: 1,
        name: "x".into(),
        description: None,
        archetype: "lib".into(),
        destination: "./x".into(),
        profiles: vec![],
        verify: VerifyMode::Default,
        source: "<t>".into(),
    };
    let err = resolve_composition(&inputs).unwrap_err();
    assert_eq!(err.code, "resolve.unknown_archetype");
}

#[test]
fn same_set_any_order_same_composition() {
    let orders = [
        ["tui", "hooks", "secrets", "distribution"],
        ["distribution", "secrets", "hooks", "tui"],
        ["hooks", "tui", "distribution", "secrets"],
    ];
    let mut compositions = Vec::new();
    for order in orders {
        compositions.push(resolve_composition(&effective(&order)).unwrap());
    }
    for c in &compositions[1..] {
        assert_eq!(c.ordered_profiles, compositions[0].ordered_profiles);
        assert_eq!(c.unit_ids, compositions[0].unit_ids);
    }
    assert_eq!(
        compositions[0].ordered_profiles,
        CANONICAL_PROFILE_ORDER
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<_>>()
    );
}

#[test]
fn canonical_order_matches_closed_profile_set() {
    // Every closed profile id appears exactly once in canonical order.
    assert_eq!(CANONICAL_PROFILE_ORDER.len(), PROFILES.len());
    for id in PROFILES {
        assert!(
            CANONICAL_PROFILE_ORDER.contains(id),
            "{id} missing from canonical order"
        );
    }
}
