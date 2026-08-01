//! MS-015.1 / SPK-104: pure-CLI GH template snapshot is catalog SoT; drift fails CI.

use std::fs;
use std::path::PathBuf;

use foundry::catalog::load_embedded_catalog;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn gh_template_ci_yml_matches_catalog_cli_unit() {
    let cat = load_embedded_catalog().expect("embed");
    let cli = cat.units.get("cli").expect("cli unit");
    let catalog_ci = cli
        .files
        .iter()
        .find(|(p, _)| p == ".github/workflows/ci.yml")
        .map(|(_, b)| b.as_str())
        .expect("cli unit must emit .github/workflows/ci.yml");

    let snap_path = repo().join("docs/freeze/gh-template/.github/workflows/ci.yml");
    let snapshot = fs::read_to_string(&snap_path).unwrap_or_else(|e| {
        panic!(
            "missing GH template snapshot at {}: {e}\nRegen: scripts/regen-gh-template-snapshot.sh",
            snap_path.display()
        );
    });
    assert_eq!(
        catalog_ci.trim(),
        snapshot.trim(),
        "GH template snapshot drifted from catalog SoT.\n\
         Regen: scripts/regen-gh-template-snapshot.sh\n\
         Then commit docs/freeze/gh-template/"
    );
}

#[test]
fn catalog_digest_freeze_matches_live_embed() {
    let live = load_embedded_catalog().unwrap().digest;
    let freeze = fs::read_to_string(repo().join("docs/freeze/catalog-digest.txt"))
        .expect("docs/freeze/catalog-digest.txt")
        .trim()
        .to_string();
    assert_eq!(
        live, freeze,
        "catalog digest freeze drifted; regen: scripts/regen-gh-template-snapshot.sh"
    );
}

#[test]
fn regen_script_exists_and_is_documented() {
    let script = repo().join("scripts/regen-gh-template-snapshot.sh");
    assert!(script.is_file(), "missing {}", script.display());
    let body = fs::read_to_string(&script).unwrap();
    assert!(body.contains("catalog/units/cli/templates"));
    assert!(body.contains("docs/freeze/gh-template"));

    let doc = repo().join("docs/evidence/SPK-104-regen.md");
    assert!(doc.is_file(), "missing regen docs");
    let d = fs::read_to_string(doc).unwrap();
    assert!(d.contains("scripts/regen-gh-template-snapshot.sh"));
}

#[test]
fn freeze_ci_is_linux_only() {
    let snap = fs::read_to_string(repo().join("docs/freeze/gh-template/.github/workflows/ci.yml"))
        .unwrap();
    assert!(snap.contains("ubuntu-latest"));
    assert!(!snap.contains("windows-latest"));
}

#[test]
fn regen_script_is_executable_path() {
    // The documented workflow runs the script directly
    // (./scripts/regen-gh-template-snapshot.sh); it must actually be
    // executable, not just present on disk.
    let script = repo().join("scripts/regen-gh-template-snapshot.sh");
    let meta = fs::metadata(&script).unwrap();
    assert!(meta.is_file());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = meta.permissions().mode();
        assert!(
            mode & 0o111 != 0,
            "expected {} to have an executable bit set, mode was {mode:o}",
            script.display()
        );
    }
}
