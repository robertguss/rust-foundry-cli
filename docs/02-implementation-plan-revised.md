# Final Revised Implementation Plan — rust-foundry

- **Artifact type:** Final revised implementation plan
- **Program:** rust-foundry
- **Status:** Accepted — delivery authority
- **Version:** 0.2
- **Plan date (baseline):** 2026-08-01
- **Revision date:** 2026-08-01
- **Delivery status:** Accepted — delivery authority (human-accepted stage `plan-revision`; subordinate to revised definitive specification for _what_ to build)
- **Implementation authority:** [`docs/specifications/02-definitive-specification-revised.md`](../specifications/02-definitive-specification-revised.md) (Accepted — implementation authority; `bf8b0f4d`)
- **Baseline plan:** [`docs/plans/01-implementation-plan.md`](01-implementation-plan.md) (v0.1; Proposed — pending plan review)
- **Plan review:** [`docs/reviews/02-implementation-plan-adversarial-review.md`](../reviews/02-implementation-plan-adversarial-review.md) (FND-200..203; gate Conditional; `23fe4223`)
- **Commissioning prompt:** [`docs/prompts/09-final-plan-revision-prompt.md`](../prompts/09-final-plan-revision-prompt.md)
- **Depends on:** Stage `plan-review` accepted
- **Identifier ranges:** PHASE-01..05 · MS-001..MS-021 · SPK-100..104 (scheduled)

> This plan defines **how to build** the Foundry product. It is subordinate to
> the revised definitive specification for _what_ to build. It dispositions every
> plan-review finding. **Status: Accepted — delivery authority** (stage
> `plan-revision` human-accepted). It is **not** a granular coding backlog.
> Product code lives in a **separate Foundry product repository** (L12 / REQ-010).

---

## 0A. Revision Summary

This revision integrates the Conditional-gate plan adversarial review
(FND-200..FND-203) into a standalone delivery plan without reopening Blueprint
locks, re-picking Core crates, inventing product features, or producing a coding
backlog.

**Primary corrections:**

1. **PHASE-01 milestone DAG** is a single consistent order: MS-003 → MS-004
   (SPK-100 golden freeze) → MS-005 (stage/place/generate) → MS-006 (SPK-101)
   (FND-200).
2. **PHASE-05 / MS-019** bind acceptance evidence to the REQ-150/151 scenario
   checklist (not bare “CI green”) (FND-202).
3. **PHASE-04 // PHASE-03 join** hardens command-surface / DoD consistency:
   parallel draft allowed; PHASE-04 exit and MS-018 require REQ-088 match against
   PHASE-03 fixtures (FND-201).
4. **Pure-CLI Core dogfood gate** (MS-021) sits after MS-008 and before profile/TUI
   expansion (MS-009/MS-010) (FND-203).

**Architecture and methodology enhancements added in this working revision:**

5. Snapshot golden testing (`insta`), property-based fuzz fixtures, plan-package
   purity scans, and stable error-code contracts are codified in PHASE-01 testing.
6. `render` emits a pure path→bytes map; `fsx` places atomically; parallelism is
   permitted but not required.
7. Verify runner uses a sanitized environment subset and timeout.
8. Template-SoT dry-run digest check is introduced at MS-008 and automated at
   MS-015.
9. `foundry sample-spec` is proposed as a v1 convenience command (requires DEC
   or spec amendment before it becomes binding).
10. MS-001 self-dogfood quality gates and profile `requires` dependency edges are
    explicit.

**Status judgment:** High finding FND-200 is **Accepted** and integrated. Medium
findings FND-201..203 are **Accepted** and integrated. No Critical findings
existed. No new major product machinery. Eligible for **delivery authority** after
human validation and stage acceptance.

## 0B. Finding Disposition Ledger

| FND     | Sev    | Disposition  | Integration summary                                                                                                                                                                                                                                                     |
| ------- | ------ | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| FND-200 | High   | **Accepted** | Single PHASE-01 DAG: MS-003 → MS-004 → MS-005 → MS-006. MS-005 Prerequisites = MS-004. MS-003 Blocks = MS-004 only. §6 critical path redrawn; PHASE-01 rule: no plan JSON field names stable until MS-004.                                                              |
| FND-201 | Medium | **Accepted** | PHASE-04 may draft after PHASE-02 emit paths; **PHASE-04 exit** requires REQ-088 string-match tests green against MS-012 justfile/cargo fallbacks + MS-013 GHA fixtures. MS-018 Prerequisites include MS-012 + MS-013. Parallelism note + §10 integration row hardened. |
| FND-202 | Medium | **Accepted** | PHASE-05 Exit + MS-019 Acceptance evidence expanded to REQ-150/151 scenario checklist. Unit SPK-101/102 do not substitute for E2E acceptance jobs. §12 PHASE-05 row updated.                                                                                            |
| FND-203 | Medium | **Accepted** | New **MS-021** pure-CLI catalog generate dogfood gate after MS-008; MS-009/MS-010 Prerequisites require MS-021. §15 dogfooding aligned. Full default verify still PHASE-03.                                                                                             |

## 0C. Integrated Correction Ledger

| ID      | Where integrated                                                                                                 |
| ------- | ---------------------------------------------------------------------------------------------------------------- |
| FND-200 | §6 dependency graph; PHASE-01 rule + exit; MS-003/004/005/006 Prerequisites/Blocks                               |
| FND-201 | §6–§7 parallelism; PHASE-04 Depends/Entry/Exit; MS-016 draft note; MS-018 Prerequisites; §10 integration table   |
| FND-202 | PHASE-05 Exit; MS-019 Acceptance evidence; §12 testing strategy                                                  |
| FND-203 | §6 milestone path; PHASE-02 Entry/Dogfooding/milestones; MS-008 Blocks; MS-021; MS-009/MS-010 Prerequisites; §15 |

## 0D. Preserved Strengths

- Write safety (REQ-050/051/053 + SPK-101) before catalog breadth
- Plan-as-contract (REQ-034/040 + SPK-100) early in PHASE-01
- Catalog offline ≠ verify offline honesty (REQ-060 vs REQ-120 + SPK-103)
- SPK-102 before PHASE-02 exit; SPK-104 for template SoT
- Phase exits do not depend on later phases
- 68/68 REQ-to-phase coverage; REC ledger 49/49 intact (no silent drop)
- No coding backlog; L12 product repo explicit; locks/non-goals intact
- Should residuals for distribution (REQ-068) and agent operability (REQ-152) honest

---

## 1. Artifact Metadata

| Field                 | Value                                                                           |
| --------------------- | ------------------------------------------------------------------------------- |
| Program               | rust-foundry                                                                    |
| Stage                 | `plan-revision`                                                                 |
| Plan path             | `docs/plans/02-implementation-plan-revised.md`                                  |
| Spec authority        | `docs/specifications/02-definitive-specification-revised.md` v0.2               |
| Blueprint             | `docs/00-program-blueprint.md` (Accepted)                                       |
| Charter               | `docs/01-research-charter.md` (Accepted)                                        |
| Product code location | **Separate Foundry product repository** (not this research repo; L12 / REQ-010) |
| Research repo role    | Specs, plans, methodology; optional bounded spikes only                         |

---

## 2. Implementation Authority

Apply this precedence when interpreting the plan (highest first):

1. Accepted `DEC-###` (none at revision time)
2. Blueprint locks and non-goals
3. Charter methodology
4. Commissioning prompt for this stage
5. **Revised definitive specification** — what to build (REQs, locks, invariants)
6. This plan — delivery sequencing (**delivery authority only after human accept**)
7. Focused research reports — evidence only (REC dispositions live in spec §28)
8. Chat history / preference — never load-bearing

**Hard rules:**

- Do not override normative REQs with sequencing convenience.
- Do not reopen REC-001..215 without DEC or new evidence.
- Do not implement product crates in the research repository (REQ-010 / L12).
- Baseline plan `01-…` is historical; execute against this revised plan after accept.
- Baseline specification `01-…` is historical only; plan against revised spec `02-…`.

---

## 3. Objectives

1. Deliver a **safe, phased sequence** to build the hybrid Foundry product:
   generator CLI + strong Core + GitHub template surface (L1 / REQ-001).
2. Establish thin **end-to-end** `validate → plan → generate` early, with
   fail-closed write semantics (REQ-050/051/053) before catalog breadth.
3. Complete the **closed embedded catalog** and emit matrix with TUI
   zero-leakage (REQ-004/062..067/065), after pure-CLI Core dogfood (MS-021).
4. Wire **verify tiers**, Core GHA, optional distribution, and catalog-SoT
   GitHub template regen (REQ-068/087/120..122).
5. Ship **portable AI-native surfaces** for Generated Projects and Foundry
   product skills (REQ-006..008, REQ-100..108, REQ-123) with hard command-surface
   consistency (REQ-088/105).
