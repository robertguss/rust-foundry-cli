//! MS-012.1: verify env hygiene + wall-clock timeout (real shipped path).

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use foundry::catalog::stub_catalog;
use foundry::generate::generate_with;
use foundry::spec::{CliOverrides, VerifyMode, normalize_effective_inputs};
use foundry::verify::{
    ForcedFail, STRIPPED_ENV_KEYS, TieredVerify, VerifyHook, VerifyOutcome, is_stripped_env_key,
    run_argv_sanitized, sanitized_env,
};

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

/// Edition 2024: env mutation is unsafe; tests document intentional host pollution.
fn set_env(key: &str, val: &str) {
    // SAFETY: single-threaded test pollution for hygiene fixtures only.
    unsafe { std::env::set_var(key, val) };
}

fn clear_env(key: &str) {
    // SAFETY: restore host env after pollution fixtures.
    unsafe { std::env::remove_var(key) };
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
    set_env("RUSTFLAGS", "--cfg foundry_host_pollution_must_not_leak");
    set_env(
        "CARGO_TARGET_DIR",
        "/tmp/foundry-host-target-dir-must-not-leak",
    );
    set_env("CARGO_ENCODED_RUSTFLAGS", "host-encoded-must-not-leak");

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

    clear_env("RUSTFLAGS");
    clear_env("CARGO_TARGET_DIR");
    clear_env("CARGO_ENCODED_RUSTFLAGS");
}

#[test]
fn host_rustflags_cannot_affect_verify_subprocess() {
    let dir = sandbox("env-hygiene");
    set_env("RUSTFLAGS", "--cfg foundry_host_pollution_must_not_leak");
    set_env(
        "CARGO_TARGET_DIR",
        dir.join("host-target").to_string_lossy().as_ref(),
    );

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

    clear_env("RUSTFLAGS");
    clear_env("CARGO_TARGET_DIR");
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
