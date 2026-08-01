# Program Blueprint — rust-foundry

- **Artifact type:** Program Blueprint
- **Program:** rust-foundry
- **Status:** Accepted
- **Version:** 0.1
- **Created:** 2026-08-01
- **Last updated:** 2026-08-01
- **Rigor tier:** standard (approved in discovery framing)

> Discovery framing approved (2026-08-01). Blueprint accepted by human
> (2026-08-01). This document does **not** conduct substantive research; it
> governs the program. Accepting commit is recorded in `research-program.toml`.

## 1. Artifact Metadata

| Field      | Value                                              |
| ---------- | -------------------------------------------------- |
| Program ID | rust-foundry                                       |
| Owner      | robertguss                                         |
| Repository | https://github.com/robertguss/rust-foundry         |
| Role       | Research program repository (methodology + design) |

**Related prior art (transferable reference, not authority here):**

- [go-foundry-cli](https://github.com/robertguss/go-foundry-cli) — shipped Go Foundry product patterns
- [python-foundry](https://github.com/robertguss/python-foundry) — sibling research program (Python hybrid foundry)
- [go-foundry-research](https://github.com/robertguss/go-foundry-research) — research process this methodology family was abstracted from (if referenced)

## 2. Product or Project Vision

**rust-foundry** is a personal, open-sourceable, **AI-native hybrid foundry**
for modern Rust CLI and TUI projects:

1. A **Rust-implemented** generator CLI that turns a declarative project input
   into a complete repository via **validate → plan (dry-run) → generate**
   (adapted from go-foundry / python-foundry, re-researched for Rust).
2. A **strong default Core** (toolchain, layout, quality gates, hooks/secrets,
   CI, agent surface) so new work starts fast and consistently.
3. Optional **capability profiles** / archetypes: **CLI** as the primary path;
   **TUI** optional (not every CLI includes a TUI).
4. A surface usable as a **GitHub template** for the default path.
5. Generated projects and the foundry itself are **agent-first**: organized,
   documented, and instrumented (skills, curated agent config) so AI coding
   agents can understand and extend them without oral tradition.
6. Defaults and docs carry **teach-as-you-go** legibility — important because
   the owner has less Rust depth than Go/Python.

This repository is the **research and specification program** for that product.
It stops at a revised definitive specification and a phase/milestone
implementation plan — not a granular coding backlog.

## 3. Problem Statement

The owner repeatedly starts (or wants to start) Rust CLI/TUI projects and must
re-establish toolchain pins, layout, lint/format/test gates, hooks, secrets
handling, CI, and agent-operable structure. That costs time, produces
inconsistent bases, and forces re-explaining repo conventions to AI coding
agents (the primary implementers). The owner has **less experience in Rust**
than in Python and Go, so silent or fashionable defaults are higher risk.
Existing one-shot scaffolds (`cargo init`, generic templates) do not encode a
closed, research-backed Core plus a dry-run generation workflow and AI-native
agent surface comparable to the go-foundry / python-foundry direction.

## 4. Intended Users and Stakeholders

| Role                     | Relationship                                                       |
| ------------------------ | ------------------------------------------------------------------ |
| **Primary operator**     | Owner (robertguss) — directs work, accepts gates                   |
| **Primary implementers** | AI coding agents (Grok Build, Cursor, Codex, and similar)          |
| **Secondary**            | Open-source readers/adopters (not v1 design focus)                 |
| **Out of scope (v1)**    | Multi-tenant orgs, template marketplaces, unknown teams as primary |

## 5. Goals

1. Design a **hybrid foundry**: generator CLI + strong default Core (+ template surface).
2. Establish **evidence-backed** modern Rust (2026) practices for Core tooling and layout, with extra rigor because the owner is less deep in Rust.
3. Support **CLI** as primary archetype and **TUI** as optional (not bundled into every CLI).
4. Make **AI-native operability** first-class: portable agent skills, clear docs/structure, closed curated agent surface for foundry and generated projects.
5. **Dogfood Rust**: implement the foundry product in Rust.
6. Adapt useful **go-foundry** and **python-foundry** patterns (spec → plan → generate, Core vs profiles, closed catalogs, AI-native surface) under **balanced transfer** — do not copy blindly.
7. Produce an **accepted revised definitive specification** and **revised implementation plan** (phases/milestones) sufficient to build the product later.
8. Keep honest **Core candidates** (research confirms or revises): rustup + `rust-toolchain.toml`, edition 2024 if practical, rustfmt, clippy, cargo test (+ evaluate nextest), anyhow/thiserror, clap, ratatui (TUI), just, pre-commit and/or hk, fnox (+ age), GitHub Actions, evaluate cargo-dist / release tooling, root `AGENTS.md` + `.agents/skills/`.

## 6. Non-Goals

1. **Windows** support (never, for this program’s product targets).
2. Multi-user / org template marketplace.
3. Framework zoo (web frameworks, game engines, embedded, WASM as Core).
4. First-class **lib-only** or multi-crate **workspace** generate targets in v1.
5. Designing primarily for anonymous public consumers over **owner + agents**.
6. Unlimited MCP/skill catalog — **closed, curated** agent tooling only for v1.
7. **Claude Code–specific** surfaces as a design target (no `CLAUDE.md` / `.claude/` requirement); portable **`AGENTS.md` + `.agents/`** only.
8. Remote/plugin catalogs; merge-into-non-empty destination as default generate behavior.
9. Containers / Windows installers as Core distribution.
10. Granular coding backlog / agent task packets as program outputs.
11. Implementing the product inside this research program beyond optional evidence spikes.

## 7. Locked Constraints

| ID  | Constraint |
| --- | ---------- |
| L1  | Product shape: **hybrid** (generator + strong default Core + GitHub template surface). |
| L2  | Foundry implementation language: **Rust** (dogfood). |
| L3  | Host OS targets: **macOS and Linux only**; **never Windows**. |
| L4  | Archetypes: **CLI** primary; **TUI** optional (not every CLI). Exact archetype-vs-profile modeling is a research decision. |
| L5  | CI: **GitHub Actions** in Core. |
| L6  | Distribution: foundry install via **cargo install + GitHub release binaries** (evaluate cargo-dist at implementation); generated projects get CI in Core and an **optional distribution profile** for public CLIs. No containers/Windows installers as Core. |
| L7  | **AI-native first**: portable agent skills under `.agents/`; root **`AGENTS.md`**; MCP default none; no Claude-specific design target. |
| L8  | Prior art: go-foundry-cli and python-foundry as **balanced transfer** references — Adopt/Adapt/Reject with evidence; not governing authority. |
| L9  | Primary user model: personal tool; agents implement; open-source OK. |
| L10 | Rigor tier: **standard**. |
| L11 | Time posture: research quality over artificial calendar pressure. |
| L12 | This repo is the **research program**; product implementation is downstream of accepted revised spec + plan. |
| L13 | Core toolchain items listed in Goals are **candidates** until research accepts or revises them with REC/DEC. |

## 8. Success Criteria

1. **Fast path:** From empty directory, a short validate/plan → generate (or template) flow yields a runnable CLI (or TUI when selected) with Core tooling wired.
2. **Agent operability:** An AI coding agent can understand layout, conventions, and how to add a subcommand or TUI surface without long oral tradition.
3. **Consistency:** Generated CLI/TUI projects share Core conventions.
4. **Decision reduction:** Owner stops hand-rebuilding the same Rust project skeleton for ordinary new CLI/TUI work.
5. **Teach-as-you-go:** Defaults and project docs make *why* Core choices exist legible (short, not a textbook) so owner and agents can learn the stack from artifacts.
6. **Program completion:** Accepted revised definitive specification + revised phase/milestone plan that an implementation repo can follow.

**Failure modes:** Scaffold that still needs tribal knowledge; foundry so flexible it reintroduces setup decision fatigue; agent surface as uncurated kitchen sink; Windows or framework-zoo scope creep; unevidenced “modern” defaults that fight 2026 Rust practice or dogfooding.

## 9. Rigor Tier

- **Selected:** standard
- **Rationale:** Personal and reversible (not high-assurance), but non-trivial: hybrid generator, 2026 Rust ecosystem survey, AI-native surface, CLI/TUI flexibility, transfer from go/python foundries, and owner’s lower Rust experience. Needs full evidence ledgers, bounded spikes, synthesis, and adversarial reviews — more than focused.
- **Approval:** Approved in discovery framing (2026-08-01); confirmed in this Blueprint upon human acceptance.

## 10. Research Graph

| Stage ID | Name | Kind | Depends on | Output | Parallel group |
| -------- | ---- | ---- | ---------- | ------ | -------------- |
| discovery | Project Discovery | discovery | — | `docs/00-program-blueprint.md` | — |
| charter | Research Charter | research-charter | discovery | `docs/01-research-charter.md` | — |
| research-rust-ecosystem | Modern Rust Ecosystem & CLI/TUI Standards | foundational (focused research) | charter | `docs/reports/01-modern-rust-ecosystem.md` | G1 |
| research-ai-native | AI-Native Repository & Agent Workflow | independent (focused research) | charter | `docs/reports/02-ai-native-agent-workflow.md` | G1 |
| research-foundry-architecture | Foundry Architecture | dependent (focused research) | charter, research-rust-ecosystem, research-ai-native | `docs/reports/03-foundry-architecture.md` | — |
| synthesis | Definitive Specification Synthesis | chief-architect-synthesis | research-foundry-architecture | `docs/specifications/01-definitive-specification.md` | — |
| spec-review | Specification Adversarial Review | adversarial-review | synthesis | `docs/reviews/01-specification-adversarial-review.md` | — |
| spec-revision | Revised Definitive Specification | artifact-revision | spec-review | `docs/specifications/02-definitive-specification-revised.md` | — |
| implementation-plan | Implementation Plan | implementation-plan | spec-revision | `docs/plans/01-implementation-plan.md` | — |
| plan-review | Implementation Plan Adversarial Review | adversarial-review | implementation-plan | `docs/reviews/02-implementation-plan-adversarial-review.md` | — |
| plan-revision | Final Revised Implementation Plan | artifact-revision | plan-review | `docs/plans/02-implementation-plan-revised.md` | — |

Prompts for focused and spine stages are created **just in time** from
`program/templates/` (not pre-authored here beyond graph identity).

### Track justification

| Track | Why it exists | Why another cannot absorb it | Decisions it informs | Consumed by |
| ----- | ------------- | ---------------------------- | -------------------- | ----------- |
| research-rust-ecosystem | Core tools, layouts, CLI/TUI stacks, testing, CI, release must be evidence-backed for 2026 Rust | Architecture assumes a Core set; AI-native assumes project shape but does not pick clap vs alternatives | Core vs profile set; edition/MSRV; layout; test/CI/release conventions | Architecture report; synthesis |
| research-ai-native | Skills, agent docs, MCP/LSP posture, DoD are first-class product requirements | Ecosystem track picks crates/tools, not agent workflow; architecture wires both | Closed agent surface; `AGENTS.md` / skills layout; command surface for agents | Architecture report; synthesis |
| research-foundry-architecture | Spec → plan → generate, catalog, Core/profiles, Rust CLI shape | Needs ecosystem + AI-native outputs as inputs | Generation model; spec format; go/python transfer; module boundaries | Synthesis |

### Omitted tracks (why)

| Omitted | Why unnecessary for this program |
| ------- | -------------------------------- |
| Domain and problem | Personal tooling domain is simple; problem locked in discovery |
| User and workflow | Single primary user; agent-operator model already locked |
| Security and threat model (full) | Personal tool; secrets via fnox candidate — fold light notes into ecosystem/architecture; not a full threat program |
| Data and integration (full) | No multi-enterprise integration fabric; CLIs may use HTTP as a profile later |
| Operations / SRE scale | No multi-tenant runtime service as the product |
| Performance and scalability | Not architecture-defining for a project generator |
| Migration and compatibility | Greenfield personal foundry; no legacy user base |
| Legal / compliance (full) | Open-source licensing note in charter/spec later; not regulated product |
| Financial / market | Not a commercial viability study |
| Scientific validation | Engineering convention + bounded spikes suffice |

## 11. Stage Descriptions and Dependencies

### discovery — Project Discovery

- **Primary question:** What problem, outcome, scope, rigor, and graph should govern the program?
- **Output:** this Blueprint
- **Completion:** Framing approved; Blueprint filled; human accepts Blueprint; commit recorded

### charter — Research Charter

- **Primary question:** How will research be conducted (methods, evidence, vocabulary, quality bar)?
- **Prerequisites:** Accepted Blueprint
- **Output:** `docs/01-research-charter.md`
- **Completion:** Charter filled per contract; validated; human accepts; commit recorded

### research-rust-ecosystem — Modern Rust Ecosystem & CLI/TUI Standards

- **Kind:** foundational focused research
- **Primary question:** What tooling, crates, layouts, testing, CI, and release practices should define Core (and profiles) for Rust CLI and optional TUI projects in 2026 on macOS/Linux?
- **Scope:** rustup/toolchain/edition/MSRV; rustfmt/clippy; test runners (cargo test vs nextest); error/CLI crates (anyhow, thiserror, clap); TUI stack (ratatui + backend); just; hooks (pre-commit and/or hk); secrets (fnox); GitHub Actions; cargo-dist or equivalents; layout conventions; teach-as-you-go documentation expectations for defaults
- **Non-goals:** Designing the generator engine; full agent skill catalog; web/async/WASM/embedded zoo; Windows
- **Inputs:** Accepted Blueprint, Charter; go-foundry and python-foundry as reference only
- **Output:** `docs/reports/01-modern-rust-ecosystem.md`
- **Identifiers:** REC-001..REC-099; RSK/OQ as needed; SPK if load-bearing claims need spikes
- **Spikes:** Expected when version pins or tool choices are contested and testable
- **Replication:** Permitted, not required by default
- **Downstream:** research-foundry-architecture, synthesis

### research-ai-native — AI-Native Repository & Agent Workflow

- **Kind:** independent focused research (parallel with ecosystem after charter)
- **Primary question:** How should the foundry and Generated Projects be structured, documented, and instrumented so AI coding agents work optimally (skills, MCP, LSP, instructions, checks)?
- **Scope:** Agent instruction files; portable skills; curated MCP/LSP posture; repo boundaries; verification hooks agents can run; definition of done; transfer/re-check of python-foundry AI-native decisions for Rust trees
- **Non-goals:** Building every MCP server; multi-agent orchestration product; model training; Claude-specific design target
- **Inputs:** Accepted Blueprint, Charter; python-foundry AI-native report as transferable prior art
- **Output:** `docs/reports/02-ai-native-agent-workflow.md`
- **Identifiers:** REC-100..REC-199
- **Spikes:** Optional (e.g. agent task success on sample layout)
- **Replication:** Permitted, not required
- **Downstream:** research-foundry-architecture, synthesis

### research-foundry-architecture — Foundry Architecture

- **Kind:** dependent focused research
- **Primary question:** What architecture implements hybrid generation (spec → plan → generate), Core/profiles/catalog, CLI vs optional TUI, and AI-native surfaces for a **Rust** foundry CLI, adapting go-foundry and python-foundry where appropriate?
- **Scope:** CLI commands; project spec format; generation plan; filesystem/write semantics; catalog; archetype/profile model (CLI + optional TUI); module layout; verify tiers; GitHub template as generated artifact; evidence gates
- **Non-goals:** Full implementation; unbounded profiles; Windows
- **Inputs:** Accepted ecosystem + AI-native reports; go-foundry-cli and python-foundry architecture as transferable reference
- **Output:** `docs/reports/03-foundry-architecture.md`
- **Identifiers:** REC-200..REC-299
- **Spikes:** Expected for load-bearing generation/plan semantics if uncertain
- **Replication:** Risk-triggered
- **Downstream:** synthesis

### Fixed spine (post-research)

| Stage | Completion criteria (summary) |
| ----- | ----------------------------- |
| synthesis | Single coherent definitive specification with REQ-001..REQ-299 as needed; consumes all research reports |
| spec-review | Adversarial findings FND-001..FND-199; no silent omission of load-bearing claims |
| spec-revision | Revised spec is implementation authority candidate; dispositions findings |
| implementation-plan | Phases/milestones only; subordinate to revised spec |
| plan-review | Findings FND-200..FND-399 |
| plan-revision | Final delivery authority plan; human acceptance + commit |

## 12. Parallelism

- **Default:** sequential where dependencies exist.
- **Parallel group G1:** `research-rust-ecosystem` and `research-ai-native` may run in parallel after Charter acceptance — they do not require each other’s findings.
- **Sequential:** `research-foundry-architecture` waits for both G1 reports.
- **Justification:** Ecosystem and AI-native concerns are separable; architecture must integrate both.

## 13. Optional Replication Points

- Replication **enabled** at program level; **not required by default**.
- Recommend considering replication if a G1 or architecture report makes a **contested, load-bearing** claim (e.g. “tool X must be Core”) with weak evidence.
- Any replication requires reconciliation per `program/` contracts before synthesis consumes results.

## 14. Artifact Inventory

| Path / area | Purpose |
| ----------- | ------- |
| `docs/00-program-blueprint.md` | This governing Blueprint |
| `docs/01-research-charter.md` | Research methods and quality bar |
| `docs/prompts/` | JIT stage prompts |
| `docs/reports/` | Focused research reports |
| `docs/specifications/` | Definitive and revised specifications |
| `docs/reviews/` | Adversarial reviews |
| `docs/plans/` | Implementation plans |
| `docs/evidence/` | SPK-### spikes |
| `docs/reconciliations/` | Replication reconciliation |
| `docs/validations/` | Validation reports |
| `docs/handoffs/` | Handoff notes |
| `decisions/` | DEC-### records |
| `research-program.toml` | Operational manifest (index only) |
| `program/` | Methodology library (not project conclusions) |

## 15. Identifier Allocations

| Namespace | Range | Notes |
| --------- | ----- | ----- |
| DEC | DEC-001..DEC-999 | Decision records |
| REC | REC-001..REC-099 | Ecosystem track |
| REC | REC-100..REC-199 | AI-native track |
| REC | REC-200..REC-299 | Architecture track |
| REQ | REQ-001..REQ-299 | Specification requirements |
| FND (spec) | FND-001..FND-199 | Spec adversarial review |
| FND (plan) | FND-200..FND-399 | Plan adversarial review |
| RSK | RSK-001..RSK-999 | Risks |
| OQ | OQ-001..OQ-999 | Open questions |
| SPK | SPK-001..SPK-999 | Evidence spikes |
| PHASE | PHASE-01..PHASE-99 | Plan phases |
| MS | MS-001..MS-999 | Plan milestones |

Never reuse IDs. Rejected/superseded IDs remain reserved.

## 16. Authority and Precedence

Follow `program/contracts/authority-and-precedence.md`.

**Project-specific notes:**

1. go-foundry-cli, python-foundry, and related research artifacts are **prior art**, not governing authority for this program unless explicitly adopted via DEC or accepted research recommendation.
2. Chat history is not authority.
3. After acceptance, precedence is roughly: accepted DEC → this Blueprint → Charter → current stage prompt → revised specification → research reports → reviews → plans → `research-program.toml` (index).

## 17. Human Approval Gates

See `program/operator/approval-gates.md`. Material gates for this program include: framing (done), Blueprint, Charter, each research report, synthesis, spec review/revision, plan review/revision, formal DECs.

## 18. Fresh-Session Policy

Every **substantive** stage runs in a **fresh session** with a self-contained attachment manifest. Preparing prompts, manifests, and mechanical fixes may occur in the current session (`research-stage` skill). Do not execute multiple substantive research stages in one context.

## 19. Validation and Commit Gates

- Independent validation (`research-validate`) before acceptance.
- Validators fix mechanical issues only; no invented research.
- Humans own Git; accepting commit hashes recorded in `research-program.toml`.
- Placeholders never unlock downstream work.

## 20. Amendment Protocol

See `program/reference/amendment-protocol.md`. Material scope/rigor/graph changes require explicit human approval and Blueprint amendment; do not silently edit governing artifacts outside commissioned revision.

## 21. Completion Criteria

See `program/operator/completion-criteria.md`. Program complete when revised definitive specification and revised implementation plan are accepted as implementation and delivery authority, manifest is accurate, and no required stage remains placeholder.

## 22. Implementation Handoff Expectation

- **Implementation authority:** accepted `docs/specifications/02-definitive-specification-revised.md`
- **Delivery authority:** accepted `docs/plans/02-implementation-plan-revised.md` (subordinate to revised spec)
- Handoff targets a separate product/implementation repository (or later branch program) — not ad-hoc coding from chat.
- Final plan stops at **phases and milestones**.

## Principal uncertainties (research must resolve)

1. Exact **Core vs profile** split (including edition/MSRV, nextest, cargo-dist).
2. **TUI** as separate archetype vs profile on CLI.
3. **Spec format** and generation engine design (go-foundry parity vs Rust-idiomatic).
4. Catalog embed + render strategy in a Rust foundry binary.
5. Verify tiers on generate (fmt/clippy/test).
6. Minimal closed set of **skills / MCP / LSP** for generated CLI/TUI projects.
7. Hooks: pre-commit vs hk (or both) for Rust trees.
8. How much of go-foundry’s catalog/plan/transaction model and python-foundry’s hybrid model transfer to Rust/Cargo.

## Completion Checklist

- [x] Discovery framing approved by human
- [x] All required sections filled (not placeholder prose)
- [x] Research tracks justified; omitted tracks justified
- [x] Identifier ranges allocated
- [x] Rigor tier approved
- [x] Human accepts Blueprint
- [x] Manifest updated; accepting commit recorded