6. Gate ship readiness on **acceptance scenarios** (REQ-150..152) with an
   executable scenario checklist, and recommended spikes SPK-100..104.
7. Keep the plan at **phase/milestone** granularity so implementers (agents +
   owner) can derive work without this research program becoming a ticket system.

---

## 4. Non-Goals

1. Granular coding backlog, sprint tickets, or agent task packets as plan content.
2. Product implementation inside the research repo beyond optional evidence spikes.
3. Re-picking Core stack (clap, ratatui, edition 2024, etc.) without DEC / new evidence.
4. Windows hosts/targets/installers (L3 / REQ-003).
5. Remote/plugin catalogs, marketplace, merge-into-non-empty default generate.
6. Claude-specific Core (`CLAUDE.md` / `.claude/`), default MCP kitchen sink.
7. First-class lib-only or multi-crate workspace generate (v1).
8. `update` / `recopy` / saved plan-file apply without rebuild (deferred / rejected in spec).
9. Claiming delivery authority before human acceptance of this revised plan stage.
10. Weakening Must REQs or locks to make sequencing easier.

---

## 5. Assumptions

1. A **new Foundry product git repository** will hold product code; this research
   repo remains methodology + design authority artifacts.
2. Implementers are primarily **AI coding agents** guided by product AGENTS/skills
   (L9); the owner accepts gates. **Milestone Prerequisites/Blocks are normative**
   for agent ordering.
3. Host development and CI targets are **Linux Required**, **macOS Recommended**
   when shipping mac bins or `distribution` (REQ-003, REQ-087); never Windows.
4. Provisional defaults in revised spec remain defaults unless a DEC changes them:
   rust-version floor `1.85`, default CI `cargo test`, tracing on full CLI,
   pre-commit Default for hooks profile, fail-closed cross-device place.
5. SPK-100..104 are **recommended gates**, not already executed; plan schedules them.
6. Exact plan JSON field names may refine under SPK-100 without changing element
   set (OQ-200 / REQ-041); **no plan JSON field names treated as stable until MS-004**.
7. Embed crate choice (`include_dir` vs rust-embed) is non-load-bearing (OQ-201).
8. Foundry skill **ids** are frozen (REQ-104); skill **bodies** land in PHASE-04.
9. Greenfield v1: no migration of foreign scaffolds (spec §18).
10. Standard rigor: second plan-review is **not** automatic unless this revision
    introduces major new machinery or leaves High blockers (it does not).

---

## 6. Dependency Graph

```text
PHASE-01  Foundry CLI + pure Construct + write safety
    │  (SPK-100 before generate shapes freeze; SPK-101 before exit)
    ▼
PHASE-02  Embedded catalog + emit matrix + Core/TUI/hooks/secrets
    │  (MS-021 pure-CLI dogfood before profiles/TUI; SPK-102 before exit)
    ├──────────────────────────────┐
    ▼                              ▼
PHASE-03  Verify + GHA + dist      PHASE-04  AI-native surfaces
    │  (SPK-103/104)                  │  (draft // PHASE-03 OK)
    │                              │  exit requires REQ-088 match
    └──────────┬───────────────────┘
               ▼
PHASE-05  Acceptance suite + freeze gates + release readiness
```

**Milestone critical path (normative — single DAG):**

```text
MS-001 → MS-002 → MS-003 → MS-004 → MS-005 → MS-006
         → MS-007 → MS-008 → MS-021 → MS-009 → MS-010 → MS-011
         → MS-012 → MS-013 → MS-014 → MS-015          (PHASE-03)
         → MS-016 → MS-017 → MS-018                   (PHASE-04; see join rules)
         → MS-019 → MS-020                            (PHASE-05)
```

**PHASE-04 join rules (normative):**

- MS-016 may **start** after MS-011 (draft AI content in parallel with PHASE-03).
- MS-017 requires MS-016 + MS-010.
- MS-018 requires MS-016 + **MS-012 + MS-013** (command-surface freeze fixtures).
- **PHASE-04 exit** cannot complete until REQ-088 string-match tests are green
  against MS-012 justfile/cargo fallbacks and MS-013 workflow fixtures.
- PHASE-05 requires PHASE-03 exit **and** PHASE-04 exit.

No phase exit depends on a later phase. Verify (PHASE-03) may use a **minimal**
catalog from PHASE-02; AI surfaces (PHASE-04) must not be required for pure-CLI
generate correctness from PHASE-02/03, but ship readiness requires PHASE-04+05.

---

## 7. Phase Overview

| Phase    | Name                                  | Depends on                                              | User-visible outcome                                                                                                    |
| -------- | ------------------------------------- | ------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| PHASE-01 | Foundry CLI, Construct, write safety  | None (product repo bootstrap)                           | Non-interactive `validate` / `plan` / `generate` with fail-closed stage→place on a minimal stub catalog or fixture tree |
| PHASE-02 | Embedded catalog and emit matrix      | PHASE-01                                                | Closed catalog units; pure CLI Core emit **dogfooded**; TUI/hooks/secrets profiles; permutation-stable plans            |
| PHASE-03 | Verify, CI, distribution, GH template | PHASE-02                                                | Default/strict/none verify; Core GHA workflows; optional cargo-dist profile; catalog-SoT template regen                 |
| PHASE-04 | AI-native surfaces and product skills | PHASE-02 (parallel draft with PHASE-03; hard exit join) | Portable AGENTS/skills on Generated Projects; Foundry product skills; REQ-088-consistent DoD; no Claude/MCP defaults    |
| PHASE-05 | Acceptance and freeze                 | PHASE-01..04 complete for v1 scope                      | Automated REQ-150/151 scenario checklist green; release/digest gates documented                                         |

**Parallelism note:** PHASE-04 template content may **draft** once PHASE-02 pure-CLI
and TUI emit paths exist (after MS-011); it must not block PHASE-03 verify/CI work.
PHASE-04 **exit** and MS-018 wait for command-surface freeze fixtures from PHASE-03
(MS-012 + MS-013). PHASE-05 requires both PHASE-03 and PHASE-04 exits.

---

## 8. Phases

## PHASE-01 — Foundry CLI, Construct, and Write Safety

- **Status:** Planned
- **Objective:** Stand up the Rust Foundry product crate with lifecycle commands,
  pure Construct shared by validate/plan/generate, and fail-closed write path
  (stage → verify-hook point → exclusive place).
- **User-visible outcome:** An owner/agent can run non-interactive
  `foundry validate|plan|generate` against a TOML Project Spec with CLI
  overrides; destination is never partially written; non-empty destinations refuse.
- **Depends on:** None (product repository bootstrap)
- **Requirements:** REQ-001, REQ-002, REQ-003, REQ-020, REQ-021, REQ-022, REQ-023,
  REQ-024, REQ-030, REQ-031, REQ-032, REQ-033, REQ-034, REQ-040, REQ-041, REQ-042,
  REQ-043, REQ-050, REQ-051, REQ-052, REQ-053, REQ-130
- **Milestones:** MS-001, MS-002, MS-003, MS-004, MS-005, MS-006
- **Primary risks:** RSK-100, RSK-101, RSK-109, RSK-110, RSK-112

### Entry Criteria

- Revised specification accepted as implementation authority.
- Product repository created (or designated) **outside** the research program repo.
- Toolchain policy chosen for dogfood (OQ-104 residual acceptable as provisional).

### Ordering rule (normative)

**No plan JSON field names are treated as stable until MS-004 (SPK-100) completes.**
Generate lifecycle (MS-005) must not freeze public plan JSON shape before MS-004.

### Scope

- Hybrid product intent encoded in docs and module map (REQ-001, REQ-130).
- Rust CLI with clap derive; commands: `validate`, `plan`, `generate`,
  `catalog list`, `catalog show`, `version` (catalog commands may stub until
  PHASE-02 embed) (REQ-020, REQ-024).
- Non-interactive first; exit codes frozen (REQ-021, REQ-023).
- `validate` / `plan` write-free w.r.t. destination (REQ-022, REQ-043).
- TOML Project Spec schema 1; explicit `--spec`; archetype `cli` only;
  secret field denylist; unknown keys hard-fail (REQ-030..033).
- CLI overrides `--name` / `--dest` / `--verify` win over TOML; applied before
  Construct; equality on effective inputs (REQ-034, REQ-040).
- Plan elements and formats per REQ-041..042; SPK-100 freezes JSON names
  **before** MS-005 generate invents fixture shapes.
- Stage-first lifecycle: success cleans stage; fail retains stage + path in
  error (REQ-050).
- Emptiness predicate and place algorithm (REQ-051); no merge/update (REQ-052).
- Stage-root path jail with escape fixtures (REQ-053).
- Host OS policy macOS+Linux only in product CI posture from day one (REQ-003).

### Explicit Non-Goals

- Full embedded Core template matrix (PHASE-02).
- Default verify runners against Generated Project gates (PHASE-03); PHASE-01
  may stub verify as no-op or fixture-only while preserving lifecycle hooks.
