//! MS-012.1: verify env hygiene + wall-clock timeout (real shipped path).

use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use std::time::Instant;

use foundry::catalog::stub_catalog;
use foundry::generate::generate_with;
use foundry::spec::{CliOverrides, VerifyMode, normalize_effective_inputs};
use foundry::verify::{
    ForcedFail, STRIPPED_ENV_KEYS, TieredVerify, VerifyHook, VerifyOutcome, is_stripped_env_key,
    run_argv_sanitized, sanitized_env,
};

/// Serializes host-env mutations across parallel tests in this binary.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn sandbox(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "foundry-verify-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Temporary host env pollution. Restores prior values on drop (including panic).
///
/// Edition 2024: `set_var` / `remove_var` are unsafe. Mutations are held under
/// [`ENV_LOCK`] so parallel tests in this binary cannot interleave.
struct EnvPollution {
    _lock: MutexGuard<'static, ()>,
    previous: Vec<(String, Option<String>)>,
}

impl EnvPollution {
    fn apply(pairs: &[(&str, &str)]) -> Self {
        let lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut previous = Vec::with_capacity(pairs.len());
        for &(key, val) in pairs {
            previous.push((key.to_string(), std::env::var(key).ok()));
            // SAFETY: exclusive under ENV_LOCK; Drop restores prior state.
            unsafe { std::env::set_var(key, val) };
        }
        Self {
            _lock: lock,
            previous,
        }
    }
}

impl Drop for EnvPollution {
    fn drop(&mut self) {
        for (key, prev) in self.previous.drain(..).rev() {
            // SAFETY: still hold ENV_LOCK via `_lock`.
            match prev {
                Some(v) => unsafe { std::env::set_var(&key, v) },
                None => unsafe { std::env::remove_var(&key) },
            }
        }
    }
}

#[test]
fn stripped_keys_documented() {
    for k in ["RUSTFLAGS", "CARGO_TARGET_DIR", "CARGO_ENCODED_RUSTFLAGS"] {
        assert!(is_stripped_env_key(k), "{k} must be stripped");
    }
    assert!(STRIPPED_ENV_KEYS.contains(&"RUSTFLAGS"));
    assert!(STRIPPED_ENV_KEYS.contains(&"CARGO_TARGET_DIR"));
}

#[test]
fn sanitized_env_excludes_host_rustflags_and_target_dir() {
    let _pollution = EnvPollution::apply(&[
        ("RUSTFLAGS", "--cfg foundry_host_pollution_must_not_leak"),
        (
            "CARGO_TARGET_DIR",
            "/tmp/foundry-host-target-dir-must-not-leak",
        ),
        ("CARGO_ENCODED_RUSTFLAGS", "host-encoded-must-not-leak"),
    ]);

    let env = sanitized_env();
    let keys: Vec<_> = env.iter().map(|(k, _)| k.as_str()).collect();
    for banned in STRIPPED_ENV_KEYS {
        assert!(
            !keys.iter().any(|k| k == banned),
            "sanitized_env leaked {banned}"
        );
    }
    for (k, v) in &env {
        assert!(
            !v.contains("foundry_host_pollution") && !v.contains("host-target-dir-must-not-leak"),
            "leaked value via {k}={v}"
        );
    }
}

#[test]
fn host_rustflags_cannot_affect_verify_subprocess() {
    let dir = sandbox("env-hygiene");
    let host_target = dir.join("host-target");
    let host_target_s = host_target.to_string_lossy().into_owned();
    let _pollution = EnvPollution::apply(&[
        ("RUSTFLAGS", "--cfg foundry_host_pollution_must_not_leak"),
        ("CARGO_TARGET_DIR", host_target_s.as_str()),
    ]);

    let outcome = run_argv_sanitized(
        &dir,
        &[
            "sh",
            "-c",
            "if [ -n \"$RUSTFLAGS\" ] || [ -n \"$CARGO_TARGET_DIR\" ]; then echo LEAKED; exit 2; fi; exit 0",
        ],
        5,
    );
    assert_eq!(
        outcome,
        VerifyOutcome::Pass,
        "host RUSTFLAGS/CARGO_TARGET_DIR leaked into verify subprocess: {outcome:?}"
    );

    drop(_pollution);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn timeout_fails_closed_no_hang() {
    let dir = sandbox("timeout");
    let start = Instant::now();
    let outcome = run_argv_sanitized(&dir, &["sh", "-c", "sleep 30"], 1);
    let elapsed = start.elapsed();
    match &outcome {
        VerifyOutcome::Fail { message } => {
            assert!(
                message.contains("timeout"),
                "expected timeout message, got {message}"
            );
        }
        other => panic!("expected Fail(timeout), got {other:?}"),
    }
    assert!(
        elapsed.as_secs() < 10,
        "timeout did not kill promptly: {elapsed:?}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn timeout_zero_fails_closed() {
    let dir = sandbox("timeout-zero");
    let outcome = run_argv_sanitized(&dir, &["true"], 0);
    match &outcome {
        VerifyOutcome::Fail { message } => {
            assert!(
                message.contains("timeout_secs"),
                "expected timeout_secs message, got {message}"
            );
        }
        other => panic!("expected Fail, got {other:?}"),
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tier_none_skips() {
    let dir = sandbox("none");
    let hook = TieredVerify {
        mode: VerifyMode::None,
        timeout_secs: 5,
    };
    assert_eq!(hook.run(&dir), VerifyOutcome::Pass);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn verify_fail_never_places_stage_retained() {
    let dir = sandbox("fail-place");
    let dest = dir.join("proj");
    let body = format!(
        r#"
schema = 1
name = "v"
archetype = "cli"
destination = "{}"
profiles = []
verify = "default"
"#,
        dest.display()
    );
    let spec = foundry::spec::parse_spec_str(&body, "<t>").unwrap();
    let inputs = normalize_effective_inputs(spec, CliOverrides::default()).unwrap();
    let err = generate_with(
        &inputs,
        &stub_catalog(),
        &ForcedFail {
            message: "injected".into(),
        },
    )
    .unwrap_err();
    assert_eq!(err.code, "generate.verify_failed");
    assert!(!dest.exists());
    assert!(err.stage_path.as_ref().unwrap().exists());
    let _ = fs::remove_dir_all(err.stage_path.as_ref().unwrap());
    let _ = fs::remove_dir_all(&dir);
}
