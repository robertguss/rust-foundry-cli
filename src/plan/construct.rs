//! Pure Construct: effective inputs + catalog view → immutable Plan (REQ-040).

use crate::VERSION;
use crate::catalog::{CatalogFileMode, CatalogView};
use crate::plan::digest::{content_sha256, seal_plan};
use crate::plan::error::ConstructError;
use crate::plan::model::{
    DependencyDelta, DestinationPolicy, FileMode, NormalizedSpecRecord, Plan, PlannedFile,
};
use crate::plan::path_jail::assert_path_jailed;
use crate::resolve::resolve_composition;
use crate::spec::EffectiveInputs;

/// Pure Construct shared by validate / plan / generate (INV-3 / REQ-040).
///
/// No filesystem I/O: only in-memory [`EffectiveInputs`] and [`CatalogView`].
/// Same inputs + catalog digest → bitwise-equal `plan_sha256`.
///
/// Path jail (REQ-053): absolute / `..` escapes in catalog paths hard-fail here.
/// Destination emptiness is re-checked at generate place time; pure Construct
/// records [`DestinationPolicy::Missing`] (admissible if dest absent) plus a
/// warning that generate re-validates.
pub fn construct(inputs: &EffectiveInputs, catalog: &CatalogView) -> Result<Plan, ConstructError> {
    let composition = resolve_composition(inputs)?;

    let mut planned_files = Vec::with_capacity(catalog.files.len());
    let mut ai_native_paths = Vec::new();

    for file in &catalog.files {
        assert_path_jailed(&file.path)?;
        let body = expand_placeholders(&file.body, inputs);
        let digest = content_sha256(body.as_bytes());
        if is_ai_native_path(&file.path) {
            ai_native_paths.push(file.path.clone());
        }
        planned_files.push(PlannedFile {
            path: file.path.clone(),
            mode: map_mode(file.mode),
            content_digest: digest,
        });
    }

    // Deterministic ordering for plan equality (catalog should already be ordered).
    planned_files.sort_by(|a, b| a.path.cmp(&b.path));
    ai_native_paths.sort();

    let dependency_deltas = stub_dependency_deltas();

    // Record canonical profile order (REQ-063) so plan equality is independent
    // of Project Spec profile list order (REQ-040).
    let normalized_spec = NormalizedSpecRecord {
        schema: inputs.schema,
        name: inputs.name.clone(),
        description: inputs.description.clone(),
        archetype: inputs.archetype.clone(),
        destination: inputs.destination.clone(),
        profiles: composition.ordered_profiles.clone(),
        verify: inputs.verify,
        source: inputs.source.clone(),
    };

    let plan = Plan {
        foundry_version: VERSION.to_string(),
        catalog_digest: catalog.digest.clone(),
        normalized_spec,
        composition,
        planned_files,
        dependency_deltas,
        ai_native_paths,
        verify: inputs.verify,
        destination_policy: DestinationPolicy::Missing,
        plan_sha256: String::new(),
        warnings: vec![
            "destination_policy=missing at pure Construct; generate re-validates emptiness (REQ-051)"
                .into(),
        ],
    };

    let sealed = seal_plan(plan);
    sealed
        .assert_elements_complete()
        .map_err(|msg| ConstructError::new("plan.incomplete", msg))?;
    Ok(sealed)
}

fn map_mode(mode: CatalogFileMode) -> FileMode {
    match mode {
        CatalogFileMode::File => FileMode::File,
        CatalogFileMode::Executable => FileMode::Executable,
        CatalogFileMode::Directory => FileMode::Directory,
    }
}

fn expand_placeholders(body: &str, inputs: &EffectiveInputs) -> String {
    body.replace("{{name}}", &inputs.name)
        .replace("{{destination}}", &inputs.destination)
        .replace("{{archetype}}", &inputs.archetype)
}

fn is_ai_native_path(path: &str) -> bool {
    path == "AGENTS.md" || path.starts_with(".agents/") || path.starts_with("agents/")
}

fn stub_dependency_deltas() -> Vec<DependencyDelta> {
    vec![DependencyDelta {
        name: "clap".into(),
        version_req: "4".into(),
        features: vec!["derive".into()],
        dev: false,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{CatalogFile, CatalogFileMode, stub_catalog};
    use crate::spec::{CliOverrides, VerifyMode, normalize_effective_inputs, parse_spec_str};

    fn minimal_inputs() -> EffectiveInputs {
        let body = r#"
schema = 1
name = "example-cli"
archetype = "cli"
destination = "./example-cli"
profiles = []
"#;
        let spec = parse_spec_str(body, "<t>").unwrap();
        normalize_effective_inputs(spec, CliOverrides::default()).unwrap()
    }

    #[test]
    fn deterministic_plan_sha256() {
        let inputs = minimal_inputs();
        let cat = stub_catalog();
        let a = construct(&inputs, &cat).unwrap();
        let b = construct(&inputs, &cat).unwrap();
        assert_eq!(a.plan_sha256, b.plan_sha256);
        assert_eq!(a, b);
    }

    #[test]
    fn absolute_catalog_path_fails() {
        let inputs = minimal_inputs();
        let mut cat = stub_catalog();
        cat.files.push(CatalogFile {
            path: "/etc/passwd".into(),
            mode: CatalogFileMode::File,
            body: "x".into(),
        });
        let err = construct(&inputs, &cat).unwrap_err();
        assert_eq!(err.code, "plan.path_jail");
    }

    #[test]
    fn parent_escape_catalog_path_fails() {
        let inputs = minimal_inputs();
        let mut cat = stub_catalog();
        cat.files.push(CatalogFile {
            path: "../escape".into(),
            mode: CatalogFileMode::File,
            body: "x".into(),
        });
        let err = construct(&inputs, &cat).unwrap_err();
        assert_eq!(err.code, "plan.path_jail");
    }

    #[test]
    fn name_override_changes_digests() {
        let a = construct(&minimal_inputs(), &stub_catalog()).unwrap();
        let body = r#"
schema = 1
name = "example-cli"
archetype = "cli"
destination = "./example-cli"
profiles = []
"#;
        let spec = parse_spec_str(body, "<t>").unwrap();
        let inputs = normalize_effective_inputs(
            spec,
            CliOverrides {
                name: Some("other".into()),
                ..Default::default()
            },
        )
        .unwrap();
        let b = construct(&inputs, &stub_catalog()).unwrap();
        assert_ne!(a.plan_sha256, b.plan_sha256);
        assert_eq!(b.normalized_spec.name, "other");
        assert_eq!(b.verify, VerifyMode::Default);
    }
}