- AI-native emit content (PHASE-04).
- Acceptance suite completeness (PHASE-05).

### Architecture and Components

Default single-crate module map (REQ-130 / spec §10.1): `cli`, `spec`, `catalog`
(stub OK), `resolve`, `plan`, `render` (minimal), `fsx`, `generate`, `verify`
(stub OK), `report`. Domain logic not only in `cli`.

### Integrations

None external required beyond local filesystem and eventual cargo for dogfood.

### Data or Migration Work

None (greenfield). Spec schema = 1 only.

### Evidence Spikes

| SPK     | When                                                           | Exit evidence                                                                                                                                                                                                                                                                                        |
| ------- | -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| SPK-100 | Before MS-005 treats plan JSON as stable; before PHASE-01 exit | Snapshot (`insta`) golden plan fixtures for minimal CLI (and slot for TUI later); element set matches REQ-041; JSON redaction for catalog digest; OQ-200 residual closed or documented                                                                                                               |
| SPK-101 | Before PHASE-01 exit (after MS-005 lifecycle exists)           | Table-driven emptiness/place/lifecycle cases green using isolated `tempfile` sandboxes; plus property-based fixtures for path traversal, denylist case/unicode variants, and profile permutations (missing, empty, non-empty incl. `.git`, file-at-path, symlink, success clean, fail retain + path) |

### Testing and Verification

- Unit/integration: parser denylist, override equality, plan purity, path jail.
- FS sandbox: validate/plan no dest writes; generate place matrix (SPK-101) using isolated `tempfile` sandboxes.
- Snapshot golden fixtures (`insta`) for plan JSON/text output; drift fails CI; `cargo insta review` is the accepted update path.
- Error-code matrix: each failure mode produces a stable code and nonzero exit code; JSON error shape is part of SPK-100 fixtures.
- Purity scan: automated test that `src/plan/**/*.rs` contains no imports of `crate::fsx`, `crate::generate`, `crate::cli`, `foundry::fsx`, `foundry::generate`, or `foundry::cli`.
- CLI help and exit codes.

### Security and Reliability

- Path jail (REQ-053 / RSK-109).
- Secrets forbidden in spec (REQ-033 / RSK-110).
- Fail-closed writes (REQ-050/051 / RSK-100, RSK-112).

### Dogfooding or Operational Validation

- Owner/agent runs validate → plan → generate on a fixture dest in temp dirs.
- Dogfood product quality gates begin lightly (fmt/clippy/test) without blocking
  on Generated Project verify tiers.

### Rollback and Reconsideration Triggers

- If exclusive place cannot be made correct on Linux+macOS without merge: **stop**;
  do not ship merge-default (Blueprint non-goal). Cross-device copy+swap requires DEC.
- If Construct cannot be shared by validate/plan/generate: **reconsider** before
  catalog expansion (INV-3 / RSK-101).

### Exit Criteria

Observable evidence:

1. Product crate builds on Linux CI; no Windows jobs.
2. `validate`, `plan`, `generate` exist; non-interactive; documented exit codes.
3. Same effective inputs → equal planned file sets (REQ-040/034); cross-command `plan_sha256` equality asserted for representative fixtures.
4. SPK-101 matrix green; SPK-100 golden plan for minimal path exists (**MS-004 before MS-005 shapes**).
5. Path jail fixtures hard-fail escapes.
6. Plan-package purity scan green: `src/plan/` imports no `fsx`, `generate`, or `cli` modules.
7. Stable error codes documented for denylist, schema, archetype, path-jail, non-empty destination, file-at-path, symlink, and place failure; JSON error shape included in SPK-100 fixtures.
8. Destination never partially placed on failure; stage path printed on fail.
9. Module map inspectable (REQ-130 Should satisfied or residual noted).

---

## PHASE-02 — Embedded Catalog and Emit Matrix

- **Status:** Planned
- **Objective:** Embed a closed catalog; implement composition order and Core
  emit; **prove pure-CLI Core offline generate** before profile/TUI expansion;
  add TUI/hooks/secrets profile units with zero pure-CLI TUI leakage.
- **User-visible outcome:** `catalog list|show` and `version` report catalog
  digest; generate produces single-crate pure CLI Core trees; optional profiles
  compose with canonical order independent of input order.
- **Depends on:** PHASE-01 exit
- **Requirements:** REQ-004, REQ-005, REQ-009, REQ-025, REQ-060, REQ-061, REQ-062,
  REQ-063, REQ-064, REQ-065, REQ-066, REQ-067, REQ-069, REQ-070, REQ-080, REQ-081,
  REQ-082, REQ-083, REQ-084, REQ-085, REQ-086, REQ-089
- **Milestones:** MS-007, MS-008, MS-021, MS-009, MS-010, MS-011
- **Primary risks:** RSK-003/103, RSK-004, RSK-005, RSK-102, RSK-104, RSK-105

### Entry Criteria

- PHASE-01 exit criteria met.
- SPK-100 minimal golden plans available to extend.

### Scope

- Closed embedded catalog; offline parse/resolve/construct/render (REQ-060).
- Custom planner-led engine (not cargo-generate/Copier as product engine) (REQ-061).
- Catalog units: `core`, `cli`, `tui`, `hooks`, `secrets`, `distribution` (unit
  may be stub until PHASE-03 for distribution) (REQ-062).
- Canonical profile fold order: `tui` → `hooks` → `secrets` → `distribution`;
  input order non-significant (REQ-063).
- Catalog units declare `requires` edges; `resolve` topologically sorts or errors
  on unsupported combinations; canonical order is the topological default.
- Core always emits; single-crate layout; CLI primary (REQ-004, REQ-009, REQ-064).
- **MS-021 gate:** offline pure-CLI catalog generate + golden match + owner/agent
  `cargo test` smoke **before** MS-009/MS-010.
- `foundry sample-spec` command emits a canonical, commented Project Spec TOML
  (no secret placeholders; optional `--profile tui` to include profile examples).
  _Authority note: this is a new v1 convenience surface; finalize via DEC or spec
  amendment before treating as binding REQ-024 extension._
- TUI generate-time inclusion only under `tui` profile (REQ-065).
- hooks / secrets profiles (Should) (REQ-066, REQ-067).
- Core pins: toolchain file, edition 2024, rustfmt/clippy, cargo test Required,
  clap derive, rust-version floor policy, tracing default full CLI, just Default,
  licensing posture (REQ-069..086, REQ-089 as applicable).
- `foundry version` reports catalog digest (REQ-025).
- Closed curated sets for tools/profiles/skills inventory (REQ-005).

### Explicit Non-Goals

- Default verify execution cost freeze (PHASE-03 / SPK-103).
- GHA workflow polish and GH template CI regen job (PHASE-03).
- Full AGENTS/skills prose (PHASE-04); emit may include path stubs if needed for
  matrix tests, but portable content contracts land in PHASE-04.
- distribution profile completeness (PHASE-03) — unit may exist empty.

### Architecture and Components

- `catalog` loads embed; digests stable across builds for same content.
- `resolve` + `plan` apply REQ-063 order.
- `render` writes planned files only under stage root (still REQ-053).

### Integrations

- Embed mechanism selection (OQ-201) recorded in product docs; non-blocking.

### Data or Migration Work

- Catalog content versioned with product release; digest bound to binary.

### Evidence Spikes

| SPK     | When                 | Exit evidence                                                                                                                           |
| ------- | -------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| SPK-102 | Before PHASE-02 exit | Pure CLI trees have zero TUI deps/files/skills; TUI profile includes ratatui path set; permutation digests equal for reordered profiles |

### Testing and Verification

- Emit matrix fixtures; forbidden-path tests for pure CLI.
- Profile permutation invariance.
- Offline generate without network for catalog/render path (REQ-060).
- Golden plans extended for TUI profile (SPK-100 continuation).
- MS-021 smoke recorded before profile/TUI matrix expansion.

### Security and Reliability

- No remote catalog fetch (INV-1 / REQ-060).
- Secrets profile scaffolding hygiene only (RSK-005); no secrets in templates.

### Dogfooding or Operational Validation

- **Required (MS-021):** Generate pure CLI from embedded catalog offline; open
  project; `cargo test` smoke (full default verify gate remains PHASE-03).
- Only after MS-021: expand to profiles and TUI fixtures.

### Rollback and Reconsideration Triggers

- Profile explosion pressure: refuse new profiles without DEC (RSK-104).
- If TUI cannot be generate-time inclusion without pure-CLI leakage: stop and
  fix before PHASE-03 (RSK-103).
- Engine complexity: prefer single crate Default; split only with DEC (RSK-105).
- If MS-021 fails: **do not** start MS-009/MS-010; fix Core emit first.

### Exit Criteria

