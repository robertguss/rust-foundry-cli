//! Fixture pass/fail suite for Project Spec parse (REQ-030..033) and validate CLI.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use foundry::spec::{
    CliOverrides, DEFAULT_VERIFY_MODE, SECRET_FIELD_DENYLIST, SpecErrorKind, VerifyMode,
    apply_overrides, normalize_effective_inputs, parse_spec_str,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn minimal_toml() -> String {
    fs::read_to_string(repo_root().join("examples/minimal-cli.toml")).expect("read fixture")
}

fn with_extra_line(extra: &str) -> String {
    let mut body = minimal_toml();
    if !body.ends_with('\n') {
        body.push('\n');
    }
    body.push_str(extra);
    if !body.ends_with('\n') {
        body.push('\n');
    }
    body
}

#[test]
fn parse_minimal_cli_example_fixture() {
    let spec = parse_spec_str(&minimal_toml(), "examples/minimal-cli.toml").unwrap();
    assert_eq!(spec.schema, 1);
    assert_eq!(spec.name, "example-cli");
    assert_eq!(spec.archetype, "cli");
    assert_eq!(spec.destination, "./example-cli");
    assert!(spec.profiles.is_empty());
    assert!(spec.verify.is_none());
    assert!(
        spec.description
            .as_deref()
            .unwrap_or("")
            .contains("Minimal")
    );
}

#[test]
fn optional_verify_and_profiles() {
    let body = r#"
schema = 1
name = "example-cli"
description = "with profiles"
archetype = "cli"
destination = "./example-cli"
profiles = ["tui", "hooks"]
verify = "strict"
"#;
    let spec = parse_spec_str(body, "<test>").unwrap();
    assert_eq!(spec.profiles, vec!["tui".to_string(), "hooks".to_string()]);
    assert_eq!(spec.verify, Some(VerifyMode::Strict));
}

#[test]
fn unsupported_schema_fails() {
    let body = r#"
schema = 2
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
"#;
    let err = parse_spec_str(body, "<t>").unwrap_err();
    assert_eq!(err.code, "spec.unsupported_schema");
    assert_eq!(err.kind, SpecErrorKind::Validation);
}

#[test]
fn unknown_key_fails() {
    let body = with_extra_line("extra_field = true\n");
    let err = parse_spec_str(&body, "<t>").unwrap_err();
    assert_eq!(err.code, "spec.unknown_key");
    assert!(err.message.contains("extra_field"));
}

#[test]
fn missing_required_field_fails() {
    let body = r#"
schema = 1
name = "x"
archetype = "cli"
profiles = []
"#;
    let err = parse_spec_str(body, "<t>").unwrap_err();
    assert_eq!(err.code, "spec.missing_field");
    assert!(err.message.contains("destination"));
}

#[test]
fn non_cli_archetype_fails() {
    let body = r#"
schema = 1
name = "x"
archetype = "lib"
destination = "./x"
profiles = []
"#;
    let err = parse_spec_str(body, "<t>").unwrap_err();
    assert_eq!(err.code, "spec.unknown_archetype");
}

#[test]
fn unknown_profile_fails() {
    let body = r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = ["http"]
"#;
    let err = parse_spec_str(body, "<t>").unwrap_err();
    assert_eq!(err.code, "spec.unknown_profile");
}

#[test]
fn invalid_toml_is_parse_error() {
    let err = parse_spec_str("schema = [", "<t>").unwrap_err();
    assert_eq!(err.code, "spec.toml");
    assert_eq!(err.kind, SpecErrorKind::Parse);
}

#[test]
fn secret_field_denylist_rejects_each_name_various_casing() {
    for name in SECRET_FIELD_DENYLIST {
        for cased in [(*name).to_string(), name.to_ascii_uppercase(), {
            let mut s = name.to_string();
            if let Some(c) = s.get_mut(0..1) {
                c.make_ascii_uppercase();
            }
            s
        }] {
            let body = format!(
                r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
{cased} = "nope"
"#
            );
            let err = parse_spec_str(&body, "<t>").unwrap_err();
            assert_eq!(
                err.code, "spec.secret_field",
                "expected denylist hit for field {cased}"
            );
        }
    }
}

