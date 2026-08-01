//! Immutable Generation Plan data model (REQ-041 / §11.2).
//!
//! Exact **JSON field names** remain provisional until MS-004 (SPK-100 / OQ-200).
//! The **element set** is normative now.

use crate::spec::VerifyMode;

/// Placeholder catalog digest (re-export of catalog stub token for plan callers).
pub use crate::catalog::STUB_CATALOG_DIGEST;

/// Unix-style file mode for a planned path (emit semantics).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    /// Regular file (non-executable), mode 0o644.
    File,
    /// Executable file, mode 0o755.
    Executable,
    /// Directory entry, mode 0o755.
    Directory,
}

impl FileMode {
    /// Canonical short label for reports / digests.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Executable => "executable",
            Self::Directory => "directory",
        }
    }
}

/// One planned emit path with content digest (REQ-041 planned files element).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedFile {
    /// Relative path under destination root (forward slashes).
    pub path: String,
    /// File kind / permission class.
    pub mode: FileMode,
    /// SHA-256 hex digest of planned file bytes (empty string for directories).
    pub content_digest: String,
}

/// Dependency / Cargo.toml delta recorded on the plan (REQ-041).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyDelta {
    /// Crate name.
    pub name: String,
    /// Version requirement string (e.g. `"1"`, `"0.8"`).
    pub version_req: String,
    /// Optional feature flags.
    pub features: Vec<String>,
    /// Whether this is a dev-dependency.
    pub dev: bool,
}

/// Format dependency deltas as `[dependencies]`/`[dev-dependencies]` TOML lines.
///
/// Shared by Construct's placeholder expansion and pure render so the bytes
/// staged always match the digest recorded on the plan (single source of
/// truth for how a [`DependencyDelta`] becomes a Cargo.toml line).
pub fn format_dependency_lines(deltas: &[DependencyDelta], dev: bool) -> String {
    deltas
        .iter()
        .filter(|d| d.dev == dev)
        .map(|d| {
            if d.features.is_empty() {
                format!("{} = \"{}\"", d.name, d.version_req)
            } else {
                let features = d
                    .features
                    .iter()
                    .map(|f| format!("\"{f}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{} = {{ version = \"{}\", features = [{features}] }}",
                    d.name, d.version_req
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Destination policy decision (predicate per REQ-051; no place yet).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestinationPolicy {
    /// Destination path does not exist (admissible for generate).
    Missing,
    /// Destination exists and is an empty directory (admissible).
    EmptyAdmissible,
    /// Destination must be refused (non-empty, file, symlink, etc.).
    Refuse {
        /// Stable reason token for reports.
        reason: String,
    },
}

impl DestinationPolicy {
    /// Canonical label for reports / digests.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Missing => "missing",
            Self::EmptyAdmissible => "empty_admissible",
            Self::Refuse { .. } => "refuse",
        }
    }
}

/// Resolved archetype + ordered profiles (composition identity for Construct).
///
/// Full catalog unit wiring lands in PHASE-02; this is the composition record
/// required by §11.2. Canonical profile order is applied by `resolve` (MS-003.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedComposition {
    /// Archetype id (v1: `cli`).
    pub archetype: String,
    /// Selected profiles in canonical order (not input order).
    pub ordered_profiles: Vec<String>,
    /// Catalog unit ids participating (core + archetype + profiles).
    pub unit_ids: Vec<String>,
}

/// Snapshot of effective / normalized Project Spec intent on the plan.
///
/// Distinct from [`crate::spec::EffectiveInputs`] so plan serialization can
/// evolve without coupling to the Construct input type field-for-field forever.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedSpecRecord {
    /// Schema version.
    pub schema: i64,
    /// Effective project name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Archetype id.
    pub archetype: String,
    /// Effective destination path string.
    pub destination: String,
    /// Profiles as selected (pre-canonical-order input list is not required here;
    /// composition holds ordered profiles).
    pub profiles: Vec<String>,
    /// Effective verify mode.
    pub verify: VerifyMode,
    /// Spec source provenance label.
    pub source: String,
}

/// Immutable Generation Plan — every REQ-041 / §11.2 element is a field.
///
/// Construct (MS-003.4) is the sole producer; plan JSON field names freeze at MS-004.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// Foundry package version string.
    pub foundry_version: String,
    /// Closed-catalog content digest (stub until MS-007).
    pub catalog_digest: String,
    /// Validated / normalized spec intent record.
    pub normalized_spec: NormalizedSpecRecord,
    /// Resolved archetype + ordered profiles (+ unit ids).
    pub composition: ResolvedComposition,
    /// Exact planned emit set (path, mode, content digest).
    pub planned_files: Vec<PlannedFile>,
    /// Dependency / Cargo.toml deltas.
    pub dependency_deltas: Vec<DependencyDelta>,
    /// AI-native paths planned (AGENTS.md, `.agents/skills/…`).
    pub ai_native_paths: Vec<String>,
    /// Post-stage verify policy.
    pub verify: VerifyMode,
    /// Destination policy decision (missing / empty-admissible / refuse).
    pub destination_policy: DestinationPolicy,
    /// Integrity hash over semantic plan content (excludes this field itself).
    pub plan_sha256: String,
    /// Non-binding notes (must not change planned file set meaning).
    pub warnings: Vec<String>,
}

impl Plan {
    /// Assert every REQ-041 element is present in a structural sense.
    ///
    /// Used by fixtures: empty `foundry_version`, empty `catalog_digest`, or
    /// empty `plan_sha256` is treated as a missing element.
    pub fn assert_elements_complete(&self) -> Result<(), &'static str> {
        if self.foundry_version.is_empty() {
            return Err("missing foundry_version");
        }
        if self.catalog_digest.is_empty() {
            return Err("missing catalog_digest");
        }
        if self.normalized_spec.name.is_empty() {
            return Err("missing normalized_spec.name");
        }
        if self.normalized_spec.archetype.is_empty() {
            return Err("missing normalized_spec.archetype");
        }
        if self.normalized_spec.destination.is_empty() {
            return Err("missing normalized_spec.destination");
        }
        if self.composition.archetype.is_empty() {
            return Err("missing composition.archetype");
        }
        if self.composition.unit_ids.is_empty() {
            return Err("missing composition.unit_ids");
        }
        // planned_files may be empty only for pathological stubs; still a present
        // element (Vec exists). Same for dependency_deltas, ai_native_paths, warnings.
        let _ = &self.planned_files;
        let _ = &self.dependency_deltas;
        let _ = &self.ai_native_paths;
        let _ = &self.verify;
        let _ = &self.destination_policy;
        if self.plan_sha256.is_empty() {
            return Err("missing plan_sha256");
        }
        if self.plan_sha256.len() != 64 || !self.plan_sha256.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err("plan_sha256 must be 64 lowercase hex chars");
        }
        let _ = &self.warnings;
        Ok(())
    }
}