1. Catalog list/show + version digest work offline.
2. Pure CLI Core generate produces single-crate tree with locked Core files.
3. **MS-021 dogfood evidence recorded** (offline generate + cargo test smoke).
4. SPK-102 green; REQ-063 permutation tests green.
5. hooks and secrets profiles emit when selected (or residuals explicitly Should-deferred with owner approval — default is emit).
6. No Windows / remote catalog / merge paths introduced.

---

## PHASE-03 — Verify Tiers, GHA, Distribution, GH Template

- **Status:** Planned
- **Objective:** Run tiered verification on staged trees before place; emit Core
  GitHub Actions; optional distribution profile; GitHub template snapshot is
  catalog SoT with regen discipline. Produce **command-surface freeze fixtures**
  consumed by PHASE-04 exit (justfile/cargo fallbacks + GHA workflow strings).
- **User-visible outcome:** `generate` default verify uses Core command surface;
  Generated Projects include Linux CI; template repo/snapshot regenerates from
  catalog without dual-edit drift.
- **Depends on:** PHASE-02 exit
- **Requirements:** REQ-068, REQ-087, REQ-120, REQ-121, REQ-122
- **Milestones:** MS-012, MS-013, MS-014, MS-015
- **Primary risks:** RSK-002, RSK-006, RSK-007, RSK-009, RSK-102, RSK-107

### Entry Criteria

- PHASE-02 pure CLI Core generate works offline for catalog/render (MS-021 done).
- Primary gate commands exist in emitted justfile/docs (`just ci` / cargo fallbacks).

### Scope

- Verify tiers: `none` | `default` | `strict` on staged tree before place (REQ-120).
- Verify tools limited to Core surface; no cargo-deny/Miri/shear as Required (REQ-121).
- Verify runner uses a controlled environment subset and a wall-clock timeout so that host configuration cannot silently pass or fail generated projects.
- Document honestly: catalog/render offline ≠ default verify offline (REQ-060 vs REQ-120).
- GHA Core CI: fmt + clippy `-D warnings` + test on Linux; macOS Recommended
  when needed; never Windows (REQ-087).
- distribution profile optional cargo-dist (Should) (REQ-068).
- GitHub template = catalog snapshot; CI/job or release step regenerates (REQ-122 / INV-4).
- **Command-surface freeze artifacts:** MS-012 freezes justfile + cargo fallback
  primary gate strings; MS-013 freezes Core GHA workflow command surface. These
  fixtures are the join contract for PHASE-04 REQ-088/105 tests.

### Explicit Non-Goals

- Full AI skill bodies (PHASE-04).
- Final acceptance scenario suite (PHASE-05) — smoke only.
- Multi-OS release matrix beyond policy above.

### Architecture and Components

- `verify` module runs process tools against stage path.
- Template regen pipeline reads catalog, not hand-edited dual tree.
- Catalog unit (or documented module) is SoT for gate strings shared by justfile,
  GHA, and later AGENTS/DoD (INV-4 spirit).

### Integrations

- GitHub Actions for product and for Generated template snapshot.
- Optional cargo-dist evaluation for Foundry product release and Generated
  distribution profile (pin carefully; RSK-006).

### Data or Migration Work

- Template snapshot artifacts stored as generated outputs; digest compared (SPK-104).

### Evidence Spikes

| SPK     | When                                                 | Exit evidence                                                                                   |
| ------- | ---------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| SPK-103 | Before default-verify-on-generate is mandatory in CI | Documented behavior for cold cache vs warm; network expectations; fallbacks when `just` missing |
| SPK-104 | Before claiming hybrid template surface complete     | Regen produces matching digest; drift fails CI                                                  |

### Testing and Verification

- Generate with `--verify none|default|strict` matrix.
- Fail verify → no place; stage retained + path (still REQ-050).
- Template regen dry-run in CI.
- Fixture set for command-surface strings available to PHASE-04.

### Security and Reliability

- Verify must not weaken path jail or emptiness rules.
- Distribution profile must not introduce Windows installers (REQ-003).

### Dogfooding or Operational Validation

- Generate pure CLI with default verify on Linux CI runner or local warm cache.
- Owner installs Foundry via cargo install path dry-run when binaries exist.

### Rollback and Reconsideration Triggers

- If default verify is unaffordable: may document warm-cache expectation but must
  not silently claim offline generate for verify (FND-006 disposition).
- Template dual-edit detected: halt releases until SoT restored (RSK-102).

### Exit Criteria

1. Default verify runs on stage before place; failure leaves dest untouched.
2. Emitted Core workflow files present for pure CLI; no Windows jobs.
3. distribution profile emit works or explicit Should residual documented.
4. SPK-103 and SPK-104 evidence recorded.
5. Catalog digest included in release checklist.
6. Command-surface freeze fixtures from MS-012/MS-013 exist for PHASE-04 join.

---

## PHASE-04 — AI-Native Surfaces and Product Skills

- **Status:** Planned
- **Objective:** Emit portable agent surfaces for Generated Projects and Foundry
  product; teach-as-you-go; enforce no Claude Core / MCP default; **prove**
  command-surface consistency with PHASE-03 fixtures before exit.
- **User-visible outcome:** Generated repos include root `AGENTS.md` + closed
  `.agents/skills/`; TUI skill only with TUI profile; Foundry product repo has
  plan-generate / catalog-inspect / foundry-quality-gates skills; DoD strings
  match justfile/CI.
- **Depends on:** PHASE-02 emit paths (content draft); **PHASE-03 MS-012+MS-013
  for exit / MS-018** (command-surface freeze)
- **Requirements:** REQ-006, REQ-007, REQ-008, REQ-088, REQ-100, REQ-101, REQ-102,
  REQ-103, REQ-104, REQ-105, REQ-106, REQ-107, REQ-108, REQ-123
- **Milestones:** MS-016, MS-017, MS-018
- **Primary risks:** RSK-050, RSK-051, RSK-052, RSK-053, RSK-055, RSK-056, RSK-106, RSK-111

### Entry Criteria

- PHASE-02 can emit path sets for pure CLI and TUI (MS-011 complete for full
  matrix; MS-016 draft may begin after MS-011).
- Core command surface strings from PHASE-02 are treated as **provisional** until
  MS-012/MS-013 freeze fixtures exist.

### Scope

- Generated `AGENTS.md` + skills layout (REQ-100, REQ-101).
- Core skills closed set; TUI skill delta only with profile (REQ-102, REQ-103).
- Foundry product skill catalog ids + bodies (REQ-104; OQ-202).
- DoD embeds command surface; rust-analyzer not DoD (REQ-105, REQ-108).
- Multi-product portability baseline; no vendor-required Core (REQ-106).
- Repo boundary rules: secrets, no Windows invention, no Claude-required, no MCP required (REQ-107).
- No Claude-specific design target; MCP default none (REQ-006, REQ-007).
- Agents primary implementers reflected in docs (REQ-008).
- Teach-as-you-go slots for Core and architecture defaults (REQ-123).
- Command surface documentation consistency (REQ-088) — **hard exit criterion**.

### Explicit Non-Goals

- Cursor rules as default Core (OQ-109).
- Unlimited skill marketplace.
- Research-program skills leaking into Generated Projects (REQ-104 separation).
- Completing PHASE-04 exit before MS-012/MS-013 fixtures exist.

### Architecture and Components

- Catalog units for agent surface files; surface separation tests (research vs
  product vs generated) per spec §10.2.

### Integrations

- None required for MCP/editor plugins in v1 defaults.
- Join integration: AGENTS/DoD/skill strings vs MS-012/MS-013 fixtures.

### Data or Migration Work

None.

### Evidence Spikes

None new beyond continuing SPK-102 inventory tests for skill paths.

### Testing and Verification

- Forbidden path tests: no `CLAUDE.md`, no `.claude/`, no default MCP kitchen sink.
- Pure CLI: no `add-tui-screen` skill; TUI profile: skill present.
- Product skills absent from Generated trees (RSK-111).
- **REQ-088 string-match tests** against PHASE-03 emitted justfile + workflow fixtures.

### Security and Reliability

- Secrets boundary text in AGENTS (RSK-055).
- Skill catalog remains closed (RSK-050).

### Dogfooding or Operational Validation

- Agent session on Generated pure CLI: follow AGENTS.md to add a subcommand via skill.
- Agent session on Foundry product: use plan-generate skill against sample spec.

### Rollback and Reconsideration Triggers

- MCP-on or Claude-Core pressure: reject without Blueprint amendment (L7).
- Skill bloat: freeze set; expansion needs DEC.
- REQ-088 mismatch at exit: fix content or re-freeze surface — do not ship drift (RSK-052).

### Exit Criteria

1. Pure CLI and TUI emit matrices include correct agent surfaces.
2. Forbidden-path CI tests green.
3. Foundry product skills present in product repo with usable bodies.
4. Teach-as-you-go slots present for non-obvious defaults.
5. REQ-006/007/106 verified by tests or checklist automation.
6. **REQ-088 string-match tests green** against MS-012 justfile/cargo fallbacks
   and MS-013 GHA workflow fixtures (hard join; not “ideally”).

