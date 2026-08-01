//! Architecture: `plan` must not import write-path modules (INV-3 purity).

use std::fs;
use std::path::PathBuf;

fn plan_sources() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/plan");
    let mut files = Vec::new();
    collect_rs(&root, &mut files);
    files
}

fn collect_rs(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).expect("read plan package dir");
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

/// Forbidden import patterns for the pure `plan` module.
const FORBIDDEN: &[&str] = &[
    "crate::fsx",
    "crate::generate",
    "crate::cli",
    "foundry::fsx",
    "foundry::generate",
    "foundry::cli",
];

#[test]
fn plan_package_does_not_import_write_path() {
    let files = plan_sources();
    assert!(!files.is_empty(), "expected plan package sources");

    for path in files {
        let src = fs::read_to_string(&path).expect("read source");
        for needle in FORBIDDEN {
            assert!(
                !src.contains(needle),
                "{} must not import write-path module via `{needle}`",
                path.display()
            );
        }
    }
}