#[test]
fn nested_secret_field_name_rejected() {
    let body = r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
[meta]
token = "leak"
"#;
    // meta is unknown top-level — order of checks: denylist walks first, then unknown keys.
    // nested under unknown key still hits denylist walk.
    let err = parse_spec_str(body, "<t>").unwrap_err();
    // Either secret_field (if denylist first) or unknown_key — denylist runs first.
    assert_eq!(err.code, "spec.secret_field");
}

#[test]
fn overrides_win_over_toml() {
    let spec = parse_spec_str(&minimal_toml(), "<t>").unwrap();
    let effective = apply_overrides(
        spec,
        Some("renamed".into()),
        Some("./elsewhere".into()),
        Some(VerifyMode::None),
    )
    .unwrap();
    assert_eq!(effective.name, "renamed");
    assert_eq!(effective.destination, "./elsewhere");
    assert_eq!(effective.verify, Some(VerifyMode::None));
}

#[test]
fn normalize_flags_win_over_toml() {
    let body = with_extra_line("verify = \"none\"");
    let spec = parse_spec_str(&body, "<t>").unwrap();
    let effective = normalize_effective_inputs(
        spec,
        CliOverrides {
            name: Some("cli-name".into()),
            dest: Some("./cli-dest".into()),
            verify: Some(VerifyMode::Strict),
        },
    )
    .unwrap();
    assert_eq!(effective.name, "cli-name");
    assert_eq!(effective.destination, "./cli-dest");
    assert_eq!(effective.verify, VerifyMode::Strict);
}

#[test]
fn normalize_missing_verify_uses_documented_default() {
    let spec = parse_spec_str(&minimal_toml(), "<t>").unwrap();
    assert!(spec.verify.is_none(), "fixture omits verify");
    let effective = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
    assert_eq!(effective.verify, DEFAULT_VERIFY_MODE);
    assert_eq!(effective.verify, VerifyMode::Default);
}

#[test]
fn normalize_empty_name_rejected() {
    let spec = parse_spec_str(&minimal_toml(), "<t>").unwrap();
    let err = normalize_effective_inputs(
        spec,
        CliOverrides {
            name: Some("".into()),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert_eq!(err.code, "spec.empty_field");
}

#[test]
fn normalize_empty_dest_rejected() {
    let spec = parse_spec_str(&minimal_toml(), "<t>").unwrap();
    let err = normalize_effective_inputs(
        spec,
        CliOverrides {
            dest: Some("  ".into()),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert_eq!(err.code, "spec.empty_field");
}

#[test]
fn cli_validate_prints_effective_default_verify() {
    let spec = repo_root().join("examples/minimal-cli.toml");
    let output = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .args(["validate", "--spec", spec.to_str().unwrap()])
        .output()
        .expect("run validate");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("verify: default"),
        "expected effective default verify in stdout, got:\n{stdout}"
    );
}

#[test]
fn cli_validate_success_exit_zero() {
    let spec = repo_root().join("examples/minimal-cli.toml");
    let output = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .args(["validate", "--spec", spec.to_str().unwrap()])
        .output()
        .expect("run validate");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("foundry validate: ok"));
    assert!(stdout.contains("example-cli"));
}

#[test]
fn cli_validate_failure_exit_nonzero() {
    let dir = tempfile_dir();
    let bad = dir.join("bad.toml");
    fs::write(
        &bad,
        r#"
schema = 99
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
"#,
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .args(["validate", "--spec", bad.to_str().unwrap()])
        .output()
        .expect("run validate");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("spec.unsupported_schema"));
}

#[test]
fn cli_validate_respects_name_override() {
    let spec = repo_root().join("examples/minimal-cli.toml");
    let output = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .args([
            "validate",
            "--spec",
            spec.to_str().unwrap(),
            "--name",
            "overridden",
        ])
        .output()
        .expect("run validate");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("name: overridden"));
}

/// Minimal temp dir without extra deps.
fn tempfile_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "foundry-spec-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}