---

## PHASE-05 — Acceptance Scenarios and Freeze Gates

- **Status:** Planned
- **Objective:** Prove product readiness with automated acceptance scenarios and
  freeze release gates; residual agent operability recommended.
- **User-visible outcome:** Documented acceptance suite green for pure CLI and
  TUI paths per REQ-150/151 checklist; release process includes catalog digest +
  template regen; known residuals explicit.
- **Depends on:** PHASE-01..04 exit for v1 scope
- **Requirements:** REQ-150, REQ-151, REQ-152
- **Milestones:** MS-019, MS-020
- **Primary risks:** Residual from all prior; RSK-103 for TUI acceptance

### Entry Criteria

- PHASE-01..04 exit criteria met for features claimed in v1 ship set.
- SPK-100..104 evidence available or residuals explicitly accepted by owner.

### Scope

- Automated acceptance: validate/plan/generate pure CLI (REQ-150) — **full scenario matrix**.
- Automated acceptance: TUI profile matrix (REQ-151) — **full scenario matrix**.
- Recommended agent operability trial (REQ-152 / SPK-050/051 equivalent).
- Freeze gates: no silent REQ drops; REC ledger still 49/49 in spec; locks intact.
- Release readiness: install paths, digest, template snapshot, changelog discipline.
- **Note:** Unit-level SPK-101/102 evidence does **not** substitute for end-to-end
  REQ-150/151 acceptance jobs (fixtures may be shared; the ship gate is E2E).

### Explicit Non-Goals

- Expanding v1 scope (new profiles, Windows, merge-update).
- Elevating REQ-152 from Should to Must without DEC.

### Architecture and Components

- Acceptance tests live in product repo CI; may use golden fixtures from spikes.
- CI jobs **name** the scenarios they cover (not only “suite exists”).

### Integrations

- CI required green on Linux for ship.

### Data or Migration Work

None.

### Evidence Spikes

| SPK                         | When                        | Exit evidence                                                            |
| --------------------------- | --------------------------- | ------------------------------------------------------------------------ |
| SPK-050/051 (or equivalent) | Before claiming agent-ready | Owner/agent trial notes; residual if matrix unavailable (REQ-152 Should) |

### Testing and Verification

- REQ-150/151 automated scenarios in CI (checklist below).
- Spot-check override equality, refuse non-empty, path jail still green.

### Security and Reliability

- Final pass: no Windows jobs; no secret fixtures; path jail still enforced.

### Dogfooding or Operational Validation

- Owner generates real personal project from Foundry; exercises quality-gates skill.
- Foundry dogfoods its own quality gates.

### Rollback and Reconsideration Triggers

- Acceptance red on write safety or TUI leakage: **block ship**; return to owning phase.
- Desire for merge-existing or remote catalog: requires Blueprint/DEC, not hotpatch.

### Exit Criteria

1. **REQ-150 scenario checklist green in CI** (every bullet; named jobs/paths):
   - [ ] `validate` + `plan` succeed on sample pure-CLI spec
   - [ ] `generate` into **missing or empty** dest yields runnable CLI
   - [ ] `generate` refuses **non-empty** dest (including dir with `.git`)
   - [ ] `generate` refuses **file-at-path** dest
   - [ ] `generate` refuses **symlink** dest
   - [ ] Plan file digests match placed tree for matching effective inputs
         (including override pairs)
   - [ ] Path-jail fixtures fail closed
   - [ ] Pure CLI: **no ratatui** / **no add-tui-screen**
   - [ ] Primary gate passes post-generate when network/cache available
   - [ ] **No CLAUDE.md**; **no default MCP**
2. **REQ-151 scenario checklist green in CI**:
   - [ ] TUI profile sample generates with TUI entry
   - [ ] ratatui/crossterm present
   - [ ] add-tui-screen skill present
   - [ ] Primary gate still passes
   - [ ] Claude/MCP defaults still forbidden
3. REQ-152 completed or residual documented with owner acceptance (Should).
4. Release checklist includes catalog digest + template regen (REQ-122).
5. No known Critical/High open defects on REQ-050/051/053/034/040/063/004/065.
6. Ship decision recorded by owner (product repo release tag).

---

## 9. Milestones

### MS-001 — Product repository bootstrap

- **Phase:** PHASE-01
- **Outcome:** Rust crate skeleton, CI Linux-only, module map stubs, license/README.
- **Prerequisites:** Spec authority accepted; product repo location chosen.
- **Acceptance evidence:**
  - `cargo fmt --check` passes.
  - `cargo clippy --all-targets -- -D warnings` passes.
  - `cargo test` passes (possibly empty test suite).
  - Linux-only CI workflow present; no Windows workflow.
- **Blocks:** MS-002

### MS-002 — Spec parse and validate command

- **Phase:** PHASE-01
- **Outcome:** TOML schema 1 parse; denylist; unknown keys fail; `validate` CLI.
- **Prerequisites:** MS-001
- **Acceptance evidence:** Fixture specs pass/fail as specified (REQ-030..033).
- **Blocks:** MS-003

### MS-003 — Pure Construct plan command

- **Phase:** PHASE-01
- **Outcome:** `plan` emits text/JSON plan elements; zero dest writes; overrides in Construct.
- **Architecture gate:** `src/plan/` MUST NOT import `crate::fsx`, `crate::generate`, `crate::cli`, or any I/O boundary module; enforced by crate structure or source-scan test.
- **Error contract:** Stable error codes for validation/override/plan failures; emitted in both text and JSON (`--format json`) outputs.
- **Prerequisites:** MS-002
- **Acceptance evidence:** Override equality tests; write-free sandbox (REQ-034/040..043); error-code fixture matrix green; cross-command `plan_sha256` equality for representative fixtures.
- **Blocks:** MS-004
- **Note:** Does **not** block MS-005 directly; SPK-100 (MS-004) must freeze plan JSON first.

### MS-004 — SPK-100 golden plan freeze

- **Phase:** PHASE-01
- **Outcome:** Golden plan JSON for minimal CLI; element set frozen; key names documented.
- **Method:** Snapshot tests in `tests/snapshots/plan/` (e.g., `insta`); `cargo test` fails on drift; `cargo insta review` is the accepted update path; JSON redaction for unstable catalog digests.
- **Prerequisites:** MS-003; plan-package purity scan green.
- **Acceptance evidence:** Golden fixtures committed; OQ-200 residual closed or listed.
- **Blocks:** MS-005, later PHASE-02 goldens
- **Rule:** After this milestone, plan JSON field names are stable for fixtures;
  additive renames need tests (§11).

### MS-005 — Stage, place, generate lifecycle

- **Phase:** PHASE-01
- **Outcome:** `generate` stages, optional stub verify hook, exclusive place; lifecycle table.
- **Architecture:** `render` produces a pure `BTreeMap<RelativePath, RenderedFile>` (or equivalent) so that placement, parallelism, and atomicity are handled in `fsx`; `fsx` writes the map into the stage root atomically/transactionally; parallel placement is permitted but not required in v1.
- **Contract check:** `generate` computes its plan independently and asserts the digest matches the MS-003 contract for the same effective inputs; on mismatch, fail before any FS write.
- **Prerequisites:** MS-004
- **Acceptance evidence:** Success cleans stage; fail retains + path (REQ-050).
- **Blocks:** MS-006

### MS-006 — SPK-101 emptiness/place/jail matrix

- **Phase:** PHASE-01
- **Outcome:** Full destination matrix + path jail fixtures green.
- **Prerequisites:** MS-005
- **Acceptance evidence:** Table-driven tests for REQ-051/053 cases.
- **Blocks:** PHASE-01 exit → MS-007

### MS-007 — Embed catalog and digests

- **Phase:** PHASE-02
- **Outcome:** Embedded catalog load; `catalog list|show`; `version` digest.
- **Extension:** `foundry sample-spec` emits a valid schema-1 TOML for the current
  catalog; acceptance includes schema validation of emitted sample.
- **Prerequisites:** PHASE-01 exit
- **Acceptance evidence:** Offline catalog ops; digest stable (REQ-025/060/062).
- **Blocks:** MS-008

### MS-008 — Core pure-CLI emit

- **Phase:** PHASE-02
- **Outcome:** Core unit always emits single-crate CLI with toolchain/edition/gates files.
- **Prerequisites:** MS-007
- **Acceptance evidence:** Tree fixtures match Core matrix (REQ-009/064/080..086).
  Lightweight template-SoT dry-run: generate a pure-CLI tree and compare digest/shape to the existing template snapshot; fail on drift.
- **Blocks:** MS-021

### MS-021 — Pure-CLI Core catalog generate dogfood gate

- **Phase:** PHASE-02
- **Outcome:** Thin vertical integration: offline pure-CLI generate from embedded
  catalog works before profile/TUI expansion.
