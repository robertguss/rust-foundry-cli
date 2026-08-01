//! MS-021 dogfood gate: offline catalog generate + cargo test smoke.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn sandbox() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "foundry-ms021-{}-{}",
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
fn offline_catalog_generate_smoke() {
    // Catalog is embedded; generate with --verify none does not need network for
    // catalog/render path (REQ-060). This is not a verify-offline claim.
    let dir = sandbox();
    let dest = dir.join("dogfood");
    let spec = dir.join("spec.toml");
    fs::write(
        &spec,
        format!(
            r#"
schema = 1
name = "dogfood-cli"
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
        .env_remove("http_proxy")
        .env_remove("https_proxy")
        .env_remove("HTTP_PROXY")
        .env_remove("HTTPS_PROXY")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dest.join("Cargo.toml").is_file());
    assert!(dest.join("src/main.rs").is_file());
    assert!(dest.join("rust-toolchain.toml").is_file());

    // cargo test smoke on generated project
    let test = Command::new("cargo")
        .args(["test", "--quiet"])
        .current_dir(&dest)
        .output()
        .expect("cargo test");
    assert!(
        test.status.success(),
        "cargo test stderr: {}\nstdout: {}",
        String::from_utf8_lossy(&test.stderr),
        String::from_utf8_lossy(&test.stdout)
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn evidence_doc_exists() {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/evidence/MS-021-dogfood.md");
    let body = fs::read_to_string(p).unwrap();
    assert!(body.contains("MS-021"));
    assert!(body.contains("REQ-060"));
}
