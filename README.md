# rust-foundry-cli

**foundry** is an AI-native hybrid foundry for modern Rust projects:
`validate` → `plan` → `generate` (CLI + strong Core + GitHub template surface).

This repository is the **product implementation**. Research, specification, and
planning live in [rust-foundry](https://github.com/robertguss/rust-foundry)
(local sibling: `../rust-foundry`).

## User guide

End-user documentation lives in [`docs/user/`](docs/user/README.md):

- [Installation](docs/user/install.md)
- [Quickstart](docs/user/quickstart.md)
- [Project Spec reference](docs/user/project-spec.md)
- [Command reference](docs/user/commands.md)
- [Catalog and profiles](docs/user/catalog.md)
- [Using a generated project](docs/user/generated-project.md)
- [Troubleshooting](docs/user/troubleshooting.md)

|                       |                                                                         |
| --------------------- | ----------------------------------------------------------------------- |
| **Crate / package**   | `foundry`                                                               |
| **CLI**               | `foundry`                                                               |
| **Rust**              | edition 2024; `rust-version` floor 1.85 (product on stable)             |
| **Hosts**             | Linux required (CI); macOS optional; **no Windows**                     |
| **Current phase**     | **v1 shipped** — PHASE-01 through PHASE-05 complete                     |
| **Current milestone** | All of MS-001 through MS-021 closed; see `docs/evidence/MS-020-ship.md` |

## Specification authority

Do **not** invent product behavior outside these documents:

| Role              | Doc                                                                                          |
| ----------------- | -------------------------------------------------------------------------------------------- |
| Product law       | [`docs/02-definitive-specification-revised.md`](docs/02-definitive-specification-revised.md) |
| Delivery sequence | [`docs/02-implementation-plan-revised.md`](docs/02-implementation-plan-revised.md)           |
| Pins / provenance | [`docs/AUTHORITY.md`](docs/AUTHORITY.md)                                                     |

Agent rules for this repo: [`AGENTS.md`](AGENTS.md).

## Status

v1 delivered — all 5 phases / 21 milestones closed (see `bd list --status
closed` and `docs/evidence/MS-020-ship.md` for the ship record and residual
ledger). Ongoing work is maintenance/hardening tracked as beads, not new
milestone delivery.

| Phase    | Outcome                                                         | Status |
| -------- | --------------------------------------------------------------- | ------ |
| PHASE-01 | CLI, pure Construct, write safety (MS-001..006)                 | Done   |
| PHASE-02 | Embedded catalog, Core emit, profiles/TUI (MS-007..011, MS-021) | Done   |
| PHASE-03 | Verify tiers, GHA, distribution, GH template (MS-012..015)      | Done   |
| PHASE-04 | AI-native surfaces, product skills (MS-016..018)                | Done   |
| PHASE-05 | Acceptance scenarios, ship freeze (MS-019..020)                 | Done   |

## Quickstart

```bash
# From this repo root
cargo test
cargo run -- version
cargo run -- validate --spec ./examples/minimal-cli.toml
cargo run -- plan --spec ./examples/minimal-cli.toml
cargo run -- generate --spec ./examples/minimal-cli.toml --dest ./out
```

## Layout

```text
src/          # single crate; modules per §10.1
catalog/      # closed catalog authoring tree (embedded via include_dir!)
docs/         # implementation authority copies, freeze fixtures, evidence
examples/     # Project Spec fixtures
tests/        # unit/integration + purity checks
```

## Development

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
just ci   # optional convenience
```

## Locks (do not reverse without DEC)

- **macOS + Linux only**; never Windows
- Exclusive place; refuse non-empty; no merge-default generate
- Closed catalog; custom engine
- **AGENTS.md** + `.agents/skills/` only; **no** Claude adapters in Core emit; MCP none
- Plan-as-contract; MS-004 before generate shapes; MS-021 before profile/TUI

See the revised specification for full REQs and the revised plan for phase gates.
