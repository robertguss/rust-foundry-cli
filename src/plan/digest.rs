//! Content digests and plan_sha256 (canonical semantic serialization).
//!
//! Serialization here is **provisional** for integrity hashing only; JSON field
//! names for the `plan` command freeze at MS-004 (SPK-100).

use sha2::{Digest, Sha256};

use crate::plan::model::Plan;

/// SHA-256 hex digest of raw bytes (planned file content).
pub fn content_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

/// Compute `plan_sha256` over the semantic plan content (excludes `plan_sha256`).
///
/// Canonical form is a stable, line-oriented encoding so two equal plans hash
/// equal regardless of construction path (REQ-040).
pub fn compute_plan_sha256(plan: &Plan) -> String {
    let mut hasher = Sha256::new();
    for line in canonical_plan_lines(plan) {
        hasher.update(line.as_bytes());
        hasher.update(b"\n");
    }
    hex::encode(hasher.finalize())
}

/// Build a plan with `plan_sha256` filled from semantic content.
pub fn seal_plan(mut plan: Plan) -> Plan {
    plan.plan_sha256.clear();
    plan.plan_sha256 = compute_plan_sha256(&plan);
    plan
}

fn canonical_plan_lines(plan: &Plan) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("foundry_version={}", plan.foundry_version));
    lines.push(format!("catalog_digest={}", plan.catalog_digest));

    let ns = &plan.normalized_spec;
    lines.push(format!("normalized_spec.schema={}", ns.schema));
    lines.push(format!("normalized_spec.name={}", ns.name));
    lines.push(format!(
        "normalized_spec.description={}",
        ns.description.as_deref().unwrap_or("")
    ));
    lines.push(format!("normalized_spec.archetype={}", ns.archetype));
    lines.push(format!("normalized_spec.destination={}", ns.destination));
    lines.push(format!(
        "normalized_spec.profiles={}",
        ns.profiles.join(",")
    ));
    lines.push(format!("normalized_spec.verify={}", ns.verify.as_str()));
    lines.push(format!("normalized_spec.source={}", ns.source));

    let c = &plan.composition;
    lines.push(format!("composition.archetype={}", c.archetype));
    lines.push(format!(
        "composition.ordered_profiles={}",
        c.ordered_profiles.join(",")
    ));
    lines.push(format!("composition.unit_ids={}", c.unit_ids.join(",")));

    for f in &plan.planned_files {
        lines.push(format!(
            "file\t{}\t{}\t{}",
            f.path,
            f.mode.as_str(),
            f.content_digest
        ));
    }
    for d in &plan.dependency_deltas {
        lines.push(format!(
            "dep\t{}\t{}\t{}\t{}",
            d.name,
            d.version_req,
            d.features.join("+"),
            if d.dev { "dev" } else { "normal" }
        ));
    }
    for p in &plan.ai_native_paths {
        lines.push(format!("ai_path\t{p}"));
    }
    lines.push(format!("verify={}", plan.verify.as_str()));
    match &plan.destination_policy {
        crate::plan::model::DestinationPolicy::Refuse { reason } => {
            lines.push(format!("destination_policy=refuse:{reason}"));
        }
        other => lines.push(format!("destination_policy={}", other.as_str())),
    }
    for w in &plan.warnings {
        lines.push(format!("warning\t{w}"));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::model::{
        DependencyDelta, DestinationPolicy, FileMode, NormalizedSpecRecord, PlannedFile,
        ResolvedComposition, STUB_CATALOG_DIGEST,
    };
    use crate::spec::VerifyMode;

    fn sample_unsealed() -> Plan {
        Plan {
            foundry_version: "0.1.0".into(),
            catalog_digest: STUB_CATALOG_DIGEST.into(),
            normalized_spec: NormalizedSpecRecord {
                schema: 1,
                name: "example-cli".into(),
                description: Some("desc".into()),
                archetype: "cli".into(),
                destination: "./example-cli".into(),
                profiles: vec![],
                verify: VerifyMode::Default,
                source: "<t>".into(),
            },
            composition: ResolvedComposition {
                archetype: "cli".into(),
                ordered_profiles: vec![],
                unit_ids: vec!["core".into(), "cli".into()],
            },
            planned_files: vec![PlannedFile {
                path: "Cargo.toml".into(),
                mode: FileMode::File,
                content_digest: content_sha256(b"[package]\nname=\"example-cli\"\n"),
            }],
            dependency_deltas: vec![DependencyDelta {
                name: "clap".into(),
                version_req: "4".into(),
                features: vec!["derive".into()],
                dev: false,
            }],
            ai_native_paths: vec!["AGENTS.md".into()],
            verify: VerifyMode::Default,
            destination_policy: DestinationPolicy::Missing,
            plan_sha256: String::new(),
            warnings: vec!["stub catalog".into()],
        }
    }

    #[test]
    fn seal_is_deterministic() {
        let a = seal_plan(sample_unsealed());
        let b = seal_plan(sample_unsealed());
        assert_eq!(a.plan_sha256, b.plan_sha256);
        assert_eq!(a.plan_sha256.len(), 64);
    }

    #[test]
    fn content_change_changes_hash() {
        let a = seal_plan(sample_unsealed());
        let mut other = sample_unsealed();
        other.planned_files[0].content_digest = content_sha256(b"different");
        let b = seal_plan(other);
        assert_ne!(a.plan_sha256, b.plan_sha256);
    }
}
