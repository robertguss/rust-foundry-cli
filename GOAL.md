# Goal prompt — implement all beads

Copy the block below into a Grok session with goal mode enabled
(see `/goal` in the user guide). Paste as a single message.

**Assumptions (locked by clarifying Qs, 2026-08-01):**

| Decision | Choice |
| -------- | ------ |
| Done when | **0 open issues** (tasks, milestones, phase epics, product epic) |
| Git | **Commit on `main` + `git push origin HEAD` after every closed bead** |
| Quality gate | **Full local CI** before close: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (+ bead AC) |
| Optional/Should | **Implement fully when possible**; residual close only if plan explicitly allows Should residual |
| On stuck/fail | **Pause and report** after 2 serious fix attempts |
| Beads state | **Include `.beads/` / bd store changes in the same commit** as the code |

---

## Copy-paste `/goal` objective

```text
/goal Deliver the entire open bead backlog in this repository (rust-foundry-cli) until there are zero open issues.

## Definition of done
Goal is complete only when `bd count` (or equivalent) shows **0 open** issues of every type: tasks, milestones, phase epics, and the product epic. Closed standing-policy beads may stay closed. Do not invent new REQs or demote locks.

## Authority (do not override)
1. docs/AUTHORITY.md pins
2. docs/02-definitive-specification-revised.md (product law)
3. docs/02-implementation-plan-revised.md (delivery sequence / DAG)
4. Product AGENTS.md (workflow; never overrides REQs)
5. The bead itself: title, description, acceptance_criteria, design, comments

Product locks (never undo): macOS+Linux only / no Windows; exclusive place / refuse non-empty; no merge-default generate; closed catalog; catalog offline ≠ verify offline; AGENTS.md + .agents/skills only (no Claude Core / MCP default); plan package pure (plan must not import fsx/generate/cli); plan JSON field names unstable until MS-004 closed.

## Authorization for this goal
You are pre-authorized to: (1) commit on `main`, (2) `git push origin HEAD` after each successful bead, (3) close beads with `bd close`, (4) stage and commit `.beads/` / beads DB export changes together with product code. Do not open PRs unless a bead explicitly requires it. Do not force-push. Do not amend commits already pushed.

## Outer loop (one bead per iteration)
Repeat until 0 open issues:

### 1. Select work
- Run `bd ready` (prefer `bd ready -t task` first, then milestones, then epics).
- Pick the single highest-priority ready issue (lowest P number; break ties by critical path / MS number).
- Prefer leaf **tasks** over milestones/epics when both are ready.
- If a **milestone or epic** is ready only because children are done: do not invent implementation work — verify children closed + any exit checklist in the bead, then go to Close path.
- If **no ready work** but open issues remain (deadlock / all blocked): **pause the goal** and report blocker graph (`bd ready --explain`, sample blocked issues). Do not break the DAG by starting blocked work.
- Claim the issue: `bd update <id> --claim` (or project-equivalent claim).

### 2. Understand the bead
- `bd show <id>` fully (description, acceptance_criteria, design, deps, comments).
- Read only the authority sections needed for this bead (do not re-read entire specs every loop).
- List concrete acceptance checks you will prove before close.

### 3. Implement (tasks only)
- Implement the minimum change that satisfies acceptance_criteria and relevant REQs.
- Optional/Should beads (distribution, sample-spec, REQ-152, etc.): **implement fully when possible**. Residual close only when the plan explicitly allows a Should residual — write residual under docs/evidence/ and put the path in the close reason.
- Exit-checklist beads (e.g. MS-006.3, MS-011.2, phase exits): produce evidence under docs/evidence/ as the bead requires; do not rubber-stamp.

### 4. Verify (mandatory before commit)
Run full local CI from repo root:
  cargo fmt --check
  cargo clippy --all-targets -- -D warnings
  cargo test
Plus any bead-specific checks (fixtures, purity, goldens, offline smoke, etc.).
All must pass. If not: fix and re-run (max **2** serious fix attempts after the first failure). If still red → **pause goal**, leave bead open (unclaim if needed), report bead id, failing command output summary, and hypothesis. Do not close; do not push a red tree.

### 5. Close the bead
- `bd close <id> --reason "<one-line: what shipped + evidence path if any>"`
- If closing a milestone/epic: confirm all children closed first; close reason cites exit evidence.

### 6. Commit + push (same commit for code + beads)
- `git status` / `git diff` — stage product changes **and** beads state (`.beads/`, issues export, etc.).
- Do not commit secrets, target/, or unrelated junk.
- Commit message format:
  `<bead-id>: <short imperative summary>`
  Body: 1–3 sentences on what/why; mention MS-### and key REQs if applicable; note tests run.
- `git push origin HEAD` (main). On push failure: pause and report (do not force-push).
- One bead → one commit → one push. Then start the next iteration.

## Milestone / epic closing order
When implementation children of a milestone are all closed, select that milestone when ready, verify acceptance/exit criteria, close it, commit (may be beads-only or evidence-only), push. Same for phase epics, then product epic last.

## Progress reporting
After each successful push, briefly state: closed id/title, commit sha, open count remaining, next ready candidate. Do not mark the **goal** complete until open count is 0 and an independent check (`bd list` / `bd count`) confirms it.

## Out of scope
- Inventing REQs, expanding profiles before MS-021, freezing plan JSON before MS-004, Windows support, merge-default generate, relaxing path jail / emptiness.
- Parallel multi-bead implementation in one iteration.
- Silent skip of failing beads.
```

---

## How to run

1. Ensure goal mode is enabled for the session.
2. On a clean `main` with network for push.
3. Paste the fenced objective (including the leading `/goal`).
4. Monitor with `/goal status` if available; `/goal pause` to stop.

## After the goal completes

- Confirm `bd list -s open` is empty.
- Confirm `git status` clean and `origin/main` matches local.
- Optionally tag a release only when MS-020 ship decision says so (do not auto-tag in this goal unless a bead requires it).
