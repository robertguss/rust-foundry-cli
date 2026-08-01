//! MS-019.1 / REQ-150: named pure-CLI acceptance scenarios (real shipped CLI path).
//!
//! Job / path names recorded in docs/evidence/REQ-150-151-acceptance.md.
//! Each test name is the automation id: `req150_<scenario>`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn foundry() -> Command {
    Command::new(env!("CARGO_BIN_EXE_foundry"))
}

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sandbox(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "foundry-req150-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_spec(path: &Path, name: &str, dest: &Path, profiles: &str) {
    fs::write(
        path,
        format!(
            r#"
schema = 1
name = "{name}"
description = "REQ-150 acceptance fixture"
archetype = "cli"
destination = "{}"
profiles = {profiles}
"#,
            dest.display()
        ),
    )
    .unwrap();
}

/// req150_validate_and_plan_sample — validate + plan on sample pure-CLI spec.
#[test]
fn req150_validate_and_plan_sample() {
    let spec = repo().join("examples/minimal-cli.toml");
    let v = foundry()
        .args(["validate", "--spec", spec.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        v.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&v.stderr)
    );
    let p = foundry()
        .args(["plan", "--spec", spec.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert!(
        p.status.success(),
        "plan failed: {}",
        String::from_utf8_lossy(&p.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&p.stdout).unwrap();
    assert_eq!(json["ok"], true);
    assert_eq!(json["plan_sha256"].as_str().unwrap().len(), 64);
    assert!(!json["planned_files"].as_array().unwrap().is_empty());
}

/// req150_generate_missing_dest — generate into missing destination succeeds.
#[test]
fn req150_generate_missing_dest() {
    let dir = sandbox("missing");
    let dest = dir.join("newproj");
    let spec = dir.join("spec.toml");
    write_spec(&spec, "newproj", &dest, "[]");
    assert!(!dest.exists());
    let out = foundry()
        .args([
            "generate",
            "--spec",
            spec.to_str().unwrap(),
            "--verify",
            "none",
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dest.join("Cargo.toml").is_file());
    assert!(dest.join("src/main.rs").is_file());
    let _ = fs::remove_dir_all(&dir);
}

/// req150_generate_empty_dest — empty dir is admissible.
#[test]
fn req150_generate_empty_dest() {
    let dir = sandbox("empty");
    let dest = dir.join("emptyproj");
    fs::create_dir_all(&dest).unwrap();
    assert!(fs::read_dir(&dest).unwrap().next().is_none());
    let spec = dir.join("spec.toml");
    write_spec(&spec, "emptyproj", &dest, "[]");
    let out = foundry()
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
    assert!(dest.join("Cargo.toml").is_file());
    let _ = fs::remove_dir_all(&dir);
}

/// req150_refuse_nonempty — non-empty dest (incl .git) refuses; dest unchanged.
#[test]
fn req150_refuse_nonempty() {
    let dir = sandbox("nonempty");
    let dest = dir.join("taken");
    fs::create_dir_all(&dest).unwrap();
    fs::create_dir_all(dest.join(".git")).unwrap();
    fs::write(dest.join(".git/HEAD"), b"ref: refs/heads/main").unwrap();
    let spec = dir.join("spec.toml");
    write_spec(&spec, "taken", &dest, "[]");
    let out = foundry()
        .args([
            "generate",
            "--spec",
            spec.to_str().unwrap(),
            "--verify",
            "none",
        ])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("fsx.refuse_non_empty"), "stderr: {stderr}");
    assert!(dest.join(".git/HEAD").is_file());
    let _ = fs::remove_dir_all(&dir);
}

/// req150_refuse_file_at_path — file at dest path refuses.
#[test]
fn req150_refuse_file_at_path() {
    let dir = sandbox("file-dest");
    let dest = dir.join("as-file");
    fs::write(&dest, b"not-a-dir").unwrap();
    let spec = dir.join("spec.toml");
    write_spec(&spec, "as-file", &dest, "[]");
    let out = foundry()
        .args([
            "generate",
            "--spec",
            spec.to_str().unwrap(),
            "--verify",
            "none",
        ])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("fsx.refuse_non_empty"));
    assert_eq!(fs::read_to_string(&dest).unwrap(), "not-a-dir");
    let _ = fs::remove_dir_all(&dir);
}

/// req150_plan_digests_match_tree — plan content digests match placed file bytes.
#[test]
fn req150_plan_digests_match_tree() {
    let dir = sandbox("digests");
    let dest = dir.join("d");
    let spec = dir.join("spec.toml");
    write_spec(&spec, "d", &dest, "[]");
    let plan_out = foundry()
        .args([
            "plan",
            "--spec",
            spec.to_str().unwrap(),
            "--format",
            "json",
            "--dest",
            dest.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(plan_out.status.success());
    let plan: serde_json::Value = serde_json::from_slice(&plan_out.stdout).unwrap();
    let generate_out = foundry()
        .args([
            "generate",
            "--spec",
            spec.to_str().unwrap(),
            "--dest",
            dest.to_str().unwrap(),
            "--verify",
            "none",
        ])
        .output()
        .unwrap();
    assert!(generate_out.status.success());
    for f in plan["planned_files"].as_array().unwrap() {
        let rel = f["path"].as_str().unwrap();
        let expected = f["content_digest"].as_str().unwrap();
        let bytes = fs::read(dest.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"));
        let actual = foundry::plan::content_sha256(&bytes);
        assert_eq!(actual, expected, "digest mismatch for {rel}");
    }
    let _ = fs::remove_dir_all(&dir);
}

/// req150_path_jail — absolute / .. planned paths hard-fail at construct.
#[test]
fn req150_path_jail() {
    use foundry::catalog::{CatalogFile, CatalogFileMode, CatalogView};
    use foundry::plan::construct;
    use foundry::spec::{EffectiveInputs, VerifyMode};

    let inputs = EffectiveInputs {
        schema: 1,
        name: "x".into(),
        description: None,
        archetype: "cli".into(),
        destination: "./x".into(),
        profiles: vec![],
        verify: VerifyMode::Default,
        source: "<t>".into(),
    };
    for bad in ["/etc/passwd", "../escape", "foo/../../etc"] {
        let cat = CatalogView {
            digest: "t".into(),
            files: vec![CatalogFile {
                path: bad.into(),
                mode: CatalogFileMode::File,
                body: "x".into(),
            }],
        };
        let err = construct(&inputs, &cat).unwrap_err();
        assert_eq!(err.code, "plan.path_jail", "path {bad}");
    }
}

/// req150_no_tui_leakage — pure CLI has no ratatui / add-tui-screen.
#[test]
fn req150_no_tui_leakage() {
    let dir = sandbox("no-tui");
    let dest = dir.join("pure");
    let spec = dir.join("spec.toml");
    write_spec(&spec, "pure", &dest, "[]");
    let out = foundry()
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

/// req150_no_claude_mcp — generated tree has no Claude Core / default MCP.
#[test]
fn req150_no_claude_mcp() {
    let dir = sandbox("no-claude");
    let dest = dir.join("ai");
    let spec = dir.join("spec.toml");
    write_spec(&spec, "ai", &dest, "[]");
    let out = foundry()
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
    assert!(!dest.join("CLAUDE.md").exists());
    assert!(!dest.join(".claude").exists());
    assert!(!dest.join(".mcp.json").exists());
    assert!(dest.join("AGENTS.md").is_file());
    let _ = fs::remove_dir_all(&dir);
}

/// Job name registry is committed and lists every req150_* scenario.
#[test]
fn req150_job_names_recorded() {
    let doc = fs::read_to_string(repo().join("docs/evidence/REQ-150-151-acceptance.md")).unwrap();
    for name in [
        "req150_validate_and_plan_sample",
        "req150_generate_missing_dest",
        "req150_generate_empty_dest",
        "req150_refuse_nonempty",
        "req150_refuse_file_at_path",
        "req150_plan_digests_match_tree",
        "req150_path_jail",
        "req150_no_tui_leakage",
        "req150_no_claude_mcp",
    ] {
        assert!(
            doc.contains(name),
            "docs/evidence/REQ-150-151-acceptance.md must record job/path {name}"
        );
    }
    assert!(doc.contains("tests/req150_acceptance.rs"));
}
