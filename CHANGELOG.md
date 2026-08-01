# Changelog

All notable changes to `foundry` are recorded here. Format loosely follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] — v1 scaffold ship

Initial v1 delivery: PHASE-01 through PHASE-05 (MS-001 through MS-021)
complete. See `docs/evidence/MS-020-ship.md` for the owner ship decision and
residual ledger, and `docs/AUTHORITY.md` for the pinned spec/plan sources.

### Added

- `foundry validate|plan|generate` — pure Construct plan-as-contract pipeline
  with write-free `validate`/`plan` and stage → verify → exclusive-place
  `generate` (REQ-040, REQ-050/051/053).
- Closed embedded catalog (`core`, `cli`, `tui`, `hooks`, `secrets`,
  `distribution` units) with content-addressed digests (REQ-060).
- Profile composition with canonical ordering, `requires` edges, and
  topological-sort hard-fail on cycles (REQ-063, MS-009).
- Tiered verify runner with sanitized environment, timeout, and cargo
  fallbacks when `just` is unavailable (MS-012).
- Generated AI-native Core surface: `AGENTS.md`, `quality-gates` /
  `add-subcommand` skills, and `add-tui-screen` when the `tui` profile is
  selected (REQ-088/101/104).
- Foundry product skills (`plan-generate`, `catalog-inspect`,
  `foundry-quality-gates`) with a REQ-088 command-surface join back to the
  frozen justfile/CI fixtures.
- REQ-150/151 acceptance automation (named `cargo test` scenarios) and
  SPK-100/101/102/104 freeze gates.

### Known residuals (see `docs/evidence/MS-020-ship.md`)

- REQ-152 agent operability trial: not run in this delivery; owners may run
  manually.
- `cargo-dist` release automation: scaffold only, not wired to a real release
  pipeline.
- Windows support: permanently out of scope (L3 / REQ-003).