- **Prerequisites:** MS-008
- **Acceptance evidence (all required):**
  1. Offline pure-CLI generate from embedded catalog succeeds on Linux.
  2. Construct/plan golden for that tree matches SPK-100 continuation for Core.
  3. Owner or agent smoke: open project, `cargo test` (not full default verify).
- **Blocks:** MS-009, MS-010
- **Note:** Full default verify remains PHASE-03; this gate is catalog/render integration only.

### MS-009 — Profile composition order

- **Phase:** PHASE-02
- **Outcome:** Canonical order; permutation-invariant digests; hooks/secrets emit.
- **Dependency rule:** Catalog units declare `requires` edges; `resolve` topologically sorts selected profiles or errors on unsupported combinations; canonical order is the topological default.
- **Prerequisites:** MS-021
- **Acceptance evidence:** REQ-063 tests; profile fixtures (REQ-066/067); unsupported profile combinations hard-fail with clear errors.
- **Blocks:** MS-010, MS-011

### MS-010 — TUI generate-time inclusion

- **Phase:** PHASE-02
- **Outcome:** `tui` profile adds TUI files/deps only when selected.
- **Dependency rule:** `tui` has no required predecessors; it folds before `hooks`.
- **Prerequisites:** MS-021, MS-009
- **Acceptance evidence:** Side-by-side pure CLI vs TUI fixtures (REQ-004/065).
- **Blocks:** MS-011

### MS-011 — SPK-102 separation gate

- **Phase:** PHASE-02
- **Outcome:** Forbidden-path and dep leakage tests for pure CLI.
- **Prerequisites:** MS-010
- **Acceptance evidence:** SPK-102 report/tests green.
- **Blocks:** PHASE-02 exit → MS-012; MS-016 (draft start)

### MS-012 — Verify tiers on stage (command-surface freeze: justfile/cargo)

- **Phase:** PHASE-03
- **Outcome:** none/default/strict verify; failure blocks place; **primary gate
  strings + cargo fallbacks frozen** for justfile/docs.
- **Isolation:** Verify runner executes with a sanitized environment subset
  (e.g., `PATH`, `HOME`, `RUSTUP_HOME`, `CARGO_HOME` retained; user `RUSTFLAGS`/`CARGO_TARGET_DIR` stripped or prefixed) and a per-test timeout so host state cannot silently pass/fail generated projects.
- **Prerequisites:** MS-011; Core justfile/cargo fallbacks exist
- **Acceptance evidence:** SPK-103 notes + tests (REQ-120/121); freeze fixture committed; env-hygiene and timeout fixtures green.
- **Blocks:** MS-013; MS-018 (with MS-013)

### MS-013 — Generated GHA Core CI (command-surface freeze: workflow)

- **Phase:** PHASE-03
- **Outcome:** Linux CI workflow emitted; no Windows; mac policy documented;
  **workflow command surface frozen** for REQ-088 join.
- **Prerequisites:** MS-008 (emit paths); MS-012 recommended for string alignment
- **Acceptance evidence:** Workflow fixture inspect (REQ-087); freeze fixture committed.
- **Blocks:** MS-014; MS-018 (with MS-012)

### MS-014 — Distribution profile (optional)

- **Phase:** PHASE-03
- **Outcome:** cargo-dist profile emit when selected; pins documented.
- **Prerequisites:** MS-009
- **Acceptance evidence:** Fixture or explicit Should residual (REQ-068).
- **Blocks:** MS-015 (soft)

### MS-015 — GH template catalog SoT regen

- **Phase:** PHASE-03
- **Outcome:** Template snapshot regenerated from catalog; digest check.
- **Prerequisites:** MS-008, MS-012 recommended
- **Acceptance evidence:** SPK-104 green (REQ-122).
- **Note:** Dry-run digest check introduced in MS-008; MS-015 makes it CI-automated.
- **Blocks:** PHASE-03 exit

### MS-016 — Generated AI-native Core surface (draft-capable)

- **Phase:** PHASE-04
- **Outcome:** AGENTS.md + Core skills on pure CLI; boundaries; DoD text draft.
- **Prerequisites:** MS-011
- **Acceptance evidence:** Content checklists + path tests (REQ-100..102, 105..108);
  DoD strings marked **provisional** until MS-018 REQ-088 match.
- **Blocks:** MS-017
- **Note:** May run // PHASE-03; does **not** alone complete PHASE-04 exit.

### MS-017 — TUI skill delta and teach-as-you-go

- **Phase:** PHASE-04
- **Outcome:** TUI skill only with profile; teach slots; no Claude/MCP defaults.
- **Prerequisites:** MS-016, MS-010
- **Acceptance evidence:** REQ-006/007/103/123 tests.
- **Blocks:** MS-018

### MS-018 — Foundry product skills + command-surface join

- **Phase:** PHASE-04
- **Outcome:** plan-generate, catalog-inspect, foundry-quality-gates bodies usable;
  **REQ-088 string-match green** against MS-012 + MS-013 fixtures.
- **Prerequisites:** MS-016, MS-017, **MS-012, MS-013**
- **Acceptance evidence:** Skill paths + dry-run agent checklist (REQ-104);
  REQ-088 tests vs justfile + GHA fixtures green.
- **Blocks:** PHASE-04 exit → MS-019

### MS-019 — Acceptance scenario automation

- **Phase:** PHASE-05
- **Outcome:** REQ-150/151 scenarios in CI with named checklist coverage.
- **Prerequisites:** PHASE-01..04 exits for claimed features
- **Acceptance evidence:** Every bullet under PHASE-05 Exit Criteria items 1–2
  green in CI; job names/paths recorded in product repo acceptance docs.
  Bare “CI green” is **not** sufficient.
- **Blocks:** MS-020

### MS-020 — Ship freeze and release readiness

- **Phase:** PHASE-05
- **Outcome:** Release checklist; residuals; optional agent operability (REQ-152).
- **Prerequisites:** MS-019
- **Acceptance evidence:** Owner ship decision; digest + template regen on release.
- **Blocks:** v1 release (product)

---

## 10. Cross-Phase Integration

| Integration                                       | Phases                          | Contract                                                                                                                                  |
| ------------------------------------------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Plan-as-contract                                  | 01→all                          | Same Construct; overrides before Construct; goldens evolve only with intentional catalog changes; **JSON names stable only after MS-004** |
| Write safety                                      | 01→all                          | Stage/place/jail never relaxed by verify or profiles                                                                                      |
| Catalog digest                                    | 02→03→05                        | `version`, template snapshot, release notes share digest definition                                                                       |
| Emit matrix                                       | 02→04                           | AI files are catalog units subject to same composition rules                                                                              |
| Verify vs offline                                 | 02↔03                           | Catalog offline tests stay separate from verify network tests                                                                             |
| TUI leakage                                       | 02→04→05                        | SPK-102 + acceptance REQ-151 continuous                                                                                                   |
| Pure-CLI dogfood                                  | 02                              | MS-021 before profile/TUI expansion                                                                                                       |
| **Command surface freeze → AI DoD / verify docs** | **03→04**                       | **MS-012/MS-013 freeze fixtures; PHASE-04 exit + MS-018 require REQ-088 match**                                                           |
| Template SoT dry-run                              | 02→03                           | MS-008 dry-run digest check; MS-015 CI automation                                                                                         |
| Hybrid surface                                    | 01 docs → 03 template → 05 ship | L1 facets all present at ship                                                                                                             |
| Dogfood gates                                     | 01→05                           | Foundry product uses same quality philosophy as Generated Core                                                                            |
| Acceptance E2E                                    | 05                              | REQ-150/151 checklist; unit spikes do not substitute                                                                                      |

Continuous integration rule: each phase adds fixtures; later phases must not
disable earlier safety tests.

---

## 11. Data or Migration Sequencing

| Topic                   | Plan                                                                   |
| ----------------------- | ---------------------------------------------------------------------- |
| Project Spec schema     | v1 only (`schema = 1`); unknown keys hard-fail; no dual-schema support |
| Plan JSON keys          | Freeze elements after MS-004 (SPK-100); additive changes need tests    |
| Catalog content         | Append-only closed set; removals need DEC                              |
| GH template             | Always regenerated from catalog; never manual dual-edit                |
| Existing user projects  | No migration / merge / update in v1                                    |
| Research → product      | Specs/plans remain in research repo; code only in product repo         |
| Command-surface strings | Freeze at MS-012/MS-013; AGENTS/DoD must match before PHASE-04 exit    |

---

## 12. Testing Strategy by Phase

