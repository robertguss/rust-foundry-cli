//! MS-007: catalog list/show + sample-spec offline.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn foundry() -> Command {
    Command::new(env!("CARGO_BIN_EXE_foundry"))
}

#[test]
fn catalog_list_offline() {
    let out = foundry().args(["catalog", "list"]).output().unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("catalog_digest:"));
    for id in ["core", "cli", "tui", "hooks", "secrets", "distribution"] {
        assert!(stdout.contains(id), "missing {id}");
    }
}

#[test]
fn catalog_show_cli() {
    let out = foundry().args(["catalog", "show", "cli"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("id: cli"));
    assert!(stdout.contains("Cargo.toml"));
}

#[test]
fn sample_spec_validates() {
    let dir = std::env::temp_dir().join(format!(
        "foundry-sample-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let sample = foundry()
        .args(["sample-spec", "--name", "demo"])
        .output()
        .unwrap();
    assert!(sample.status.success());
    let path = dir.join("sample.toml");
    fs::write(&path, &sample.stdout).unwrap();
    let val = foundry()
        .args(["validate", "--spec", path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        val.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&val.stderr)
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn version_includes_catalog_digest_hex() {
    let out = foundry().arg("version").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout
        .lines()
        .find(|l| l.starts_with("catalog_digest:"))
        .unwrap();
    let digest = line.split_whitespace().nth(1).unwrap();
    assert_eq!(digest.len(), 64);
    assert!(digest.chars().all(|c| c.is_ascii_hexdigit()));
}

#[allow(dead_code)]
fn _p() -> PathBuf {
    PathBuf::new()
}
