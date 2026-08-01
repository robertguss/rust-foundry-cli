//! MS-003.4: pure Construct — deterministic plan_sha256 + path jail.

use foundry::catalog::{CatalogFile, CatalogFileMode, CatalogView, stub_catalog};
use foundry::plan::{assert_path_jailed, construct};
use foundry::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};

fn load_effective(toml: &str) -> foundry::spec::EffectiveInputs {
    let spec = parse_spec_str(toml, "<t>").unwrap();
    normalize_effective_inputs(spec, CliOverrides::default()).unwrap()
}

const MINIMAL: &str = r#"
schema = 1
name = "example-cli"
description = "Minimal"
archetype = "cli"
destination = "./example-cli"
profiles = []
"#;

#[test]
fn same_inputs_equal_plan_sha256() {
    let inputs = load_effective(MINIMAL);
    let cat = stub_catalog();
    let a = construct(&inputs, &cat).unwrap();
    let b = construct(&inputs, &cat).unwrap();
    assert_eq!(a.plan_sha256, b.plan_sha256);
    assert_eq!(a.planned_files, b.planned_files);
    a.assert_elements_complete().unwrap();
}

#[test]
fn stub_catalog_plans_expected_paths() {
    let plan = construct(&load_effective(MINIMAL), &stub_catalog()).unwrap();
    let paths: Vec<_> = plan.planned_files.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&"Cargo.toml"));
    assert!(paths.contains(&"src/main.rs"));
    assert!(paths.contains(&"README.md"));
    assert!(paths.contains(&"AGENTS.md"));
    assert!(plan.ai_native_paths.iter().any(|p| p == "AGENTS.md"));
}

#[test]
fn path_jail_absolute() {
    let err = assert_path_jailed("/tmp/x").unwrap_err();
    assert_eq!(err.code, "plan.path_jail");
}

#[test]
fn path_jail_dotdot() {
    let err = assert_path_jailed("../../etc/passwd").unwrap_err();
    assert_eq!(err.code, "plan.path_jail");
}

#[test]
fn construct_rejects_jailed_catalog_entry() {
    let inputs = load_effective(MINIMAL);
    let cat = CatalogView {
        digest: "evil".into(),
        files: vec![CatalogFile {
            path: "../outside".into(),
            mode: CatalogFileMode::File,
            body: "nope".into(),
        }],
    };
    let err = construct(&inputs, &cat).unwrap_err();
    assert_eq!(err.code, "plan.path_jail");
}