| Phase    | Focus                            | Minimum evidence                                                                                                                                                                 |
| -------- | -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| PHASE-01 | Unit + FS sandbox + CLI          | Override equality; write-free plan; SPK-100 **before** generate shapes; SPK-101; path jail; snapshot goldens; plan purity scan; stable error codes; property-based fuzz fixtures |
| PHASE-02 | Fixture trees + matrix           | Core emit; **MS-021 dogfood**; profile permutations; SPK-102; offline catalog                                                                                                    |
| PHASE-03 | Process integration              | Verify tiers; workflow fixtures; SPK-103/104; command-surface freeze                                                                                                             |
| PHASE-04 | Content + forbidden paths + join | AGENTS/skills inventories; Claude/MCP absence; **REQ-088 match**                                                                                                                 |
| PHASE-05 | End-to-end acceptance            | **REQ-150/151 scenario checklist CI**; optional REQ-152                                                                                                                          |

Shared rules: no Windows CI jobs; secrets never in fixtures as real material;
tests must be non-interactive.

---

## 13. Security Activities by Phase

| Phase    | Activities                                                                              |
| -------- | --------------------------------------------------------------------------------------- |
| PHASE-01 | Denylist (REQ-033); path jail (REQ-053); fail-closed place (REQ-050/051)                |
| PHASE-02 | Closed catalog (no remote code fetch); secrets profile hygiene; no secret templates     |
| PHASE-03 | Verify surface limited; distribution without Windows installers; template SoT integrity |
| PHASE-04 | AGENTS boundary rules; no MCP kitchen sink; no Claude-required surfaces                 |
| PHASE-05 | Final security regression on jail/emptiness/forbidden paths                             |

Threat model remains personal tool, fail-closed writes; no multi-tenant claims.

---

## 14. Operations and Release Readiness

| Item                      | When                                  | Notes                            |
| ------------------------- | ------------------------------------- | -------------------------------- |
| Product CI (Linux)        | PHASE-01+                             | Required                         |
| macOS CI                  | When claiming mac bins / distribution | Recommended (REQ-087)            |
| cargo install path        | PHASE-03+                             | Primary distribution for Foundry |
| GH release binaries       | Evaluate with cargo-dist              | Optional; pin carefully          |
| Catalog digest in release | PHASE-02+                             | Must at ship (PHASE-05)          |
| Template snapshot regen   | PHASE-03+                             | Must on catalog change + release |
| Version command           | PHASE-02                              | Reports digest                   |
| Support OS statement      | All phases                            | macOS + Linux only               |

Release is blocked if SPK-101/102 regressions exist, template digest drifts, or
REQ-150/151 checklist incomplete.

---

## 15. Dogfooding

| Layer              | Practice                                                                              |
| ------------------ | ------------------------------------------------------------------------------------- |
| Foundry product    | Implemented in Rust (L2); uses fmt/clippy/test; product skills for agent implementers |
| Generated Projects | Owner generates real CLIs; runs quality-gates; adds subcommands via skills            |
| Hybrid template    | Occasional generate-from-template path to detect SoT drift                            |
| Research program   | Does **not** dogfood by implementing product crates here (L12)                        |

**Dogfood before broad profile expansion (normative):** MS-021 pure-CLI catalog
generate + `cargo test` smoke **before** MS-009/MS-010. Full default verify for
owner-generated projects before investing heavily in distribution polish remains
PHASE-03 guidance (not a PHASE-02 exit substitute for MS-021).

---

## 16. Risk Register

| ID      | Risk                        | Plan mitigation                                        | Phase focus | Residual                |
| ------- | --------------------------- | ------------------------------------------------------ | ----------- | ----------------------- |
| RSK-100 | Destructive overwrite       | REQ-050/051; SPK-101 before catalog breadth            | 01          | Override DEC later only |
| RSK-101 | Plan/apply divergence       | Pure Construct; golden plans; MS-004 before MS-005     | 01–02       | Implementer error       |
| RSK-109 | Path escape                 | REQ-053 fixtures continuous                            | 01–05       | Impl bugs               |
| RSK-112 | Stage leftovers             | Success clean / fail path print                        | 01          | Disk if abandoned       |
| RSK-110 | Secrets in spec             | Denylist + AGENTS                                      | 01, 04      | Non-denylist keys       |
| RSK-103 | TUI leakage                 | SPK-102 + acceptance REQ-151; MS-021 before TUI matrix | 02, 05      | Template bugs           |
| RSK-102 | Catalog/template drift      | REQ-122; SPK-104                                       | 03, 05      | CI failure mode         |
| RSK-107 | Verify host gaps            | cargo fallbacks; SPK-103                               | 03          | just missing            |
| RSK-006 | cargo-dist churn            | Optional profile only                                  | 03          | Pin carefully           |
| RSK-007 | macOS CI skip               | Linux Required policy                                  | 03          | Coverage gap            |
| RSK-051 | MCP creep                   | REQ-007 tests                                          | 04          | Social pressure         |
| RSK-052 | DoD drift from CI           | REQ-088 hard PHASE-04 exit; MS-012/013 freeze          | 03–04       | Copy edits after freeze |
| RSK-053 | Agent product overfit       | REQ-006/106                                            | 04          | Adapter temptation      |
| RSK-111 | Product skills in Generated | Separation tests                                       | 04          | —                       |
| RSK-104 | Profile explosion           | Closed set + DEC                                       | 02          | Demand pressure         |
| RSK-105 | Engine complexity           | Single crate Default                                   | 01–02       | Complexity              |

Plan-local delivery risks:

| ID           | Risk                                    | Mitigation                                                    |
| ------------ | --------------------------------------- | ------------------------------------------------------------- |
| RSK-PLAN-001 | Implementing in research repo           | Explicit product-repo assumption; REQ-010                     |
| RSK-PLAN-002 | Plan treated as REQ override            | Authority section; spec wins                                  |
| RSK-PLAN-003 | Skipping spikes under schedule pressure | Exit criteria require SPK evidence or owner-accepted residual |
| RSK-PLAN-004 | Agent-ambiguous milestone graphs        | Single DAG; hard Prerequisites/Blocks (FND-200 disposition)   |
| RSK-PLAN-005 | Soft parallel joins                     | Hard PHASE-04 exit REQ-088 (FND-201 disposition)              |
| RSK-PLAN-006 | Performative “CI green” ship            | REQ-150/151 checklist (FND-202 disposition)                   |

---

## 17. Open Questions

Carry from revised spec (non-blocking unless noted):

| ID     | Question                    | Plan handling                              |
| ------ | --------------------------- | ------------------------------------------ |
| OQ-200 | Exact plan JSON field names | SPK-100 in MS-004 (before MS-005)          |
| OQ-201 | Embed crate choice          | Decide in MS-007; document                 |
| OQ-202 | Foundry skill body prose    | MS-018                                     |
| OQ-104 | Foundry product rustc pin   | Bootstrap MS-001; dogfood Generated policy |
| OQ-106 | GH template regen cadence   | On catalog change + release (MS-015)       |
| OQ-107 | Saved plan file apply       | Deferred post-v1                           |
| OQ-109 | Cursor rules                | Not default Core                           |
| OQ-110 | Secrets skill               | Deferred; profile scaffolding first        |

Closed provisional (do not reopen without evidence): OQ-100..103, OQ-105, OQ-003, OQ-051.

**Plan-local OQs:**

| ID          | Question                                       | Blocking? | Disposition                        |
| ----------- | ---------------------------------------------- | --------- | ---------------------------------- |
| OQ-PLAN-001 | Product repository name/host                   | No        | Owner chooses at PHASE-01 entry    |
| OQ-PLAN-002 | Whether distribution profile is in v1 ship set | No        | Should; residual allowed at MS-014 |
| OQ-PLAN-003 | Depth of REQ-152 agent matrix                  | No        | Recommended; document residual     |

---

## 18. Rollback and Reconsideration Triggers

| Trigger                                                             | Action                                                   |
| ------------------------------------------------------------------- | -------------------------------------------------------- |
| Write safety defect (partial place, non-empty clobber, jail escape) | Halt ship; fix in PHASE-01 semantics before new features |
| Pure CLI TUI leakage                                                | Halt profile expansion; return to SPK-102                |
| MS-021 pure-CLI dogfood fails                                       | Do not start MS-009/MS-010; fix Core                     |
| REQ-088 mismatch at PHASE-04 exit                                   | Fix AI content or re-freeze surface; do not ship         |
| Proposal to default merge-into-non-empty                            | Reject unless Blueprint amendment + DEC                  |
| Windows support request                                             | Reject (L3)                                              |
| Remote catalog / marketplace                                        | Reject without Blueprint amendment                       |
| MCP default on / Claude Core                                        | Reject (L7)                                              |
| Construct not shared across validate/plan/generate                  | Stop catalog work; restore INV-3                         |
| Cross-device place needs copy+swap                                  | Requires DEC; default remains fail-closed                |
| New profile id                                                      | Requires DEC; closed set                                 |
| Plan conflicts with revised REQ                                     | Spec wins; revise plan, do not hot-weaken REQ            |
| Major new machinery in plan                                         | Risk-trigger possible extra plan-review                  |

---

## 19. Requirement-to-Phase Traceability

Every REQ from revised specification §23:

