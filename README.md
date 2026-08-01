# rust-foundry-cli

**foundry** is an AI-native hybrid foundry for modern Rust projects:
`validate` → `plan` → `generate` (CLI + strong Core + GitHub template surface).

This repository is the **product implementation**. Research, specification, and
planning live in [rust-foundry](https://github.com/robertguss/rust-foundry)
(local sibling: `../rust-foundry`).

| | |
| - | - |
| **Crate / package** | `foundry` |
| **CLI** | `foundry` |
| **Rust** | edition 2024; `rust-version` floor 1.85 (product on stable) |
| **Hosts** | Linux required (CI); macOS optional; **no Windows** |
| **Current phase** | **PHASE-01** — CLI + Construct + write safety |
| **Current milestone** | **MS-002 complete** → next **MS-003** |

## Specification authority

Do **not** invent product behavior outside these documents:

| Role | Doc |
| ---- | --- |
| Product law | [`docs/02-definitive-specification-revised.md`](docs/02-definitive-specification-revised.md) |
| Delivery sequence | [`docs/02-implementation-plan-revised.md`](docs/02-implementation-plan-revised.md) |
| Pins / provenance | [`docs/AUTHORITY.md`](docs/AUTHORITY.md) |

Agent rules for this repo: [`AGENTS.md`](AGENTS.md).

## Status

PHASE-01 in progress:

- MS-001: Single-crate module map, Linux-only CI, `foundry version`
- MS-002: TOML schema 1 parse; denylist; `foundry validate --spec`

| MS | Outcome | Status |
| -- | ------- | ------ |
| MS-001 | Product repo bootstrap | Done |
| MS-002 | Spec parse + `validate` | Done |
| MS-003 | Pure Construct `plan` (write-free) | Next |
| MS-004 | SPK-100 golden plan freeze | Pending |
| MS-005 | Stage / place / `generate` lifecycle | Pending |
| MS-006 | SPK-101 emptiness/place/jail matrix | Pending |

## Quickstart

```bash
# From this repo root
cargo test
cargo run -- version
cargo run -- validate --spec ./examples/minimal-cli.toml

# After MS-003:
# cargo run -- plan --spec ./examples/minimal-cli.toml
# generate after MS-005:
# cargo run -- generate --spec ./examples/minimal-cli.toml --dest ./out
```

## Layout

```text
src/          # single crate; modules per §10.1
catalog/      # closed catalog authoring tree (embed in MS-007)
docs/         # implementation authority copies
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
