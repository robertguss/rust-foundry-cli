//! Text/JSON encoding of plans and diagnostics (pure encode).
//!
//! REQ-042: text default; JSON via `--format json`. Field names provisional
//! until MS-004 (SPK-100 / OQ-200).

use serde_json::{Value, json};

use crate::plan::{DestinationPolicy, Plan};

/// Output format for plan reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReportFormat {
    /// Human-readable text (default).
    #[default]
    Text,
    /// Machine JSON (provisional keys until MS-004).
    Json,
}

impl ReportFormat {
    /// Parse format flag values (`text` | `json`).
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "text" => Some(Self::Text),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

/// Encode a successful plan as text (agent-skimmable).
pub fn format_plan_text(plan: &Plan) -> String {
    let mut out = String::new();
    out.push_str("foundry plan\n");
    out.push_str(&format!("  foundry_version: {}\n", plan.foundry_version));
    out.push_str(&format!("  catalog_digest: {}\n", plan.catalog_digest));
    out.push_str(&format!("  plan_sha256: {}\n", plan.plan_sha256));
    out.push_str(&format!("  verify: {}\n", plan.verify.as_str()));
    out.push_str(&format!(
        "  destination_policy: {}\n",
        destination_policy_label(&plan.destination_policy)
    ));

    let ns = &plan.normalized_spec;
    out.push_str("  normalized_spec:\n");
    out.push_str(&format!("    name: {}\n", ns.name));
    out.push_str(&format!("    archetype: {}\n", ns.archetype));
    out.push_str(&format!("    destination: {}\n", ns.destination));
    out.push_str(&format!("    schema: {}\n", ns.schema));
    if ns.profiles.is_empty() {
        out.push_str("    profiles: []\n");
    } else {
        out.push_str(&format!("    profiles: [{}]\n", ns.profiles.join(", ")));
    }

    let c = &plan.composition;
    out.push_str("  composition:\n");
    out.push_str(&format!("    archetype: {}\n", c.archetype));
    out.push_str(&format!(
        "    ordered_profiles: [{}]\n",
        c.ordered_profiles.join(", ")
    ));
    out.push_str(&format!("    unit_ids: [{}]\n", c.unit_ids.join(", ")));

    out.push_str(&format!(
        "  planned_files: ({})\n",
        plan.planned_files.len()
    ));
    for f in &plan.planned_files {
        out.push_str(&format!(
            "    - {} ({}) {}\n",
            f.path,
            f.mode.as_str(),
            f.content_digest
        ));
    }

    out.push_str(&format!(
        "  dependency_deltas: ({})\n",
        plan.dependency_deltas.len()
    ));
    for d in &plan.dependency_deltas {
        let kind = if d.dev { "dev" } else { "normal" };
        out.push_str(&format!(
            "    - {} {} features=[{}] ({})\n",
            d.name,
            d.version_req,
            d.features.join(","),
            kind
        ));
    }

    out.push_str(&format!(
        "  ai_native_paths: [{}]\n",
        plan.ai_native_paths.join(", ")
    ));

    if plan.warnings.is_empty() {
        out.push_str("  warnings: []\n");
    } else {
        out.push_str("  warnings:\n");
        for w in &plan.warnings {
            out.push_str(&format!("    - {w}\n"));
        }
    }
    out
}

/// Encode a successful plan as JSON (provisional field names).
pub fn format_plan_json(plan: &Plan) -> String {
    let value = plan_to_json(plan);
    // Pretty JSON for agents; stable key order via serde_json::Map insertion order
    // is not guaranteed — use compact then pretty via to_string_pretty.
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".into())
}

/// Encode a plan report error as JSON (stable shape for agents / SPK-100).
///
/// Shape: `{ "ok": false, "error": { "code": "...", "message": "..." } }`
pub fn format_error_json(code: &str, message: &str) -> String {
    let value = json!({
        "ok": false,
        "error": {
            "code": code,
            "message": message,
        }
    });
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| {
        r#"{"ok":false,"error":{"code":"report.encode","message":"json encode failed"}}"#.into()
    })
}

/// Encode plan or error according to format.
pub fn format_plan(plan: &Plan, format: ReportFormat) -> String {
    match format {
        ReportFormat::Text => format_plan_text(plan),
        ReportFormat::Json => format_plan_json(plan),
    }
}

fn destination_policy_label(policy: &DestinationPolicy) -> String {
    match policy {
        DestinationPolicy::Missing => "missing".into(),
        DestinationPolicy::EmptyAdmissible => "empty_admissible".into(),
        DestinationPolicy::Refuse { reason } => format!("refuse:{reason}"),
    }
}

fn plan_to_json(plan: &Plan) -> Value {
    let ns = &plan.normalized_spec;
    let c = &plan.composition;
    json!({
        "ok": true,
        "foundry_version": plan.foundry_version,
        "catalog_digest": plan.catalog_digest,
        "plan_sha256": plan.plan_sha256,
        "verify": plan.verify.as_str(),
        "destination_policy": destination_policy_label(&plan.destination_policy),
        "normalized_spec": {
            "schema": ns.schema,
            "name": ns.name,
            "description": ns.description,
            "archetype": ns.archetype,
            "destination": ns.destination,
            "profiles": ns.profiles,
            "verify": ns.verify.as_str(),
            "source": ns.source,
        },
        "composition": {
            "archetype": c.archetype,
            "ordered_profiles": c.ordered_profiles,
            "unit_ids": c.unit_ids,
        },
        "planned_files": plan.planned_files.iter().map(|f| json!({
            "path": f.path,
            "mode": f.mode.as_str(),
            "content_digest": f.content_digest,
        })).collect::<Vec<_>>(),
        "dependency_deltas": plan.dependency_deltas.iter().map(|d| json!({
            "name": d.name,
            "version_req": d.version_req,
            "features": d.features,
            "dev": d.dev,
        })).collect::<Vec<_>>(),
        "ai_native_paths": plan.ai_native_paths,
        "warnings": plan.warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::stub_catalog;
    use crate::plan::construct;
    use crate::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};

    fn sample_plan() -> Plan {
        let body = r#"
schema = 1
name = "example-cli"
archetype = "cli"
destination = "./example-cli"
profiles = []
"#;
        let spec = parse_spec_str(body, "<t>").unwrap();
        let inputs = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
        construct(&inputs, &stub_catalog()).unwrap()
    }

    #[test]
    fn text_contains_core_elements() {
        let text = format_plan_text(&sample_plan());
        assert!(text.contains("foundry plan"));
        assert!(text.contains("plan_sha256:"));
        assert!(text.contains("planned_files:"));
        assert!(text.contains("Cargo.toml"));
        assert!(text.contains("composition:"));
    }

    #[test]
    fn json_includes_plan_sha256_and_ok() {
        let raw = format_plan_json(&sample_plan());
        let v: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(v["ok"], true);
        assert!(v["plan_sha256"].as_str().unwrap().len() == 64);
        assert!(!v["planned_files"].as_array().unwrap().is_empty());
    }

    #[test]
    fn error_json_stable_shape() {
        let raw = format_error_json("spec.empty_field", "bad");
        let v: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(v["ok"], false);
        assert_eq!(v["error"]["code"], "spec.empty_field");
        assert_eq!(v["error"]["message"], "bad");
    }
}
