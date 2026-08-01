//! MS-008: Core pure-CLI emit from embedded catalog (not cargo-generate).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use foundry::catalog::{default_cli_catalog_view, load_embedded_catalog};
use foundry::plan::construct;
use foundry::render::render;
use foundry::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};

#[test]
fn core_templates_present_in_embed() {
    let cat = load_embedded_catalog().unwrap();
    let core = cat.units.get("core").unwrap();
    let paths: Vec<_> = core.files.iter().map(|(p, _)| p.as_str()).collect();
    assert!(paths.contains(&"AGENTS.md"));
    assert!(paths.contains(&".gitignore"));
    assert!(paths.contains(&"rust-toolchain.toml"));
    let cli = cat.units.get("cli").unwrap();
    let paths: Vec<_> = cli.files.iter().map(|(p, _)| p.as_str()).collect();
    assert!(paths.contains(&"Cargo.toml"));
    assert!(paths.contains(&"src/main.rs"));
    assert!(paths.contains(&"justfile"));
}

#[test]
fn construct_render_from_embedded_offline() {
    let body = r#"
schema = 1
name = "core-emit"
archetype = "cli"
destination = "./core-emit"
profiles = []
"#;
    let spec = parse_spec_str(body, "<t>").unwrap();
    let inputs = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
    let view = default_cli_catalog_view();
    assert_eq!(view.digest.len(), 64);
    let plan = construct(&inputs, &view).unwrap();
    let map = render(&plan, &view).unwrap();
    assert!(map.contains_key("Cargo.toml"));
    assert!(map.contains_key("rust-toolchain.toml"));
    assert!(map.contains_key("justfile"));
    let cargo = String::from_utf8_lossy(&map["Cargo.toml"].bytes);
    assert!(cargo.contains("core-emit"));
    assert!(cargo.contains("clap"));
}

#[test]
fn no_cargo_generate_in_product_sources() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    fn walk(dir: &std::path::Path, hits: &mut Vec<String>) {
        for ent in fs::read_dir(dir).unwrap() {
            let p = ent.unwrap().path();
            if p.is_dir() {
                walk(&p, hits);
            } else if p.extension().and_then(|e| e.to_str()) == Some("rs") {
                let s = fs::read_to_string(&p).unwrap();
                if s.contains("cargo-generate") || s.contains("cargo_generate") {
                    hits.push(p.display().to_string());
                }
            }
        }
    }
    let mut hits = Vec::new();
    walk(&root, &mut hits);
    assert!(hits.is_empty(), "cargo-generate references: {hits:?}");
}

#[test]
fn dry_run_digest_stable_for_core_fixture() {
    let view = default_cli_catalog_view();
    let body = r#"
schema = 1
name = "example-cli"
description = "Minimal cli cell for validate/plan goldens"
archetype = "cli"
destination = "./example-cli"
profiles = []
"#;
    let spec = parse_spec_str(body, "examples/minimal-cli.toml").unwrap();
    let inputs = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
    let a = construct(&inputs, &view).unwrap().plan_sha256;
    let b = construct(&inputs, &view).unwrap().plan_sha256;
    assert_eq!(a, b);
    assert_eq!(a.len(), 64);
}

#[test]
fn generate_cli_emits_core_tree() {
    let dir = std::env::temp_dir().join(format!(
        "foundry-core-emit-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("gen");
    let spec = dir.join("spec.toml");
    fs::write(
        &spec,
        format!(
            r#"
schema = 1
name = "gen"
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
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    for path in [
        "Cargo.toml",
        "src/main.rs",
        "justfile",
        ".gitignore",
        "AGENTS.md",
        "rust-toolchain.toml",
    ] {
        assert!(dest.join(path).is_file(), "missing {path}");
    }
    let _ = fs::remove_dir_all(&dir);
}

/// REQ-101: AGENTS.md content contract — must cover project summary,
/// authority pointers, canonical commands, definition of done, layout map,
/// boundaries, skill index, and at least one teach-as-you-go note.
#[test]
fn agents_md_covers_req101_content_contract() {
    let cat = load_embedded_catalog().unwrap();
    let core = cat.units.get("core").unwrap();
    let (_, body) = core
        .files
        .iter()
        .find(|(p, _)| p == "AGENTS.md")
        .expect("core unit ships AGENTS.md");

    for (label, needle) in [
        ("project summary", "## Summary"),
        ("authority pointers", "## Authority"),
        ("canonical commands", "## Canonical commands"),
        ("definition of done", "## Definition of done"),
        ("layout map", "## Layout"),
        ("boundaries", "## Boundaries"),
        ("skill index", "## Skills"),
        ("teach-as-you-go", "## Why these defaults"),
    ] {
        assert!(
            body.contains(needle),
            "AGENTS.md missing {label} ({needle})"
        );
    }
    // REQ-107: explicit boundary statements, not just a section header.
    for needle in ["secrets", "Windows", "CLAUDE.md", "MCP"] {
        assert!(
            body.contains(needle),
            "AGENTS.md missing boundary note: {needle}"
        );
    }
}
