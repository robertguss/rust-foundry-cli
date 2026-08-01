# MS-002 — Spec parse and validate command

| Field | Value |
| ----- | ----- |
| Milestone | MS-002 |
| Phase | PHASE-01 |
| Date | 2026-08-01 |
| Status | Complete (local acceptance) |

## Outcome

TOML schema 1 parse; unknown keys hard-fail; archetype `cli` only; secret
field-name denylist (REQ-033); `foundry validate --spec` with optional
`--name` / `--dest` / `--verify` overrides (REQ-034 surface).

## Acceptance evidence (plan)

| Criterion | Evidence |
| --------- | -------- |
| Fixture specs pass/fail (REQ-030..033) | `tests/spec_validate.rs` |
| Explicit `--spec` / exit codes | CLI tests in same file |
| Denylist documented | `src/spec/denylist.rs`, product AGENTS.md |

## Secret field denylist (REQ-033)

Case-insensitive field **names** (top-level or nested):

`password`, `secret`, `token`, `api_key`, `private_key`, `access_key`, `client_secret`

## Next

MS-003 — Pure Construct `plan` command (write-free; overrides in Construct).
