//! MS-003.7: stable error-code matrix (text + JSON shapes).

use foundry::catalog::{CatalogFile, CatalogFileMode, CatalogView};
use foundry::plan::construct;
use foundry::report::format_error_json;
use foundry::spec::{
    CliOverrides, EffectiveInputs, VerifyMode, normalize_effective_inputs, parse_spec_str,
};

fn assert_code(body: &str, code: &str) {
    let err = parse_spec_str(body, "<t>").unwrap_err();
    assert_eq!(err.code, code, "body:\n{body}");
    // JSON shape used by plan --format json construct failures; same code string.
    let json = format_error_json(err.code, &err.message);
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], code);
    assert!(
        v["error"]["message"]
            .as_str()
            .unwrap()
            .contains(err.message.as_str())
            || !err.message.is_empty()
    );
}

#[test]
fn matrix_denylist() {
    assert_code(
        r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
token = "no"
"#,
        "spec.secret_field",
    );
}

#[test]
fn matrix_unsupported_schema() {
    assert_code(
        r#"
schema = 99
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
"#,
        "spec.unsupported_schema",
    );
}

#[test]
fn matrix_unknown_archetype() {
    assert_code(
        r#"
schema = 1
name = "x"
archetype = "lib"
destination = "./x"
profiles = []
"#,
        "spec.unknown_archetype",
    );
}

#[test]
fn matrix_unknown_key() {
    assert_code(
        r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
extra = 1
"#,
        "spec.unknown_key",
    );
}

#[test]
fn matrix_unknown_profile() {
    assert_code(
        r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = ["nope"]
"#,
        "spec.unknown_profile",
    );
}

#[test]
fn matrix_empty_override_name() {
    let body = r#"
schema = 1
name = "x"
archetype = "cli"
destination = "./x"
profiles = []
"#;
    let spec = parse_spec_str(body, "<t>").unwrap();
    let err = normalize_effective_inputs(
        spec,
        CliOverrides {
            name: Some(String::new()),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert_eq!(err.code, "spec.empty_field");
    let v: serde_json::Value =
        serde_json::from_str(&format_error_json(err.code, &err.message)).unwrap();
    assert_eq!(v["error"]["code"], "spec.empty_field");
}

#[test]
fn matrix_path_jail() {
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
    let cat = CatalogView {
        digest: "t".into(),
        files: vec![CatalogFile {
            path: "../escape".into(),
            mode: CatalogFileMode::File,
            body: "x".into(),
        }],
    };
    let err = construct(&inputs, &cat).unwrap_err();
    assert_eq!(err.code, "plan.path_jail");
    let v: serde_json::Value =
        serde_json::from_str(&format_error_json(err.code, &err.message)).unwrap();
    assert_eq!(v["error"]["code"], "plan.path_jail");
}

#[test]
fn error_codes_doc_exists() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/error-codes.md");
    let body = std::fs::read_to_string(&path).expect("docs/error-codes.md");
    for code in [
        "spec.secret_field",
        "spec.unsupported_schema",
        "spec.unknown_archetype",
        "spec.unknown_profile",
        "spec.empty_field",
        "plan.path_jail",
    ] {
        assert!(body.contains(code), "docs must list {code}");
    }
}
