//! Load embedded catalog and digests (pure load after embed).
//!
//! **OQ-201:** embed via `include_dir` (see `docs/evidence/OQ-201-embed.md`).
//! Closed catalog offline (REQ-060). Digest is SHA-256 over canonical
//! unit path + file bytes (not foundry version alone).

use std::collections::BTreeMap;

use include_dir::{Dir, include_dir};
use toml::Value;

use crate::plan::{FileMode, content_sha256};

/// Embedded `catalog/` tree (compile-time).
static CATALOG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/catalog");

/// Placeholder used when catalog is not yet loaded (tests only).
pub const STUB_CATALOG_DIGEST: &str = "stub-catalog-v0-unembedded";

/// Emit mode recorded on a catalog template entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogFileMode {
    /// Regular file.
    File,
    /// Executable file.
    Executable,
    /// Directory entry.
    Directory,
}

/// One catalog template file (relative path + body).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogFile {
    /// Relative path under project root.
    pub path: String,
    /// Emit mode.
    pub mode: CatalogFileMode,
    /// Template body (UTF-8).
    pub body: String,
}

/// In-memory catalog view for Construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogView {
    /// Catalog content digest for this view.
    pub digest: String,
    /// Template files in deterministic order.
    pub files: Vec<CatalogFile>,
}

/// Parsed catalog unit manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitManifest {
    /// Unit id (`core`, `cli`, …).
    pub id: String,
    /// Kind: core | archetype | profile.
    pub kind: String,
    /// Human description.
    pub description: String,
    /// Required unit ids.
    pub requires: Vec<String>,
    /// Files relative path → template body.
    pub files: Vec<(String, String)>,
}

/// Full embedded catalog index.
#[derive(Debug, Clone)]
pub struct EmbeddedCatalog {
    /// Content digest of entire authoring tree.
    pub digest: String,
    /// Units keyed by id.
    pub units: BTreeMap<String, UnitManifest>,
}

/// Load and parse the embedded closed catalog (offline, pure once embedded).
pub fn load_embedded_catalog() -> Result<EmbeddedCatalog, CatalogError> {
    let mut units = BTreeMap::new();
    let units_dir = CATALOG_DIR.get_dir("units").ok_or_else(|| {
        CatalogError::new("catalog.missing_units", "embedded catalog has no units/")
    })?;

    for entry in units_dir.dirs() {
        let unit_id = entry
            .path()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        if unit_id.is_empty() {
            continue;
        }
        let manifest_file = find_file(entry, "MANIFEST.toml").ok_or_else(|| {
            CatalogError::new(
                "catalog.missing_manifest",
                format!("unit {unit_id} missing MANIFEST.toml"),
            )
        })?;
        let text = std::str::from_utf8(manifest_file.contents()).map_err(|e| {
            CatalogError::new(
                "catalog.manifest_utf8",
                format!("unit {unit_id} MANIFEST not utf8: {e}"),
            )
        })?;
        let unit = parse_unit_manifest(text, entry)?;
        if unit.id != unit_id {
            return Err(CatalogError::new(
                "catalog.id_mismatch",
                format!("dir {unit_id} vs manifest id {}", unit.id),
            ));
        }
        units.insert(unit.id.clone(), unit);
    }

    let digest = compute_catalog_digest(&units);
    Ok(EmbeddedCatalog { digest, units })
}

/// Build a [`CatalogView`] for the given composition unit ids (order preserved).
pub fn catalog_view_for_units(
    catalog: &EmbeddedCatalog,
    unit_ids: &[String],
) -> Result<CatalogView, CatalogError> {
    let mut files: BTreeMap<String, CatalogFile> = BTreeMap::new();
    for id in unit_ids {
        let unit = catalog.units.get(id).ok_or_else(|| {
            CatalogError::new("catalog.unknown_unit", format!("unknown unit id {id:?}"))
        })?;
        for (path, body) in &unit.files {
            files.insert(
                path.clone(),
                CatalogFile {
                    path: path.clone(),
                    mode: CatalogFileMode::File,
                    body: body.clone(),
                },
            );
        }
    }
    Ok(CatalogView {
        digest: catalog.digest.clone(),
        files: files.into_values().collect(),
    })
}

/// Default pure-CLI composition view: core + cli (replaces pre-MS-007 stub content).
pub fn default_cli_catalog_view() -> CatalogView {
    match load_embedded_catalog() {
        Ok(cat) => catalog_view_for_units(&cat, &["core".into(), "cli".into()])
            .unwrap_or_else(|_| legacy_stub_catalog()),
        Err(_) => legacy_stub_catalog(),
    }
}

/// Alias retained for Construct callers (now backed by embed when available).
pub fn stub_catalog() -> CatalogView {
    default_cli_catalog_view()
}

/// Catalog error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogError {
    /// Stable code.
    pub code: &'static str,
    /// Message.
    pub message: String,
}

impl CatalogError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for CatalogError {}

