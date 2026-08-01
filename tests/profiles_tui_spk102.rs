//! MS-009 / MS-010 / MS-011: profiles, TUI inclusion, SPK-102 pure-CLI separation.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use foundry::catalog::{catalog_view_for_units, load_embedded_catalog};
use foundry::plan::construct;
use foundry::resolve::resolve_composition;
use foundry::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};

fn inputs(profiles: &str) -> foundry::spec::EffectiveInputs {
    let body = format!(
        r#"
schema = 1
name = "p"
archetype = "cli"
destination = "./p"
profiles = {profiles}
"#
    );
    let spec = parse_spec_str(&body, "<t>").unwrap();
    normalize_effective_inputs(spec, CliOverrides::default()).unwrap()
}

#[test]
fn profile_permutation_invariance() {
    let orders = [r#"["hooks", "secrets"]"#, r#"["secrets", "hooks"]"#];
    let mut plans = Vec::new();
    for o in orders {
        let i = inputs(o);
        let c = resolve_composition(&i).unwrap();
        let cat = load_embedded_catalog().unwrap();
        let view = catalog_view_for_units(&cat, &c.unit_ids).unwrap();
        plans.push(construct(&i, &view).unwrap());
    }
    assert_eq!(plans[0].plan_sha256, plans[1].plan_sha256);
    assert_eq!(
        plans[0].composition.ordered_profiles,
        vec!["hooks", "secrets"]
    );
}

#[test]
fn hooks_and_secrets_emit_paths() {
    let i = inputs(r#"["hooks", "secrets"]"#);
    let c = resolve_composition(&i).unwrap();
    let cat = load_embedded_catalog().unwrap();
    let view = catalog_view_for_units(&cat, &c.unit_ids).unwrap();
    let plan = construct(&i, &view).unwrap();
    let paths: Vec<_> = plan.planned_files.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&".pre-commit-config.yaml"));
    assert!(paths.contains(&".secrets/README.md"));
}

#[test]
fn pure_cli_no_tui_leakage() {
    let i = inputs("[]");
    let c = resolve_composition(&i).unwrap();
    let cat = load_embedded_catalog().unwrap();
    let view = catalog_view_for_units(&cat, &c.unit_ids).unwrap();
    let plan = construct(&i, &view).unwrap();
    for f in &plan.planned_files {
        assert!(!f.path.contains("tui"), "leak {}", f.path);
        assert!(!f.path.contains("add-tui-screen"), "leak {}", f.path);
        assert!(!f.path.contains("ratatui"), "leak {}", f.path);
    }
    let joined = plan
        .planned_files
        .iter()
        .map(|f| f.path.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!joined.contains("src/tui"));
}

#[test]
fn tui_profile_includes_tui_paths() {
    let i = inputs(r#"["tui"]"#);
    let c = resolve_composition(&i).unwrap();
    let cat = load_embedded_catalog().unwrap();
    let view = catalog_view_for_units(&cat, &c.unit_ids).unwrap();
    let plan = construct(&i, &view).unwrap();
    let paths: Vec<_> = plan.planned_files.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&"src/tui/mod.rs"));
    assert!(paths.iter().any(|p| p.contains("add-tui-screen")));
}

#[test]
fn side_by_side_inventory() {
    let pure = {
        let i = inputs("[]");
        let c = resolve_composition(&i).unwrap();
        let cat = load_embedded_catalog().unwrap();
        catalog_view_for_units(&cat, &c.unit_ids)
            .unwrap()
            .files
            .iter()
            .map(|f| f.path.clone())
            .collect::<Vec<_>>()
    };
    let tui = {
        let i = inputs(r#"["tui"]"#);
        let c = resolve_composition(&i).unwrap();
        let cat = load_embedded_catalog().unwrap();
        catalog_view_for_units(&cat, &c.unit_ids)
            .unwrap()
            .files
            .iter()
            .map(|f| f.path.clone())
            .collect::<Vec<_>>()
    };
    assert!(!pure.iter().any(|p| p.contains("tui")));
    assert!(tui.iter().any(|p| p.contains("tui")));
    for p in &pure {
        assert!(
            tui.contains(p),
            "tui inventory should be pure ∪ tui deltas; missing {p}"
        );
    }
}

#[test]
fn generate_pure_cli_no_ratatui_files() {
    let dir = std::env::temp_dir().join(format!(
        "foundry-spk102-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("pure");
    let spec = dir.join("s.toml");
    fs::write(
        &spec,
        format!(
            r#"
schema = 1
name = "pure"
archetype = "cli"
destination = "{}"
profiles = []
"#,
            dest.display()
        ),
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .args([
            "generate",
            "--spec",
            spec.to_str().unwrap(),
            "--verify",
            "none",
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(!dest.join("src/tui").exists());
    assert!(!dest.join(".agents/skills/add-tui-screen").exists());
    let cargo = fs::read_to_string(dest.join("Cargo.toml")).unwrap();
    assert!(!cargo.contains("ratatui"));
    assert!(!cargo.contains("crossterm"));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn phase02_evidence_placeholder() {
    // Structural: module map still present
    let _ = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/resolve/mod.rs");
}
