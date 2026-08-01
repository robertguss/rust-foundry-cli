//! MS-003.6: wire `foundry plan` CLI (non-interactive, exit codes).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn foundry() -> Command {
    Command::new(env!("CARGO_BIN_EXE_foundry"))
}

fn tempfile_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "foundry-plan-cli-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn plan_help_documents_public_overrides() {
    let output = foundry().args(["plan", "--help"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--spec"));
    assert!(stdout.contains("--name"));
    assert!(stdout.contains("--dest"));
    assert!(stdout.contains("--verify"));
    assert!(stdout.contains("--format"));
    assert!(stdout.contains("--out"));
}

#[test]
fn plan_success_exit_zero_text() {
    let spec = repo_root().join("examples/minimal-cli.toml");
    let output = foundry()
        .args(["plan", "--spec", spec.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("foundry plan"));
    assert!(stdout.contains("plan_sha256:"));
    assert!(stdout.contains("example-cli"));
}

#[test]
fn plan_json_format() {
    let spec = repo_root().join("examples/minimal-cli.toml");
    let output = foundry()
        .args(["plan", "--spec", spec.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["plan_sha256"].as_str().unwrap().len(), 64);
}

#[test]
fn plan_out_writes_file_not_dest() {
    let dir = tempfile_dir();
    let dest = dir.join("example-cli");
    fs::create_dir_all(&dest).unwrap();
    let keep = dest.join("KEEP");
    fs::write(&keep, b"stay").unwrap();
    let out = dir.join("plan.json");
    let spec = repo_root().join("examples/minimal-cli.toml");

    let output = foundry()
        .args([
            "plan",
            "--spec",
            spec.to_str().unwrap(),
            "--format",
            "json",
            "--out",
            out.to_str().unwrap(),
            "--dest",
            dest.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(out.is_file());
    let body = fs::read_to_string(&out).unwrap();
    assert!(body.contains("plan_sha256"));
    assert_eq!(fs::read_to_string(&keep).unwrap(), "stay");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn plan_failure_exit_nonzero() {
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
    let output = foundry()
        .args(["plan", "--spec", bad.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("spec.unsupported_schema"));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn plan_failure_json_format_emits_stable_json_error() {
    // Spec-stage failures (before Construct) must still respect --format json
    // (REQ-042 stable JSON error shape for agents), not fall back to text.
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
    let output = foundry()
        .args(["plan", "--spec", bad.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let v: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("expected JSON error on stderr, got {stderr:?}: {e}"));
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "spec.unsupported_schema");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn plan_empty_name_override_fails() {
    let spec = repo_root().join("examples/minimal-cli.toml");
    let output = foundry()
        .args(["plan", "--spec", spec.to_str().unwrap(), "--name", ""])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("spec.empty_field"));
}