fn parse_unit_manifest(text: &str, unit_dir: &Dir<'_>) -> Result<UnitManifest, CatalogError> {
    let table: toml::map::Map<String, Value> = toml::from_str(text)
        .map_err(|e| CatalogError::new("catalog.manifest_parse", e.to_string()))?;

    let id = table
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| CatalogError::new("catalog.manifest_field", "id required"))?
        .to_string();
    let kind = table
        .get("kind")
        .and_then(|v| v.as_str())
        .unwrap_or("profile")
        .to_string();
    let description = table
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let requires = table
        .get("requires")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    let mut files = Vec::new();
    if let Some(Value::Array(entries)) = table.get("files") {
        for entry in entries {
            let Some(t) = entry.as_table() else { continue };
            let path = t
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CatalogError::new("catalog.manifest_field", "files.path"))?
                .to_string();
            let template_rel = t
                .get("template")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CatalogError::new("catalog.manifest_field", "files.template"))?;
            let file = find_file(unit_dir, template_rel).ok_or_else(|| {
                CatalogError::new(
                    "catalog.missing_template",
                    format!("unit {id}: missing template {template_rel}"),
                )
            })?;
            let body = std::str::from_utf8(file.contents())
                .map_err(|e| CatalogError::new("catalog.template_utf8", e.to_string()))?
                .to_string();
            files.push((path, body));
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(UnitManifest {
        id,
        kind,
        description,
        requires,
        files,
    })
}

fn compute_catalog_digest(units: &BTreeMap<String, UnitManifest>) -> String {
    let mut chunks = Vec::new();
    for (id, unit) in units {
        chunks.push(format!("unit:{id}:kind:{}", unit.kind));
        for req in &unit.requires {
            chunks.push(format!("unit:{id}:requires:{req}"));
        }
        for (path, body) in &unit.files {
            chunks.push(format!(
                "unit:{id}:file:{path}:{}",
                content_sha256(body.as_bytes())
            ));
        }
    }
    content_sha256(chunks.join("\n").as_bytes())
}

/// Pre-embed fallback (should not be hit once catalog/ is complete).
fn legacy_stub_catalog() -> CatalogView {
    CatalogView {
        digest: STUB_CATALOG_DIGEST.to_string(),
        files: vec![
            CatalogFile {
                path: "Cargo.toml".into(),
                mode: CatalogFileMode::File,
                body: "[package]\nname = \"{{name}}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"
                    .into(),
            },
            CatalogFile {
                path: "src/main.rs".into(),
                mode: CatalogFileMode::File,
                body: "fn main() { println!(\"{{name}}\"); }\n".into(),
            },
        ],
    }
}

/// Map catalog file mode to plan file mode (used by render).
pub fn to_plan_mode(mode: CatalogFileMode) -> FileMode {
    match mode {
        CatalogFileMode::File => FileMode::File,
        CatalogFileMode::Executable => FileMode::Executable,
        CatalogFileMode::Directory => FileMode::Directory,
    }
}

/// Locate a file under `dir` by relative path (include_dir stores full paths).
fn find_file<'a>(dir: &'a Dir<'a>, rel: &str) -> Option<&'a include_dir::File<'a>> {
    let rel = rel.trim_start_matches("./");
    // Direct relative lookup first.
    if let Some(f) = dir.get_file(rel) {
        return Some(f);
    }
    // include_dir file paths are often rooted at the embed root (e.g. units/cli/...).
    for f in dir.files() {
        let p = f.path();
        if p.ends_with(rel) || p.file_name().and_then(|n| n.to_str()) == Some(rel) {
            return Some(f);
        }
        // template path like "templates/Cargo.toml"
        if p.to_string_lossy().ends_with(rel) {
            return Some(f);
        }
    }
    // Recurse into subdirs
    for sub in dir.dirs() {
        if let Some(f) = find_file(sub, rel) {
            return Some(f);
        }
    }
    None
}

/// Sample schema-1 Project Spec TOML (convenience; MS-007.4).
pub fn sample_spec_toml(name: &str, profiles: &[&str]) -> String {
    let profiles_lit = if profiles.is_empty() {
        "[]".to_string()
    } else {
        let inner = profiles
            .iter()
            .map(|p| format!("\"{p}\""))
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{inner}]")
    };
    format!(
        r#"# Sample Project Spec (schema = 1) — generated by `foundry sample-spec`
# Validate with: foundry validate --spec this-file.toml

schema = 1
name = "{name}"
description = "Sample pure-CLI project"
archetype = "cli"
destination = "./{name}"
profiles = {profiles_lit}
# verify = "default"
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embed_loads_all_units() {
        let cat = load_embedded_catalog().unwrap();
        for id in ["core", "cli", "tui", "hooks", "secrets", "distribution"] {
            assert!(cat.units.contains_key(id), "missing unit {id}");
        }
        assert_eq!(cat.digest.len(), 64);
        assert_ne!(cat.digest, STUB_CATALOG_DIGEST);
    }

    #[test]
    fn default_cli_view_has_cargo() {
        let view = default_cli_catalog_view();
        assert!(view.files.iter().any(|f| f.path == "Cargo.toml"));
        assert!(view.files.iter().any(|f| f.path == "AGENTS.md"));
    }
}
