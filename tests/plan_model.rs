//! MS-003.3: Plan model populates every REQ-041 / §11.2 element.

use foundry::VERSION;
use foundry::plan::{
    DependencyDelta, DestinationPolicy, FileMode, NormalizedSpecRecord, Plan, PlannedFile,
    ResolvedComposition, STUB_CATALOG_DIGEST, content_sha256, seal_plan,
};
use foundry::spec::VerifyMode;

fn fixture_plan() -> Plan {
    let cargo_toml =
        b"[package]\nname = \"example-cli\"\nversion = \"0.1.0\"\nedition = \"2021\"\n";
    let agents = b"# AGENTS.md\n";
    seal_plan(Plan {
        foundry_version: VERSION.into(),
        catalog_digest: STUB_CATALOG_DIGEST.into(),
        normalized_spec: NormalizedSpecRecord {
            schema: 1,
            name: "example-cli".into(),
            description: Some("Minimal cli cell".into()),
            archetype: "cli".into(),
            destination: "./example-cli".into(),
            profiles: vec![],
            verify: VerifyMode::Default,
            source: "examples/minimal-cli.toml".into(),
        },
        composition: ResolvedComposition {
            archetype: "cli".into(),
            ordered_profiles: vec![],
            unit_ids: vec!["core".into(), "cli".into()],
        },
        planned_files: vec![
            PlannedFile {
                path: "Cargo.toml".into(),
                mode: FileMode::File,
                content_digest: content_sha256(cargo_toml),
            },
            PlannedFile {
                path: "src/main.rs".into(),
                mode: FileMode::File,
                content_digest: content_sha256(b"fn main() {}\n"),
            },
            PlannedFile {
                path: "AGENTS.md".into(),
                mode: FileMode::File,
                content_digest: content_sha256(agents),
            },
        ],
        dependency_deltas: vec![DependencyDelta {
            name: "clap".into(),
            version_req: "4".into(),
            features: vec!["derive".into()],
            dev: false,
        }],
        ai_native_paths: vec![
            "AGENTS.md".into(),
            ".agents/skills/quality-gates/SKILL.md".into(),
        ],
        verify: VerifyMode::Default,
        destination_policy: DestinationPolicy::Missing,
        plan_sha256: String::new(),
        warnings: vec!["using stub catalog digest until MS-007".into()],
    })
}

#[test]
fn fixture_populates_every_req041_element() {
    let plan = fixture_plan();
    plan.assert_elements_complete()
        .expect("all REQ-041 elements present");

    // Explicit element presence checks (missing any fails this fixture).
    assert!(!plan.foundry_version.is_empty(), "foundry version");
    assert!(!plan.catalog_digest.is_empty(), "catalog digest");
    assert_eq!(plan.normalized_spec.schema, 1, "normalized spec");
    assert_eq!(plan.composition.archetype, "cli", "resolved composition");
    assert!(
        !plan.planned_files.is_empty(),
        "planned files (path/mode/content digest)"
    );
    for f in &plan.planned_files {
        assert!(!f.path.is_empty());
        assert!(!f.mode.as_str().is_empty());
        assert_eq!(f.content_digest.len(), 64);
    }
    assert!(
        !plan.dependency_deltas.is_empty(),
        "dependency / Cargo.toml deltas"
    );
    assert!(!plan.ai_native_paths.is_empty(), "AI-native paths");
    assert_eq!(plan.verify, VerifyMode::Default, "verify mode");
    assert_eq!(
        plan.destination_policy.as_str(),
        "missing",
        "destination policy"
    );
    assert_eq!(plan.plan_sha256.len(), 64, "plan_sha256");
    assert!(!plan.warnings.is_empty(), "warnings element present");
}

#[test]
fn missing_foundry_version_fails_completeness() {
    let mut plan = fixture_plan();
    plan.foundry_version.clear();
    assert_eq!(
        plan.assert_elements_complete(),
        Err("missing foundry_version")
    );
}

#[test]
fn missing_plan_sha256_fails_completeness() {
    let mut plan = fixture_plan();
    plan.plan_sha256.clear();
    assert_eq!(plan.assert_elements_complete(), Err("missing plan_sha256"));
}

#[test]
fn missing_unit_ids_fails_completeness() {
    let mut plan = fixture_plan();
    plan.composition.unit_ids.clear();
    assert_eq!(
        plan.assert_elements_complete(),
        Err("missing composition.unit_ids")
    );
}

#[test]
fn plan_sha256_stable_for_identical_fixture() {
    let a = fixture_plan();
    let b = fixture_plan();
    assert_eq!(a.plan_sha256, b.plan_sha256);
}
