//! Construct immutable Generation Plan (pure).
//!
//! Plan-as-contract (REQ-034/040). Must not import write-path modules
//! (`fsx`, `generate`, `cli`). Model: MS-003.3; construct: MS-003.4;
//! golden freeze: MS-004.

mod digest;
mod model;

pub use digest::{compute_plan_sha256, content_sha256, seal_plan};
pub use model::{
    DependencyDelta, DestinationPolicy, FileMode, NormalizedSpecRecord, Plan, PlannedFile,
    ResolvedComposition, STUB_CATALOG_DIGEST,
};