| REQ     | Phase    | Notes                                                     |
| ------- | -------- | --------------------------------------------------------- |
| REQ-001 | PHASE-01 | Hybrid shape in product docs + facets through PHASE-03/05 |
| REQ-002 | PHASE-01 | Rust product language                                     |
| REQ-003 | PHASE-01 | OS policy continuous in CI                                |
| REQ-004 | PHASE-02 | CLI primary; verified SPK-102/REQ-151; after MS-021       |
| REQ-005 | PHASE-02 | Closed sets; skills freeze PHASE-04                       |
| REQ-006 | PHASE-04 | No Claude target                                          |
| REQ-007 | PHASE-04 | MCP default none                                          |
| REQ-008 | PHASE-04 | Agents primary                                            |
| REQ-009 | PHASE-02 | Single-crate generate                                     |
| REQ-010 | N/A      | Governance: product code not in research repo             |
| REQ-020 | PHASE-01 | Lifecycle commands (catalog complete PHASE-02)            |
| REQ-021 | PHASE-01 | Non-interactive                                           |
| REQ-022 | PHASE-01 | validate/plan write-free                                  |
| REQ-023 | PHASE-01 | Exit codes                                                |
| REQ-024 | PHASE-01 | clap derive Foundry CLI                                   |
| REQ-025 | PHASE-02 | version + catalog digest                                  |
| REQ-030 | PHASE-01 | TOML schema 1                                             |
| REQ-031 | PHASE-01 | Explicit `--spec`                                         |
| REQ-032 | PHASE-01 | Archetype `cli` only                                      |
| REQ-033 | PHASE-01 | Secret denylist                                           |
| REQ-034 | PHASE-01 | CLI overrides                                             |
| REQ-040 | PHASE-01 | Plan-as-contract                                          |
| REQ-041 | PHASE-01 | Plan elements; SPK-100 at MS-004                          |
| REQ-042 | PHASE-01 | Plan formats                                              |
| REQ-043 | PHASE-01 | Plan zero dest writes                                     |
| REQ-050 | PHASE-01 | Stage lifecycle                                           |
| REQ-051 | PHASE-01 | Emptiness/place; SPK-101                                  |
| REQ-052 | PHASE-01 | No merge/update v1                                        |
| REQ-053 | PHASE-01 | Path jail                                                 |
| REQ-060 | PHASE-02 | Closed embedded catalog offline                           |
| REQ-061 | PHASE-02 | Custom planner-led engine                                 |
| REQ-062 | PHASE-02 | Catalog unit set                                          |
| REQ-063 | PHASE-02 | Emit composition order                                    |
| REQ-064 | PHASE-02 | Core always emits; MS-021 dogfood                         |
| REQ-065 | PHASE-02 | TUI generate-time inclusion after MS-021                  |
| REQ-066 | PHASE-02 | hooks profile                                             |
| REQ-067 | PHASE-02 | secrets profile                                           |
| REQ-068 | PHASE-03 | distribution profile                                      |
| REQ-069 | PHASE-02 | rust-version floor                                        |
| REQ-070 | PHASE-02 | tracing default full CLI                                  |
| REQ-080 | PHASE-02 | Toolchain pin file                                        |
| REQ-081 | PHASE-02 | Edition 2024                                              |
| REQ-082 | PHASE-02 | rustfmt/clippy                                            |
| REQ-083 | PHASE-02 | cargo test Required                                       |
| REQ-084 | PHASE-02 | anyhow/thiserror policy                                   |
| REQ-085 | PHASE-02 | clap derive Generated CLI                                 |
| REQ-086 | PHASE-02 | just Default                                              |
| REQ-087 | PHASE-03 | GHA Core CI; MS-013 freeze                                |
| REQ-088 | PHASE-04 | Command surface docs; hard exit join                      |
| REQ-089 | PHASE-02 | Licensing posture                                         |
| REQ-100 | PHASE-04 | AGENTS + skills layout                                    |
| REQ-101 | PHASE-04 | AGENTS content contract                                   |
| REQ-102 | PHASE-04 | Generated Core skills                                     |
| REQ-103 | PHASE-04 | TUI skill delta                                           |
| REQ-104 | PHASE-04 | Foundry product skills                                    |
| REQ-105 | PHASE-04 | DoD embeds command surface                                |
| REQ-106 | PHASE-04 | Portability baseline                                      |
| REQ-107 | PHASE-04 | Boundary rules                                            |
| REQ-108 | PHASE-04 | rust-analyzer not DoD                                     |
| REQ-120 | PHASE-03 | Default verify tier; MS-012 freeze                        |
| REQ-121 | PHASE-03 | Verify tools limited                                      |
| REQ-122 | PHASE-03 | GH template catalog SoT                                   |
| REQ-123 | PHASE-04 | Teach-as-you-go                                           |
| REQ-130 | PHASE-01 | Module boundaries Should                                  |
| REQ-150 | PHASE-05 | Acceptance pure CLI — checklist on exit/MS-019            |
| REQ-151 | PHASE-05 | Acceptance TUI matrix — checklist on exit/MS-019          |
| REQ-152 | PHASE-05 | Acceptance agent operability Should                       |
| REQ-160 | N/A      | Prior-art transfer posture (docs/spec integrity)          |

**REC ledger:** Implementation phases consume surviving REQs only. Spec §28 remains
**49/49** dispositions. This plan does not drop, renumber, or re-disposition RECs.

**Governance REQs (N/A phase):** REQ-010 and REQ-160 constrain process and narrative
continuously; they are not product-crate deliverables.

---

## 20. Definition of Plan Completion

This **revised** plan is complete for validation and human acceptance when:

1. All contract sections above are filled and standalone.
2. Finding Disposition Ledger covers FND-200..FND-203 with exactly one enum each.
3. Integrated corrections appear in body (not ledger-only).
4. PHASE-01..05 cover revised-spec phase focuses with executable entry/exit criteria.
5. Milestones form a **single consistent DAG** (graph + Prerequisites + Blocks).
6. SPK-100..104 are scheduled with consumer phases.
7. Requirement-to-phase table includes every REQ from §23.
8. Load-bearing write safety, plan-as-contract, offline≠verify, emit/TUI,
   AI-native, and acceptance concerns are explicit in phase scope/exits.
9. Product implementation location is explicit (separate repo).
10. No remaining implementation-blocking plan findings.
11. Independent validation (`research-validate`) has run.
12. Human approval + `accepted_commit` recorded for stage `plan-revision` (done).

**Delivery authority:** After human acceptance of this stage, this document is
**`Accepted — delivery authority`** for _how_ to sequence the product build. It
remains subordinate to the revised definitive specification for _what_ to build.

---

## 21. Product Delivery / Program Handoff

When this plan is human-accepted:

1. Research graph stages discovery → plan-revision are **complete**.
2. Product work begins in the **separate Foundry product repository** under this
   plan’s phases/milestones (not in the research repo).
3. Implementation authority for product behavior remains
   `docs/specifications/02-definitive-specification-revised.md`.
4. Do not treat baseline plan `01-…` as delivery authority.
5. Do not reopen RECs/FNDs without DEC or new evidence.
6. Optional: update program `HANDOFF.md` for product delivery orientation;
   mark program status completed when owner so decides.

**Recommended first product steps after accept:**

1. Create/designate product repository (MS-001).
2. Follow PHASE-01 DAG strictly: MS-003 → MS-004 → MS-005 → MS-006.
3. Do not expand catalog profiles/TUI until MS-021 is green.

---

## Completion Checklist

- [x] Full standalone revised implementation plan written
- [x] Finding Disposition Ledger covers FND-200..FND-203 with exactly one enum each
- [x] High finding FND-200 Accepted and integrated
- [x] Integrated Correction Ledger present
- [x] Preserved Strengths present
- [x] Single consistent milestone DAG (§6 + MS Prerequisites + MS Blocks)
- [x] PHASE-04 // PHASE-03 join hard rule for REQ-088/DoD (FND-201)
- [x] PHASE-05 / MS-019 bound to REQ-150/151 scenario checklist (FND-202)
- [x] Pure-CLI dogfood gate MS-021 before profile/TUI expansion (FND-203)
- [x] Load-bearing write safety, plan-as-contract, offline≠verify, emit/TUI, AI, acceptance preserved
- [x] Requirement-to-phase table covers every REQ in revised spec §23
- [x] REC ledger integrity acknowledged (49/49; no silent drop)
- [x] Locks L1–L9/L12 and non-goals intact
- [x] Product implementation location explicit (not this research repo)
- [x] Status honest: Proposed — pending validation and human acceptance; eligible for delivery authority
- [x] Updated handoff toward product delivery / program completion
- [x] No product code; baseline `01-…` not rewritten as output
- [x] Artifact standalone without chat history
- [x] Phases and milestones only (no task backlog)
- [x] Independent validation (`research-validate`) — see `docs/validations/02-implementation-plan-revised-validation.md`
- [x] Human approval of plan-revision stage
