# PHASE-01 exit checklist evidence (MS-006.3)

Date: 2026-08-01  
Tree: post MS-003..MS-006 implementation on `main`.

| # | Criterion | Evidence | Status |
| - | --------- | -------- | ------ |
| 1 | Product crate builds on Linux; no Windows jobs | `cargo build` / `cargo test` on Linux; no Windows target or GHA windows jobs in repo | **pass** |
| 2 | validate/plan/generate exist; non-interactive; exit codes | CLI integration tests: `tests/spec_validate.rs`, `tests/plan_cli.rs`, `tests/generate_lifecycle.rs` | **pass** |
| 3 | Same effective inputs → equal plan_sha256 | `tests/plan_integration.rs`, `tests/construct.rs` | **pass** |
| 4 | SPK-100 golden + SPK-101 matrix | `tests/snapshots/plan/*`, `tests/plan_snapshots.rs`, `tests/spk101_matrix.rs` | **pass** |
| 5 | Path jail hard-fail escapes | `tests/path_jail_denylist.rs`, construct path jail unit tests | **pass** |
| 6 | plan-package purity | `tests/purity.rs` (no fsx/generate/cli imports in `src/plan/`) | **pass** |
| 7 | Stable error codes | `docs/error-codes.md`, `tests/error_codes.rs` | **pass** |
| 8 | No partial place on failure; stage path on fail | `tests/generate_lifecycle.rs`, `tests/spk101_matrix.rs` (verify fail / refuse) | **pass** |
| 9 | REQ-130 module map | `tests/module_map.rs`; `src/{cli,spec,catalog,resolve,plan,render,fsx,generate,verify,report}` | **pass** |

## Residuals

- **Cross-device EXDEV place:** fail-closed implemented (`fsx.cross_device`); full multi-device CI exercise not run on single-FS runners (documented; no silent copy+swap).
- **Catalog stub:** PHASE-01 uses stub catalog until MS-007 embed; digests redacted in goldens.
- **Verify tiers:** PHASE-01 stub only; real default verify is PHASE-03.

## Commands run for this gate

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
