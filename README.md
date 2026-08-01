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
| **Current milestone** | **MS-001 complete** → next **MS-002** |

## Specification authority

Do **not** invent product behavior outside these documents:

| Role | Doc |
| ---- | --- |
| Product law | [`docs/02-definitive-specification-revised.md`](docs/02-definitive-specification-revised.md) |
| Delivery sequence | [`docs/02-implementation-plan-revised.md`](docs/02-implementation-plan-revised.md) |
| Pins / provenance | [`docs/AUTHORITY.md`](docs/AUTHORITY.md) |

Agent rules for this repo: [`AGENTS.md`](AGENTS.md).

## Status

MS-001 scaffold is in place:

- Single-crate module map (§10.1 stubs)
- Linux-only GitHub Actions CI
- `foundry version` command
- Authority copies of revised spec, revised plan, and Blueprint

PHASE-01 next milestones:

| MS | Outcome |
| -- | ------- |
| MS-002 | TOML schema 1 parse; denylist; `validate` |
| MS-003 | Pure Construct `plan` (write-free) |
| MS-004 | SPK-100 golden plan freeze |
| MS-005 | Stage / place / `generate` lifecycle |
| MS-006 | SPK-101 emptiness/place/jail matrix |

## Quickstart

```bash
# From this repo root
cargo test
cargo run -- version

# Intended dry-run workflow (once MS-002/MS-003 land):
# cargo run -- validate --spec ./examples/minimal-cli.toml
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
