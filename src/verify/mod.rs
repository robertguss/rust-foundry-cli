//! Tiered cargo/just runners on staged trees.
//!
//! PHASE-01: pluggable stub hook for lifecycle tests. Real default/strict
//! tiers land in PHASE-03 (MS-012).

use std::path::Path;

/// Outcome of a verify hook run against a stage path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyOutcome {
    /// Verification passed (or skipped).
    Pass,
    /// Verification failed; generate must not place.
    Fail {
        /// Human-readable reason.
        message: String,
    },
}

/// Pluggable verify hook (lifecycle-preserving).
pub trait VerifyHook {
    /// Run against the staged project tree.
    fn run(&self, stage_path: &Path) -> VerifyOutcome;
}

/// Always-pass stub (default for `--verify none` and PHASE-01 generate).
#[derive(Debug, Default, Clone, Copy)]
pub struct AlwaysPass;

impl VerifyHook for AlwaysPass {
    fn run(&self, _stage_path: &Path) -> VerifyOutcome {
        VerifyOutcome::Pass
    }
}

/// Injectable fail hook for lifecycle tests (no real cargo).
#[derive(Debug, Clone)]
pub struct ForcedFail {
    /// Failure message.
    pub message: String,
}

impl VerifyHook for ForcedFail {
    fn run(&self, _stage_path: &Path) -> VerifyOutcome {
        VerifyOutcome::Fail {
            message: self.message.clone(),
        }
    }
}

/// Select PHASE-01 stub behavior from effective verify mode.
///
/// Real runners in PHASE-03 replace this mapping. `none` → always pass;
/// `default`/`strict` → always pass stub for now (lifecycle preserved).
pub fn phase01_stub_hook(_mode: crate::spec::VerifyMode) -> AlwaysPass {
    AlwaysPass
}
