# MS-001 — Product repository bootstrap

| Field | Value |
| ----- | ----- |
| Milestone | MS-001 |
| Phase | PHASE-01 |
| Date | 2026-08-01 |
| Status | Complete (local acceptance) |

## Outcome

Rust crate skeleton, Linux-only CI, §10.1 module map stubs, dual license,
README/AGENTS, authority copies of revised spec/plan/Blueprint.

## Acceptance evidence (plan)

| Criterion | Evidence |
| --------- | -------- |
| `cargo test` passes | Run in this milestone; see CI and local `just ci` / `cargo test` |
| No Windows workflow | `.github/workflows/ci.yml` uses `ubuntu-latest` only |

## Deliverables

- Package `foundry` (edition 2024, `rust-version = "1.85"`)
- Binary `foundry` with `version` subcommand
- Modules: `cli`, `spec`, `catalog`, `resolve`, `plan`, `render`, `fsx`,
  `generate`, `verify`, `report`
- Purity test: `plan` must not import write-path modules
- Docs: `AUTHORITY.md` pins research commits
- Example fixture: `examples/minimal-cli.toml`

## Next

MS-002 — Spec parse and `validate` command (REQ-030..033).
