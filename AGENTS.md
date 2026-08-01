# Agent rules — rust-foundry (product)

This is the **product** repository. Research methodology and stage machinery
live in the sibling research repo (`rust-foundry`), not here.

## Authority (read before coding)

1. [`docs/02-definitive-specification-revised.md`](docs/02-definitive-specification-revised.md) — **product law**
2. [`docs/02-implementation-plan-revised.md`](docs/02-implementation-plan-revised.md) — **delivery sequence**
3. [`docs/AUTHORITY.md`](docs/AUTHORITY.md) — pinned source commits
4. This file — product workflow only

Do not invent REQs, demote locks, or treat chat history as authority.

## Current delivery position

- **Phase:** PHASE-01 — Foundry CLI, Construct, write safety
- **Completed:** MS-001 (product repository bootstrap)
- **Next:** MS-002 — Spec parse and `validate` command
- **Strict DAG after MS-002:** MS-003 → MS-004 (SPK-100) → MS-005 → MS-006 (SPK-101)
- **Not yet:** embedded catalog (MS-007), Core emit (MS-008), MS-021 dogfood,
  profiles/TUI (MS-009/MS-010)

## Package layout (revised-spec §10.1)

```text
src/
  main.rs      # binary process boundary
  lib.rs       # crate root
  cli/         # clap wiring only
  spec/        # parse + validate (pure)
  catalog/     # load manifests, digests
  resolve/     # archetype/profile resolution (pure)
  plan/        # Construct plan (pure)
  report/      # text/JSON encoding
  render/      # templates → bytes
  fsx/         # stage + exclusive place (MS-005+)
  generate/    # lifecycle orchestration (MS-005+)
  verify/      # tiered runners (stub PHASE-01; full PHASE-03)
catalog/       # closed catalog authoring tree (embed later)
```

**Purity rule:** `plan` MUST NOT import `fsx`, `generate`, or `cli`.

**Ordering rule:** No plan JSON field names stable until MS-004 (SPK-100).

## Product locks (never silently undo)

- macOS + Linux only; **no Windows**
- Exclusive place; refuse non-empty default; no merge-default generate
- Closed catalog; offline catalog ≠ offline verify
- AGENTS.md + `.agents/skills/` only for Generated Projects; no Claude Core; MCP none
- Plan-as-contract (same Construct for validate/plan/generate)
- Do not expand profiles/TUI until **MS-021** green
- Do not ship on bare “CI green” without REQ-150/151 checklist (PHASE-05)

## Foundry vs Generated surfaces

Research-program skills and research `AGENTS.md` rules must **not** ship into
Generated Project emit. Keep product-agent docs and Generated Core agent surface
separate.

## Commands

```bash
cargo build
cargo test
cargo fmt --check
cargo clippy -- -D warnings
cargo run -- version
just ci   # when justfile gates are used
```

## Definition of done (local)

- Tests green; no purity violations
- Phase/milestone exit criteria from the revised plan before claiming done
- No secret material in Project Specs or goldens
- No product code in the research repo (L12 / REQ-010)
