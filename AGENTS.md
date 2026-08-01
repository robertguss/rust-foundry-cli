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
- **Completed:** MS-001 (bootstrap), MS-002 (`validate` + schema 1 + denylist)
- **Next:** MS-003 — Pure Construct `plan` command
- **Strict DAG:** MS-003 → MS-004 (SPK-100) → MS-005 → MS-006 (SPK-101)
- **Not yet:** embedded catalog (MS-007), Core emit (MS-008), MS-021 dogfood,
  profiles/TUI (MS-009/MS-010)

## Secret field denylist (REQ-033)

Case-insensitive **field names** (any nesting): `password`, `secret`, `token`,
`api_key`, `private_key`, `access_key`, `client_secret`.

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

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:970c3bf2 -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.

## Agent Context Profiles

The managed Beads block is task-tracking guidance, not permission to override repository, user, or orchestrator instructions.

- **Conservative (default)**: Use `bd` for task tracking. Do not run git commits, git pushes, or Dolt remote sync unless explicitly asked. At handoff, report changed files, validation, and suggested next commands.
- **Minimal**: Keep tool instruction files as pointers to `bd prime`; use the same conservative git policy unless active instructions say otherwise.
- **Team-maintainer**: Only when the repository explicitly opts in, agents may close beads, run quality gates, commit, and push as part of session close. A current "do not commit" or "do not push" instruction still wins.

## Session Completion

This protocol applies when ending a Beads implementation workflow. It is subordinate to explicit user, repository, and orchestrator instructions.

1. **File issues for remaining work** - Create beads for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **Handle git/sync by active profile**:
   ```bash
   # Conservative/minimal/default: report status and proposed commands; wait for approval.
   git status

   # Team-maintainer opt-in only, unless current instructions forbid it:
   git pull --rebase
   bd dolt push
   git push
   git status
   ```
5. **Hand off** - Summarize changes, validation, issue status, and any blocked sync/commit/push step

**Critical rules:**
- Explicit user or orchestrator instructions override this Beads block.
- Do not commit or push without clear authority from the active profile or the current user request.
- If a required sync or push is blocked, stop and report the exact command and error.
<!-- END BEADS INTEGRATION -->

<!-- BEGIN BEADS CODEX SETUP: generated by bd setup codex -->
## Beads Issue Tracker

Use Beads (`bd`) for durable task tracking in repositories that include it. Use the `beads` skill at `.agents/skills/beads/SKILL.md` (project install) or `~/.agents/skills/beads/SKILL.md` (global install) for Beads workflow guidance, then use the `bd` CLI for issue operations.

### Quick Reference

```bash
bd ready                # Find available work
bd show <id>            # View issue details
bd update <id> --claim  # Claim work
bd close <id>           # Complete work
bd prime                # Refresh Beads context
```

### Rules

- Use `bd` for all task tracking; do not create markdown TODO lists.
- Run `bd prime` when Beads context is missing or stale. Codex 0.129.0+ can load Beads context automatically through native hooks; use `/hooks` to inspect or toggle them.
- Keep persistent project memory in Beads via `bd remember`; do not create ad hoc memory files.

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.
<!-- END BEADS CODEX SETUP -->
