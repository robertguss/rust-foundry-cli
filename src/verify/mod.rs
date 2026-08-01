//! Tiered cargo/just runners on staged trees (PHASE-03 / MS-012).
//!
//! Command surface freeze fixtures: `docs/freeze/command-surface-justfile.txt`
//! and `docs/freeze/command-surface-cargo.txt` (REQ-088 join).
//!
//! Isolation (normative):
//! - Sanitized environment subset only (no host `RUSTFLAGS` / `CARGO_TARGET_DIR`)
//! - Per-run wall-clock timeout (fail-closed)

use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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

/// Environment keys that MUST NOT leak from the host into verify (MS-012.1).
pub const STRIPPED_ENV_KEYS: &[&str] = &[
    "RUSTFLAGS",
    "CARGO_TARGET_DIR",
    "CARGO_ENCODED_RUSTFLAGS",
    "RUSTC_WRAPPER",
    "RUSTC_WORKSPACE_WRAPPER",
    "CARGO_BUILD_RUSTFLAGS",
];

/// Environment keys retained for cargo/rustup to function.
pub const RETAINED_ENV_KEYS: &[&str] = &[
    "PATH",
    "HOME",
    "CARGO_HOME",
    "RUSTUP_HOME",
    "USER",
    "TMPDIR",
    "TERM",
    "LANG",
    "LC_ALL",
];

/// Default/strict verify runner using just-or-cargo fallbacks.
#[derive(Debug, Clone)]
pub struct TieredVerify {
    /// Verify mode.
    pub mode: VerifyMode,
    /// Wall-clock timeout in seconds for the entire primary gate (0 = fail immediately as misconfig).
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

/// Run an arbitrary argv under sanitized env + wall-clock timeout (testable surface).
///
/// Used by [`TieredVerify`] and by env-hygiene / timeout fixtures.
pub fn run_argv_sanitized(stage_path: &Path, argv: &[&str], timeout_secs: u64) -> VerifyOutcome {
    if argv.is_empty() {
        return VerifyOutcome::Fail {
            message: "verify argv empty".into(),
        };
    }
    if timeout_secs == 0 {
        return VerifyOutcome::Fail {
            message: "verify timeout_secs must be > 0 (fail-closed)".into(),
        };
    }
    match run_command_with_timeout(stage_path, argv, Duration::from_secs(timeout_secs)) {
        Ok(status) if status.success() => VerifyOutcome::Pass,
        Ok(status) => VerifyOutcome::Fail {
            message: format!(
                "verify command `{}` failed with status {status}",
                argv.join(" ")
            ),
        },
        Err(RunError::Timeout) => VerifyOutcome::Fail {
            message: format!(
                "verify timeout after {timeout_secs}s running `{}` (fail-closed)",
                argv.join(" ")
            ),
        },
        Err(RunError::Spawn(e)) => VerifyOutcome::Fail {
            message: format!("failed to spawn `{}`: {e}", argv.join(" ")),
        },
        Err(RunError::Wait(e)) => VerifyOutcome::Fail {
            message: format!("failed waiting for `{}`: {e}", argv.join(" ")),
        },
    }
}

fn run_primary_gate(stage_path: &Path, timeout_secs: u64) -> VerifyOutcome {
    // Prefer `just check` when justfile + just exist.
    let justfile = stage_path.join("justfile");
    if justfile.is_file() && command_exists("just") {
        match run_argv_sanitized(stage_path, &["just", "check"], timeout_secs) {
            VerifyOutcome::Pass => return VerifyOutcome::Pass,
            VerifyOutcome::Fail { message } if message.contains("timeout") => {
                return VerifyOutcome::Fail { message };
            }
            VerifyOutcome::Fail { message } if message.contains("failed to spawn") => {
                // Fall through to cargo fallbacks.
                let _ = message;
            }
            VerifyOutcome::Fail { message } => {
                // just ran but failed — still a gate failure (not silent fallthrough to skip).
                // Fall through to cargo fallbacks when just is broken mid-run? Spec says
                // prefer just when present; if just check fails, that is the gate result.
                return VerifyOutcome::Fail { message };
            }
        }
    }

    // Shared wall-clock budget across cargo fallback steps.
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    for cmd in CARGO_FALLBACK_CMDS {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return VerifyOutcome::Fail {
                message: format!(
                    "verify timeout after {timeout_secs}s before finishing primary gate (fail-closed)"
                ),
            };
        }
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let step_timeout = remaining.as_secs().max(1);
        match run_argv_sanitized(stage_path, &parts, step_timeout) {
            VerifyOutcome::Pass => {}
            fail => return fail,
        }
    }
    VerifyOutcome::Pass
}

#[derive(Debug)]
enum RunError {
    Timeout,
    Spawn(std::io::Error),
    Wait(std::io::Error),
}

fn run_command_with_timeout(
    stage_path: &Path,
    argv: &[&str],
    timeout: Duration,
) -> Result<ExitStatus, RunError> {
    let mut cmd = Command::new(argv[0]);
    if argv.len() > 1 {
        cmd.args(&argv[1..]);
    }
    cmd.current_dir(stage_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env_clear()
        .envs(sanitized_env());

    let mut child = cmd.spawn().map_err(RunError::Spawn)?;
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(RunError::Timeout);
                }
                thread::sleep(Duration::from_millis(20));
            }
            Err(e) => return Err(RunError::Wait(e)),
        }
    }
}

fn command_exists(name: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {name} >/dev/null 2>&1")])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Build the sanitized environment for verify subprocesses.
///
/// Explicitly excludes [`STRIPPED_ENV_KEYS`] even if present in the host process.
pub fn sanitized_env() -> Vec<(String, String)> {
    let mut env = Vec::new();
    for key in RETAINED_ENV_KEYS {
        if STRIPPED_ENV_KEYS.contains(key) {
            continue;
        }
        if let Ok(v) = std::env::var(key) {
            env.push(((*key).to_string(), v));
        }
    }
    // Defense in depth: never copy stripped keys.
    debug_assert!(
        env.iter()
            .all(|(k, _)| !STRIPPED_ENV_KEYS.iter().any(|s| s == k))
    );
    env
}

/// True if `key` is stripped from verify env (host leak prevention).
pub fn is_stripped_env_key(key: &str) -> bool {
    STRIPPED_ENV_KEYS
        .iter()
        .any(|k| k.eq_ignore_ascii_case(key))
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
