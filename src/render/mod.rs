//! Templates → bytes (pure given catalog + plan).
//!
//! Produces the path→bytes map that fsx materializes under a stage root.
//! No filesystem I/O.

use std::collections::BTreeMap;

use crate::catalog::{CatalogFileMode, CatalogView};
use crate::plan::assert_path_jailed;
use crate::plan::{FileMode, Plan, content_sha256};

/// One rendered file ready for staging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedFile {
    /// Emit mode.
    pub mode: FileMode,
    /// Exact bytes to write.
    pub bytes: Vec<u8>,
}

/// Deterministic path → rendered file map (sorted keys).
pub type RenderMap = BTreeMap<String, RenderedFile>;

/// Failure while rendering a plan against a catalog view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderError {
    /// Stable error code.
    pub code: &'static str,
    /// Human message.
    pub message: String,
}

impl RenderError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RenderError {}

/// Pure render: expand catalog templates to match [`Plan`] digests.
///
/// Deterministic for the same plan + catalog. Path jail re-checked (REQ-053).
/// No FS writes.
pub fn render(plan: &Plan, catalog: &CatalogView) -> Result<RenderMap, RenderError> {
    if catalog.digest != plan.catalog_digest {
        return Err(RenderError::new(
            "render.catalog_mismatch",
            format!(
                "catalog digest {} != plan catalog_digest {}",
                catalog.digest, plan.catalog_digest
            ),
        ));
    }

    let name = &plan.normalized_spec.name;
    let destination = &plan.normalized_spec.destination;
    let archetype = &plan.normalized_spec.archetype;

    let mut by_path: BTreeMap<String, (FileMode, Vec<u8>)> = BTreeMap::new();
    for file in &catalog.files {
        assert_path_jailed(&file.path).map_err(|e| RenderError::new(e.code, e.message))?;
        let body = file
            .body
            .replace("{{name}}", name)
            .replace("{{destination}}", destination)
            .replace("{{archetype}}", archetype);
        let bytes = body.into_bytes();
        by_path.insert(file.path.clone(), (map_mode(file.mode), bytes));
    }

    // Ensure every planned file is produced and digests match.
    let mut out = RenderMap::new();
    for planned in &plan.planned_files {
        assert_path_jailed(&planned.path).map_err(|e| RenderError::new(e.code, e.message))?;
        let Some((mode, bytes)) = by_path.remove(&planned.path) else {
            return Err(RenderError::new(
                "render.missing_template",
                format!("plan path {:?} has no catalog template", planned.path),
            ));
        };
        let digest = content_sha256(&bytes);
        if digest != planned.content_digest {
            return Err(RenderError::new(
                "render.digest_mismatch",
                format!(
                    "path {:?}: rendered digest {digest} != plan {}",
                    planned.path, planned.content_digest
                ),
            ));
        }
        if mode != planned.mode {
            return Err(RenderError::new(
                "render.mode_mismatch",
                format!(
                    "path {:?}: mode {} != plan {}",
                    planned.path,
                    mode.as_str(),
                    planned.mode.as_str()
                ),
            ));
        }
        out.insert(planned.path.clone(), RenderedFile { mode, bytes });
    }

    if !by_path.is_empty() {
        let extra: Vec<_> = by_path.keys().cloned().collect();
        return Err(RenderError::new(
            "render.extra_templates",
            format!("catalog has templates not in plan: {extra:?}"),
        ));
    }

    Ok(out)
}

fn map_mode(mode: CatalogFileMode) -> FileMode {
    match mode {
        CatalogFileMode::File => FileMode::File,
        CatalogFileMode::Executable => FileMode::Executable,
        CatalogFileMode::Directory => FileMode::Directory,
    }
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
    fn deterministic_map() {
        let plan = sample_plan();
        let cat = stub_catalog();
        let a = render(&plan, &cat).unwrap();
        let b = render(&plan, &cat).unwrap();
        assert_eq!(a, b);
        assert!(a.contains_key("Cargo.toml"));
        assert!(a.contains_key("src/main.rs"));
        let cargo = &a["Cargo.toml"];
        assert!(String::from_utf8_lossy(&cargo.bytes).contains("example-cli"));
    }
}
