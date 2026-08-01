//! MS-006.2: path-jail + denylist exhaustive/property fixtures (REQ-053/033).

use foundry::catalog::stub_catalog;
use foundry::catalog::{CatalogFile, CatalogFileMode, CatalogView};
use foundry::fsx;
use foundry::plan::{assert_path_jailed, construct};
use foundry::render::render;
use foundry::spec::{CliOverrides, normalize_effective_inputs};
use foundry::spec::{EffectiveInputs, SECRET_FIELD_DENYLIST, VerifyMode, parse_spec_str};
use std::fs;

#[test]
fn path_jail_exhaustive_escapes() {
    let escapes = [
        "/etc/passwd",
        "/tmp/x",
        "\\absolute",
        "../outside",
        "../../etc",
        "foo/../../../etc/passwd",
        "a/../../b",
        "C:/windows",
        "c:\\windows",
    ];
    for p in escapes {
        let err = assert_path_jailed(p).unwrap_err();
        assert_eq!(err.code, "plan.path_jail", "path {p}");
    }
}

#[test]
fn path_jail_safe_paths_ok() {
    for p in [
        "src/main.rs",
        "a/b/c",
        "a/./b",
        "foo/bar/../baz",
        ".gitignore",
    ] {
        assert_path_jailed(p).unwrap_or_else(|e| panic!("{p}: {e}"));
    }
}

#[test]
fn construct_never_accepts_escape_paths() {
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
    for escape in ["../x", "/tmp/x", "foo/../../y"] {
        let cat = CatalogView {
            digest: "t".into(),
            files: vec![CatalogFile {
                path: escape.into(),
                mode: CatalogFileMode::File,
                body: "x".into(),
            }],
        };
        let err = construct(&inputs, &cat).unwrap_err();
        assert_eq!(err.code, "plan.path_jail");
    }
}

#[test]
fn stage_materialize_rejects_absolute_map_key() {
    // Defense: even if a RenderMap were hand-built with absolute path, stage fails closed.
    use foundry::plan::FileMode;
    use foundry::render::{RenderMap, RenderedFile};

    let dir = std::env::temp_dir().join(format!(
        "foundry-jail-stage-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("d");
    let mut map = RenderMap::new();
    map.insert(
        "../escape".into(),
        RenderedFile {
            mode: FileMode::File,
            bytes: b"no".to_vec(),
        },
    );
    let err = fsx::stage_render_map(&dest, &map).unwrap_err();
    assert_eq!(err.code, "plan.path_jail");
    // Outside path not created
    assert!(!dir.join("escape").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn denylist_case_variants_exhaustive() {
    for name in SECRET_FIELD_DENYLIST {
        for variant in case_variants(name) {
            let body = format!(
                r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
{variant} = "secret-value"
"#
            );
            let err = parse_spec_str(&body, "<t>").unwrap_err();
            assert_eq!(err.code, "spec.secret_field", "variant {variant} of {name}");
        }
    }
}

#[test]
fn denylist_nested_and_array_table() {
    // Denylist walk runs before unknown-key checks (validate_raw). Nested denied
    // names under unknown tables must still hard-fail as spec.secret_field.
    let bodies = [
        r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
[nested]
api_key = "x"
"#,
        r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
[[items]]
Token = "x"
"#,
        r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
[cfg]
PASSWORD = "x"
"#,
    ];
    for body in bodies {
        let err = parse_spec_str(body, "<t>").unwrap_err();
        assert_eq!(
            err.code, "spec.secret_field",
            "expected denylist hit, got {} for:\n{body}",
            err.code
        );
    }
}

/// ASCII case variants: original, upper, lower, mixed (first upper).
fn case_variants(name: &str) -> Vec<String> {
    let mut v = vec![
        name.to_string(),
        name.to_ascii_uppercase(),
        name.to_ascii_lowercase(),
    ];
    if let Some(first) = name.chars().next() {
        let mut mixed = first.to_ascii_uppercase().to_string();
        mixed.push_str(&name[first.len_utf8()..].to_ascii_lowercase());
        v.push(mixed);
    }
    v.sort();
    v.dedup();
    v
}

#[test]
fn render_roundtrip_matches_plan_digests() {
    let body = r#"
schema = 1
name = "round"
archetype = "cli"
destination = "./round"
profiles = []
"#;
    let spec = parse_spec_str(body, "<t>").unwrap();
    let inputs = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
    let cat = stub_catalog();
    let plan = construct(&inputs, &cat).unwrap();
    let map = render(&plan, &cat).unwrap();
    assert_eq!(map.len(), plan.planned_files.len());
}
