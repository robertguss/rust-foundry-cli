//! Foundry product library — hybrid generate (`validate` → `plan` → `generate`).
//!
//! Module map follows revised-spec §10.1 (REQ-130). Domain logic lives here;
//! [`cli`] is the clap I/O boundary only.

#![deny(missing_docs)]

pub mod catalog;
pub mod cli;
pub mod fsx;
pub mod generate;
pub mod plan;
pub mod render;
pub mod report;
pub mod resolve;
pub mod spec;
pub mod verify;

/// Package version string (catalog digest lands later).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
