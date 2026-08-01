//! Smoke tests for the scaffold CLI.

use std::process::Command;

#[test]
fn version_subcommand_prints_package_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .arg("version")
        .output()
        .expect("run foundry version");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("foundry"),
        "stdout missing 'foundry': {stdout:?}"
    );
    assert!(
        stdout.contains(env!("CARGO_PKG_VERSION")),
        "stdout missing version: {stdout:?}"
    );
    assert!(
        stdout.contains("catalog_digest:"),
        "stdout missing catalog_digest: {stdout:?}"
    );
}
