//! Tiered cargo/just runners on staged trees (PHASE-03 / MS-012).
//!
//! Command surface freeze fixtures: `docs/freeze/command-surface-justfile.txt`
//! and `docs/freeze/command-surface-cargo.txt` (REQ-088 join).

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::spec::VerifyMode;

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

/// Always-pass stub.
#[derive(Debug, Default, Clone, Copy)]
pub struct AlwaysPass;

impl VerifyHook for AlwaysPass {
    fn run(&self, _stage_path: &Path) -> VerifyOutcome {
        VerifyOutcome::Pass
    }
}

/// Injectable fail hook for lifecycle tests.
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

/// Frozen primary-gate command lines (must match freeze fixtures / justfile).
pub const JUST_CHECK_LINE: &str =
    "cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test";

/// Cargo fallback sequence when `just` is missing (SPK-103).
pub const CARGO_FALLBACK_CMDS: &[&str] = &[
    "cargo fmt --check",
    "cargo clippy --all-targets -- -D warnings",
    "cargo test",
];

/// Default/strict verify runner using just-or-cargo fallbacks.
#[derive(Debug, Clone)]
pub struct TieredVerify {
    /// Verify mode.
    pub mode: VerifyMode,
    /// Optional timeout seconds (0 = no limit in tests).
    pub timeout_secs: u64,
}

impl Default for TieredVerify {
    fn default() -> Self {
        Self {
            mode: VerifyMode::Default,
            timeout_secs: 600,
        }
    }
}

impl VerifyHook for TieredVerify {
    fn run(&self, stage_path: &Path) -> VerifyOutcome {
        match self.mode {
            VerifyMode::None => VerifyOutcome::Pass,
            VerifyMode::Default | VerifyMode::Strict => {
                run_primary_gate(stage_path, self.timeout_secs)
            }
        }
    }
}

fn run_primary_gate(stage_path: &Path, _timeout_secs: u64) -> VerifyOutcome {
    // Prefer `just check` when justfile + just exist.
    let justfile = stage_path.join("justfile");
    if justfile.is_file() && command_exists("just") {
        let status = Command::new("just")
            .arg("check")
            .current_dir(stage_path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env_clear()
            .envs(sanitized_env())
            .status();
        match status {
            Ok(s) if s.success() => return VerifyOutcome::Pass,
            Ok(s) => {
                return VerifyOutcome::Fail {
                    message: format!("just check failed with status {s}"),
                };
            }
            Err(e) => {
                // Fall through to cargo fallbacks.
                let _ = e;
            }
        }
    }

    for cmd in CARGO_FALLBACK_CMDS {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let mut c = Command::new(parts[0]);
        c.args(&parts[1..])
            .current_dir(stage_path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env_clear()
            .envs(sanitized_env());
        match c.status() {
            Ok(s) if s.success() => {}
            Ok(s) => {
                return VerifyOutcome::Fail {
                    message: format!("primary gate step `{cmd}` failed with status {s}"),
                };
            }
            Err(e) => {
                return VerifyOutcome::Fail {
                    message: format!("failed to spawn `{cmd}`: {e}"),
                };
            }
        }
    }
    VerifyOutcome::Pass
}

fn command_exists(name: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {name} >/dev/null 2>&1")])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn sanitized_env() -> Vec<(String, String)> {
    // Minimal PATH + HOME + cargo/rustup so runners work; strip secrets-ish keys.
    let mut env = Vec::new();
    for key in [
        "PATH",
        "HOME",
        "CARGO_HOME",
        "RUSTUP_HOME",
        "USER",
        "TMPDIR",
        "TERM",
    ] {
        if let Ok(v) = std::env::var(key) {
            env.push((key.to_string(), v));
        }
    }
    env
}

/// Select verify hook for generate.
pub fn hook_for_mode(mode: VerifyMode) -> Box<dyn VerifyHook> {
    match mode {
        VerifyMode::None => Box::new(AlwaysPass),
        VerifyMode::Default | VerifyMode::Strict => Box::new(TieredVerify {
            mode,
            timeout_secs: 600,
        }),
    }
}

/// Back-compat name used by generate module.
pub fn phase01_stub_hook(mode: VerifyMode) -> Box<dyn VerifyHook> {
    hook_for_mode(mode)
}

// Keep Duration import used if we add timeout later.
#[allow(dead_code)]
fn _timeout(d: Duration) -> Duration {
    d
}
