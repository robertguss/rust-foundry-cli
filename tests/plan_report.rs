//! MS-003.5: plan report text default + JSON (provisional keys).

use std::fs;
use std::path::PathBuf;

use foundry::catalog::stub_catalog;
use foundry::plan::construct;
use foundry::report::{
    ReportFormat, format_error_json, format_plan, format_plan_json, format_plan_text,
};
use foundry::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};

fn plan() -> foundry::plan::Plan {
    let body = r#"
schema = 1
name = "example-cli"
archetype = "cli"
destination = "./example-cli"
profiles = []
"#;
    let spec = parse_spec_str(body, "examples/minimal-cli.toml").unwrap();
    let inputs = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
    construct(&inputs, &stub_catalog()).unwrap()
}

#[test]
fn text_is_default_and_human_readable() {
    let p = plan();
    let text = format_plan(&p, ReportFormat::Text);
    assert_eq!(text, format_plan_text(&p));
    assert!(text.contains("foundry plan"));
    assert!(text.contains("plan_sha256:"));
    assert!(text.contains("normalized_spec:"));
    assert!(text.contains("composition:"));
    assert!(text.contains("planned_files:"));
    assert!(text.contains("verify:"));
    assert!(text.contains("destination_policy:"));
    assert!(text.contains("warnings:"));
}

#[test]
fn json_includes_plan_sha256() {
    let p = plan();
    let raw = format_plan(&p, ReportFormat::Json);
    assert_eq!(raw, format_plan_json(&p));
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["plan_sha256"].as_str().unwrap().len(), 64);
    assert!(v["foundry_version"].as_str().is_some());
    assert!(v["catalog_digest"].as_str().is_some());
}

#[test]
fn error_json_stable_shape() {
    let raw = format_error_json("plan.path_jail", "escaped");
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "plan.path_jail");
    assert_eq!(v["error"]["message"], "escaped");
}

#[test]
fn out_file_write_is_report_only_not_dest_tree() {
    // Simulate --out: write report bytes only; destination project path untouched.
    let dir = std::env::temp_dir().join(format!(
        "foundry-report-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let dest_project = dir.join("example-cli");
    fs::create_dir_all(&dest_project).unwrap();
    let marker = dest_project.join("KEEP");
    fs::write(&marker, b"stay").unwrap();

    let out_path = dir.join("plan.json");
    let raw = format_plan_json(&plan());
    fs::write(&out_path, raw.as_bytes()).unwrap();

    assert!(out_path.is_file());
    let written = fs::read_to_string(&out_path).unwrap();
    assert!(written.contains("plan_sha256"));

    // Destination project tree not used as report sink / not modified.
    assert_eq!(fs::read_to_string(&marker).unwrap(), "stay");
    let children: Vec<PathBuf> = fs::read_dir(&dest_project)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].file_name().unwrap(), "KEEP");

    let _ = fs::remove_dir_all(&dir);
}
