//! MS-004 / SPK-100: golden plan snapshots (insta).
//!
//! Update path: `cargo insta review` (or `cargo insta accept` after intentional
//! catalog/plan changes). Redaction policy: see `docs/plan-json.md` and
//! `redact_plan_json` below.

use foundry::VERSION;
use foundry::catalog::stub_catalog;
use foundry::plan::construct;
use foundry::report::{format_error_json, format_plan_json, format_plan_text};
use foundry::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};

/// Redact fields that change across builds without semantic plan drift.
///
/// Policy (MS-004.1):
/// - `foundry_version` → `<FOUNDARY_VERSION>` (tied to package version)
/// - `catalog_digest` → `<CATALOG_DIGEST>` until MS-007 pins a real digest
/// - file `content_digest` values stay (they are the contract)
/// - `plan_sha256` is **not** redacted: it is the integrity contract; when
///   redacting version/digest for display goldens we recompute a redacted
///   view that also substitutes plan_sha256 with `<PLAN_SHA256>` so the
///   snapshot stays readable. Equality tests still assert live digests.
fn redact_plan_json(raw: &str) -> String {
    let mut v: serde_json::Value = serde_json::from_str(raw).expect("plan json");
    if let Some(obj) = v.as_object_mut() {
        obj.insert(
            "foundry_version".into(),
            serde_json::Value::String("<FOUNDRY_VERSION>".into()),
        );
        obj.insert(
            "catalog_digest".into(),
            serde_json::Value::String("<CATALOG_DIGEST>".into()),
        );
        obj.insert(
            "plan_sha256".into(),
            serde_json::Value::String("<PLAN_SHA256>".into()),
        );
    }
    serde_json::to_string_pretty(&v).unwrap()
}

fn redact_plan_text(raw: &str) -> String {
    let mut out = raw.to_string();
    out = out.replace(VERSION, "<FOUNDRY_VERSION>");
    // Catalog stub token
    out = out.replace("stub-catalog-v0-unembedded", "<CATALOG_DIGEST>");
    // plan_sha256 line
    let mut lines = Vec::new();
    for line in out.lines() {
        if let Some(rest) = line.strip_prefix("  plan_sha256: ") {
            if rest.len() == 64 && rest.chars().all(|c| c.is_ascii_hexdigit()) {
                lines.push("  plan_sha256: <PLAN_SHA256>".to_string());
                continue;
            }
        }
        // planned file content digests: keep them (contract)
        lines.push(line.to_string());
    }
    lines.join("\n") + "\n"
}

fn minimal_plan() -> foundry::plan::Plan {
    let body = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/minimal-cli.toml"),
    )
    .expect("minimal fixture");
    let spec = parse_spec_str(&body, "examples/minimal-cli.toml").unwrap();
    let inputs = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
    construct(&inputs, &stub_catalog()).unwrap()
}

fn snapshot_settings() -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("snapshots/plan");
    settings.set_prepend_module_to_snapshot(false);
    settings
}

#[test]
fn golden_minimal_cli_plan_json() {
    let plan = minimal_plan();
    let raw = format_plan_json(&plan);
    // Live integrity: sha present
    assert_eq!(plan.plan_sha256.len(), 64);
    let redacted = redact_plan_json(&raw);
    snapshot_settings().bind(|| {
        insta::assert_snapshot!("minimal_cli_plan_json", redacted);
    });
}

#[test]
fn golden_minimal_cli_plan_text() {
    let plan = minimal_plan();
    let raw = format_plan_text(&plan);
    let redacted = redact_plan_text(&raw);
    snapshot_settings().bind(|| {
        insta::assert_snapshot!("minimal_cli_plan_text", redacted);
    });
}

#[test]
fn golden_error_json_shape() {
    let raw = format_error_json(
        "plan.path_jail",
        "path escapes project root via '..': \"../x\"",
    );
    snapshot_settings().bind(|| {
        insta::assert_snapshot!("plan_error_json", raw);
    });
}

#[test]
fn req041_elements_present_in_golden_json() {
    let plan = minimal_plan();
    let v: serde_json::Value = serde_json::from_str(&format_plan_json(&plan)).unwrap();
    for key in [
        "foundry_version",
        "catalog_digest",
        "normalized_spec",
        "composition",
        "planned_files",
        "dependency_deltas",
        "ai_native_paths",
        "verify",
        "destination_policy",
        "plan_sha256",
        "warnings",
    ] {
        assert!(v.get(key).is_some(), "missing REQ-041 element key {key}");
    }
    assert!(!v["planned_files"].as_array().unwrap().is_empty());
}
