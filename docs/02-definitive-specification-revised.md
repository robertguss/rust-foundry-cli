# Revised Definitive Specification — rust-foundry

- **Artifact type:** Revised definitive specification
- **Program:** rust-foundry
- **Status:** Accepted — implementation authority
- **Version:** 0.2
- **Created:** 2026-08-01 (synthesis baseline v0.1)
- **Last updated:** 2026-08-01
- **Revision date:** 2026-08-01
- **Baseline:** `docs/specifications/01-definitive-specification.md` (v0.1; Proposed — pending adversarial review)
- **Review:** `docs/reviews/01-specification-adversarial-review.md` (FND-001..FND-008; gate Conditional)
- **Synthesis stage accepted:** 2026-08-01 (`0e43d65…`)
- **Spec-review stage accepted:** 2026-08-01 (`6fc4ae2…`)
- **Implementation status:** Not started (research program artifact only; this file is implementation authority once stage is human-accepted)
- **Depends on:** Accepted Blueprint; Accepted Charter; accepted reports;
  accepted proposed specification; accepted adversarial review
- **Commissioning prompt:** `docs/prompts/06-specification-revision-prompt.md`
- **Requirement range:** REQ-001..REQ-160 (remaining REQ-161..REQ-299 reserved; new in revision: REQ-034, REQ-053)
- **Phase labels:** PHASE-01..PHASE-05 (high-level; not delivery authority)
- **Finding dispositions:** FND-001..FND-008 (all Accepted or Accepted with modification)

> This revised specification dispositions every adversarial finding and integrates
> accepted corrections. **Artifact status:** `Accepted — implementation authority`
> (High findings FND-001..003 resolved; no known blocking contradiction). Stage
> acceptance in `research-program.toml` remains a separate human gate. Research
> reports remain evidence; REC dispositions live in §28; FND dispositions live
> in the Finding Disposition Ledger below.

---

## 1. Artifact Metadata

| Field | Value |
| ----- | ----- |
| Program ID | rust-foundry |
| Owner | robertguss |
| Product name | rust-foundry (Foundry product) |
| Generated artifacts | Generated Projects (CLI primary; optional TUI profile) |
| Host OS | macOS + Linux only; **never Windows** |
| Foundry language | Rust (dogfood) |
| Prior art (transfer only) | go-foundry-cli; python-foundry architecture |
| Authority | This revised spec (`02-…`) is implementation authority (stage accept still human gate) |

---

## 1A. Revision Summary

This revision integrates the Conditional-gate adversarial review (FND-001..FND-008)
into a standalone definitive specification without reopening Blueprint locks,
re-picking Core crates, or adding product features beyond what findings require.

**Primary corrections:**

1. **CLI overrides** are normative: allowed flags, CLI-over-TOML precedence, and
   Construct inclusion (FND-001 → REQ-034; §11.1; §12.1).
2. **Destination emptiness and exclusive place** are testable Musts: zero-child
   emptiness predicate, admissible targets, same-FS rename preferred with
   fail-closed cross-device Default (FND-002 → REQ-051).
3. **Stage-root path jail** is a Must REQ with escape fixtures (FND-003 → REQ-053).
4. **Profile fold order** is canonical catalog order, independent of input order
   (FND-004 → REQ-063; normalize in resolve).
5. **Stage lifecycle** success cleanup / fail retain + error path (FND-005 → REQ-050).
6. **Offline wording** splits catalog/render offline from verify network needs
   (FND-006 → REQ-060, REQ-120, §15).
7. **Ghost REQ-044** reference fixed to REQ-040..043 (FND-007).
8. **Secret field denylist** operationalized (FND-008 → REQ-033).

**Status judgment:** All three High findings are **Accepted** and integrated.
Medium and Low findings are Accepted (FND-008 Accepted with modification via
explicit denylist). No Critical findings existed. No new major machinery was
introduced beyond tightening existing write/CLI/security contracts. Eligible for
**implementation authority**.

## 1B. Finding Disposition Ledger

| FND | Sev | Disposition | Integration summary |
| --- | --- | ----------- | ------------------- |
| FND-001 | High | **Accepted** | Override set `--name`/`--dest`/`--verify`; CLI wins over TOML; apply before Construct; §12.1 + **REQ-034**; REQ-040 effective inputs |
| FND-002 | High | **Accepted** | Emptiness = zero `read_dir` children; missing path + empty dir admissible; file/symlink/non-empty refuse; place algorithm in **REQ-051**; REQ-150 cases |
| FND-003 | High | **Accepted** | **REQ-053** stage-root jail; absolute/`..`/symlink escape hard-fail; fixtures in §16.2 / REQ-053 acceptance |
| FND-004 | Medium | **Accepted** | Canonical profile order `tui` → `hooks` → `secrets` → `distribution`; input order non-significant; **REQ-063** + §9.2 |
| FND-005 | Medium | **Accepted** | **REQ-050** lifecycle table: success cleans stage; fail retains + error prints stage path |
| FND-006 | Medium | **Accepted** | Catalog/render offline Must; default/strict verify may need network or warm cache; **REQ-060**, **REQ-120**, §15 |
| FND-007 | Low | **Accepted** | §2 plan schema freeze text → **REQ-040..043** (no REQ-044) |
| FND-008 | Low | **Accepted with modification** | Case-insensitive field-name denylist Must (not vague “patterns”); **REQ-033** |

No findings Rejected, Deferred, or Not applicable.

## 1C. Integrated Correction Ledger

| Area | Baseline defect | Correction |
| ---- | --------------- | ---------- |
| §11.1 / §12.1 / REQ-034 | Overrides narrative-only | Normative flags + precedence + Construct |
| REQ-040 | Equality without override order | Equality on **effective inputs after overrides** |
| REQ-051 / §9.1 / §13.4 | Soft empty MAY; undefined empty | Must emptiness predicate + place algorithm |
| REQ-053 / §14 | Path jail prose-only | Must REQ + tests |
| REQ-050 / §15 | Stage lifecycle docs-only | Must success/fail table |
| REQ-063 / §9.2 | Ordered profiles without order key | Canonical catalog order |
| REQ-060 / REQ-120 / §15 | Offline over-claim | Catalog offline ≠ verify offline |
| §2 | REQ-040..044 ghost | REQ-040..043 |
| REQ-033 | Non-operational patterns | Named denylist |
| Traceability / REC notes | Missing new REQs | REQ-034, REQ-053 rows; REC-201/203 notes updated |
| §30 handoff | Adversarial review | Implementation plan handoff |

## 1D. Preserved Strengths

- **49/49 REC disposition ledger** complete (§28); Merged targets remain real.
- **Pure Construct** plan-as-contract (INV-3 / REQ-040) retained and strengthened by override normalization.
- **Refuse non-empty default** (INV-2); no merge-into-non-empty default introduced.
- **TUI generate-time inclusion** and zero pure-CLI TUI leakage (REQ-004, REQ-065, REQ-103).
- **Skill surface separation** (research / product / generated) and closed product skill ids.
- **Closed embedded catalog** (INV-1 / REQ-060); no remote marketplace.
- **No Windows / Claude Core / MCP default-on** (L3, L7; REQ-003, REQ-006, REQ-007).
- Hybrid L1 shape, Rust dogfood L2, GHA Core L5, CLI primary L4 unchanged.
- Prefer simplification: no force-overwrite flag, no new profile kinds, no second engine.

## 2. Executive Decision Summary

**rust-foundry** is a **planner-led Rust CLI** hybrid foundry that:

1. Reads a **versioned TOML Project Spec** (`schema = 1`)
2. Resolves **archetype `cli` only** + a **closed ordered profile set**
3. Builds an **immutable Generation Plan** (text + JSON; content digests)
4. Renders into a **sibling staging directory**
5. Runs **tiered verification** aligned to ecosystem command surface (default: primary gate)
6. **Exclusively places** the finished tree (no merge; refuse non-empty by default)
7. Emits **locked Core tooling** and **portable AI-native surfaces** as invariants

| Decision area | Freeze |
| ------------- | ------ |
| Lifecycle | `validate` → `plan` → `generate` (+ `catalog`, `version`) |
| Spec | TOML Project Spec; explicit `--spec`; non-interactive first |
| Plan | Plan-as-contract; pure Construct shared by validate/plan/generate |
| Writes | Stage → verify → exclusive place; refuse non-empty default |
| Catalog | Closed; embedded in binary; custom planner-led renderer |
| TUI | Optional profile `tui`; **generate-time file inclusion** |
| Stack | clap derive, rustfmt/clippy, cargo test, edition 2024, GHA Core |
| AI-native | Root `AGENTS.md` + `.agents/skills/`; MCP default none; no Claude target |
| Foundry product skills | `plan-generate`, `catalog-inspect`, `foundry-quality-gates` |
| GH template | Catalog SoT; CI-generated pure-CLI snapshot |
| Prior art | Adopt lifecycle/plan/place/catalog SoT; Adapt verify/embed; Reject remote catalogs, Claude, merge-update v1 |

**Synthesis judgment calls (provisional defaults):**

| Topic | Decision | Residual |
| ----- | -------- | -------- |
| `rust-version` stamp | Emit floor `1.85` in Generated Projects; do not auto-stamp “today’s stable minor” | OQ-100 closed as provisional Default |
| Default CLI logging | Include `tracing` + `tracing-subscriber` for default full CLI; omit only documented ultra-minimal samples | OQ-102 closed |
| nextest in CI templates | Default CI uses `cargo test`; nextest documented optional / strict verify | OQ-101 closed |
| macOS CI | Linux Required; macOS Recommended when shipping mac bins or `distribution` profile | OQ-103 closed |
| Plan schema | Freeze elements in REQ-040..043; exact JSON field names may refine with SPK-100 | OQ-200 |

---

## 3. Authority and Intended Use

### Authority ladder (for this revised specification)

1. Accepted `DEC-###` (none at revision time; check `decisions/`)
2. Accepted Program Blueprint locks and non-goals
3. Accepted Research Charter methodology
4. **This revised definitive specification** (implementation authority when stage is human-accepted; artifact status already `Accepted — implementation authority`)
5. Accepted adversarial review as historical finding source (dispositions integrated; not a parallel law)
6. Baseline proposed specification `01-…` (superseded by this revision for implementation)
7. Accepted focused research reports (evidence + REC dispositions in §28)
8. `research-program.toml` (index only)

### Intended use

| Audience | Use |
| -------- | --- |
| Implementation planners | Derive PHASE/MS delivery plan subordinate to this revised spec |
| Implementers (agents + owner) | Build Foundry product and catalog content against normative REQs |
| Validators / future reviewers | Risk-triggered re-review only if major new machinery appears |
| Not for | Reopening closed RECs without DEC / new evidence; product code in research repo |

### Not authority

Chat history; dual-stream raw dumps under `scripts/exa-output/`; placeholder prior specs; popularity metrics; the baseline `01-…` file (historical; superseded for implementation).

---

## 4. Problem and Product Definition

### Problem

Starting Rust CLI/TUI work repeatedly re-establishes toolchain pins, layout, quality gates, hooks/secrets, CI, and agent-operable structure. That produces inconsistent bases and forces re-explaining conventions to AI coding agents (primary implementers). The owner has less Rust depth than Go/Python, so silent fashionable defaults are high risk. One-shot scaffolds do not encode a closed Core, dry-run generation, or portable agent surface.

### Product

**Foundry product:** a Rust CLI (`foundry`) that generates complete single-crate CLI application repositories from a declarative Project Spec via validate → plan → generate.

**Generated Projects:** runnable CLI apps with Core tooling, optional capability profiles (`tui`, `hooks`, `secrets`, `distribution`), and portable agent surface.

**Hybrid surface:** generator CLI + strong Core + GitHub template (catalog snapshot of default pure CLI).

### Users

| Role | Relationship |
| ---- | ------------ |
| Primary operator | Owner (robertguss) |
| Primary implementers | AI coding agents |
| Secondary | Open-source readers (not v1 design focus) |

---

## 5. Goals and Non-Goals

### Goals

1. Hybrid foundry: generator + strong Core + GitHub template surface (L1).
2. Evidence-backed 2026 Rust Core for Generated Projects.
3. CLI primary; TUI optional (not forced).
4. AI-native first: portable skills and docs; closed curated agent surface.
5. Dogfood: implement Foundry product in Rust.
6. Balanced transfer from go/python-foundry (Adopt/Adapt/Reject).
7. Teach-as-you-go legibility for non-obvious defaults.
8. Spec and plan sufficient for later implementation (this document + plan stages).

### Non-goals (v1)

1. Windows support (never for this product line).
2. Multi-user / org template marketplace.
3. Framework zoo (web, game, embedded, WASM as Core).
4. First-class lib-only or multi-crate workspace generate.
5. Claude-specific design targets (`CLAUDE.md`, `.claude/` as Core).
6. Remote/plugin catalogs; merge-into-non-empty as default generate.
7. Containers / Windows installers as Core distribution.
8. Unlimited MCP/skill catalogs.
9. Granular coding backlog as research program output.
10. Product implementation inside the research repo beyond optional spikes.

---

## 6. Locked Decisions and Invariants

| ID | Invariant | Normative effect |
| -- | --------- | ---------------- |
| L1 | Hybrid foundry | Generator + Core + GH template all required product facets |
| L2 | Foundry in Rust | Product implementation language MUST be Rust |
| L3 | macOS + Linux only | MUST NOT support Windows hosts/targets/installers |
| L4 | CLI primary; TUI optional | Pure CLI MUST NOT receive TUI deps/files/skills |
| L5 | GHA in Core | Generated Projects MUST emit GitHub Actions CI |
| L6 | Distribution optional | cargo-dist only via profile |
| L7 | Portable AI surface | `AGENTS.md` + `.agents/`; MCP default none; no Claude target |
| L8 | Balanced prior art | go/python-foundry are transfer only, not sole proof |
| L9 | Agents primary | Surfaces MUST be agent-operable without oral tradition |
| L12 | Research ≠ product | This research repo is not the Foundry product implementation |
| INV-1 | Closed catalogs | Core tools, profiles, and skills are closed sets |
| INV-2 | Fail-closed writes | Default generate refuses non-empty destinations |
| INV-3 | Plan-as-contract | Same pure Construct for validate/plan/generate inputs |
| INV-4 | Catalog SoT | GH template is snapshot; not dual-edited authority |

---

## 7. Final Technology Stack

### 7.1 Foundry product (implementation)

| Layer | Choice | Class |
| ----- | ------ | ----- |
| Language | Rust | Required |
| CLI framework | clap derive | Required (dogfood) |
| Layout | Thin `main` + `lib`; modules per §10 | Default |
| Spec parse | TOML (`toml` crate or equivalent) | Required |
| Catalog embed | `include_dir!` / rust-embed equivalent | Required model |
| Hosts | macOS, Linux | Required |
| Install | cargo install + GH release binaries | Default |
| Completions | bash/zsh/fish only | Default |
| Product skills | plan-generate, catalog-inspect, foundry-quality-gates | Required ids |

### 7.2 Generated Projects (Core + profiles)

| Layer | Choice | Class |
| ----- | ------ | ----- |
| Toolchain | `rust-toolchain.toml` channel stable; components rustfmt, clippy; profile minimal | Required |
| Edition | 2024 | Required |
| rust-version | Floor **1.85** stamped in Cargo.toml | Default |
| Layout | Single crate; `src/main.rs` + `src/lib.rs` | Default |
| Format/lint | rustfmt; clippy; CI `-D warnings` | Required |
| Test | `cargo test` Required; nextest Default optional | Required/Default |
| Errors | anyhow Default for app CLI; thiserror when typed lib errors | Default |
| CLI | clap + derive | Required |
| TUI | ratatui + crossterm only if profile `tui` | Optional profile |
| Tasks | just + justfile (`fmt`, `lint`, `test`, `ci`/`check`) | Default |
| Hooks | pre-commit Default when profile `hooks`; hk Watchlist | Optional profile |
| Secrets | fnox + age scaffolding when profile `secrets`; gitignore hygiene always | Optional / Required hygiene |
| CI | GitHub Actions; Linux Required; no Windows | Required |
| Distribution | cargo-dist when profile `distribution`; Linux+macOS targets only | Optional profile |
| Logging | tracing + tracing-subscriber for default full CLI | Default |
| Agent surface | AGENTS.md + `.agents/skills/*` | Required |
| MCP | none by default | Required posture |
| Claude files | never emit | Required absence |

---

## 8. System Context

```text
┌──────────────────┐     Project Spec (TOML)      ┌─────────────────────┐
│ Owner / Agents   │ ───────────────────────────► │ foundry CLI         │
│ (macOS / Linux)  │ ◄── plan text/JSON ───────── │ validate/plan/gen   │
└──────────────────┘                              │ catalog / version   │
                                                  └──────────┬──────────┘
                                                             │ embed
                                                             ▼
                                                  ┌─────────────────────┐
                                                  │ Closed catalog      │
                                                  │ core + cli + profiles│
                                                  └──────────┬──────────┘
                                                             │ generate
                                                             ▼
                                                  ┌─────────────────────┐
                                                  │ Generated Project   │
                                                  │ (empty dest place)  │
                                                  └──────────┬──────────┘
                                                             │
                         ┌───────────────────────────────────┼────────────────┐
                         ▼                                   ▼                ▼
                  cargo/just gates                    GitHub Actions      Agents via
                  + optional profiles                 (Core CI)           AGENTS.md/skills

Catalog SoT ──CI regen──► GitHub template repo (pure-CLI snapshot; onboarding only)
```

External systems: rustup/cargo toolchain on host; optional just; GitHub for CI/template; no remote catalog fetch in v1 default path.

---

## 9. Architecture

### 9.1 Generation pipeline

| Stage | Inputs | Outputs | Writes dest? | Failure / lifecycle |
| ----- | ------ | ------- | ------------ | ------------------- |
| Parse/validate spec | `--spec` + CLI overrides | Normalized **effective** inputs | No | exit ≠ 0 |
| Resolve | Effective inputs + catalog | Archetype + **canonically ordered** profiles | No | exit ≠ 0; unknown profile hard fail |
| Construct plan | Resolve result | Immutable plan + digests | No | exit ≠ 0; path jail fail (REQ-053) |
| Stage render | Plan | Sibling staging directory | No (dest untouched) | dest clean; stage retained on fail |
| Verify (tier) | Staged tree | pass/fail | No place on fail (default tier) | exit ≠ 0; stage retained; error prints stage path |
| Exclusive place | Staged tree | Destination project | Yes | fail if inadmissible dest; on success stage cleaned |

**Commands:**

| Command | Writes? | Role |
| ------- | ------- | ---- |
| `foundry validate --spec PATH` [overrides] | No | Pure pipeline success/fail on effective inputs |
| `foundry plan --spec PATH` [overrides] | No | Emit Generation Plan (text default; JSON mode) |
| `foundry generate --spec PATH` [overrides] | Yes | Construct → stage → verify → place |
| `foundry catalog list\|show` | No | Inspect closed catalog |
| `foundry version` | No | Version + catalog digest |

Overrides (`--name`, `--dest`, `--verify`) per §12.1 and REQ-034 apply to validate/plan/generate.

### 9.2 Composition model

Deterministic left-fold: `core` → `archetype:cli` → **canonically ordered** profiles from closed set.

**Canonical profile fold order (independent of input array order):**
`tui` → `hooks` → `secrets` → `distribution`.

Only selected profiles participate; membership is a set. Two specs that differ only
in profile list permutation MUST resolve to the same ordered composition and the
same planned file set (REQ-063).

### 9.3 TUI mechanism

**Generate-time file inclusion** when `tui` ∈ profiles. When absent: no TUI sources, deps, or skills. Feature-flag-only TUI and dual full template flavors are rejected as primary mechanisms.

### 9.4 Engine

Custom planner-led renderer. **MUST NOT** shell to cargo-generate or Copier as the product engine.

---

## 10. Components and Boundaries

### 10.1 Foundry product modules (Default: one crate)

| Module | Responsibility | Purity |
| ------ | -------------- | ------ |
| `cli` | clap wiring only | I/O boundary |
| `spec` | Parse + validate TOML | Pure |
| `catalog` | Load embedded catalog, digests | Pure load |
| `resolve` | Archetype/profile resolution | Pure |
| `plan` | Construct immutable plan | Pure |
| `render` | Templates → bytes | Pure given catalog |
| `fsx` | Stage + exclusive place | FS |
| `generate` | Lifecycle orchestration | FS + verify |
| `verify` | Tiered cargo/just runners | Process |
| `report` | Text/JSON encoding | Pure |

Domain logic MUST NOT live only in `cli`. Optional later split to `foundry-core` + `foundry-cli` if a second consumer appears. Many micro-crates rejected for v1.

### 10.2 Repo surface separation

| Repo kind | Skills | Commands |
| --------- | ------ | -------- |
| Research program (this repo) | research-* skills | `just status`, `just check` |
| Foundry product | plan-generate, catalog-inspect, foundry-quality-gates | foundry CLI + dogfood gates |
| Generated Project | quality-gates, add-subcommand (+ add-tui-screen if tui) | REC-016 / §12 command surface |

---

## 11. Data Model

### 11.1 Project Spec (TOML, schema = 1)

| Field | Required? | Validation |
| ----- | --------- | ---------- |
| `schema` | Required | Supported integer; v1 → `1`; else hard fail |
| `name` | Required | Non-empty; crate-name-safe recommended |
| `description` | Optional | Free text |
| `archetype` | Required | Exactly `cli` in v1 |
| `destination` | Required | Path; basename SHOULD match `name` |
| `profiles` | Required key; may be `[]` | Each id ∈ closed catalog |
| `verify` | Optional | `none` \| `default` \| `strict` |
| unknown keys | — | **Hard fail** |
| secrets | Forbidden | No secret material |

CLI: explicit `--spec PATH` or `--spec -` (stdin).

**CLI field overrides (normative — REQ-034):**

| Flag | Overrides TOML field | Commands |
| ---- | -------------------- | -------- |
| `--name NAME` | `name` | validate, plan, generate |
| `--dest PATH` | `destination` | validate, plan, generate |
| `--verify none\|default\|strict` | `verify` | validate, plan, generate |

**Precedence:** when a CLI override flag is present, it **wins** over the corresponding TOML field. Overrides apply when building **normalized effective inputs before Construct**. validate, plan, and generate with the same effective inputs MUST produce equal planned file sets (REQ-040). Profile membership has **no** CLI override in v1 (profiles only via TOML); profile **order** is canonical (REQ-063), not input order.

### 11.2 Generation Plan (elements)

| Element | Purpose |
| ------- | ------- |
| foundry version + catalog digest | Reproducibility |
| validated/normalized spec | Intent record |
| resolved archetype + ordered profiles | Composition |
| planned files (path, mode, content digest) | Exact emit set |
| dependency / Cargo.toml deltas | Dep set |
| AI-native paths planned | Agent surface check |
| verify mode | Post-stage policy |
| destination policy decision | missing / empty-admissible / refuse (predicate REQ-051) |
| `plan_sha256` | Integrity |
| warnings | Non-binding notes |

Emit text by default; JSON via `--format json` / `--out`. Exact JSON field names MAY be refined under SPK-100 without changing element set (OQ-200).

### 11.3 Catalog units

| Unit id | Kind | Always? |
| ------- | ---- | ------- |
| `core` | core | Always |
| `cli` | archetype | Exactly one (v1 only `cli`) |
| `tui` | profile | Optional |
| `hooks` | profile | Optional |
| `secrets` | profile | Optional |
| `distribution` | profile | Optional |

Catalog content digest bound to foundry release.

---

## 12. Interfaces and Integrations

### 12.1 Foundry CLI (human/agent)

| Intent | Command |
| ------ | ------- |
| Validate | `foundry validate --spec PATH` [`--name NAME`] [`--dest PATH`] [`--verify none\|default\|strict`] |
| Plan | `foundry plan --spec PATH` [`--name NAME`] [`--dest PATH`] [`--verify none\|default\|strict`] [`--format json`] [`--out FILE`] |
| Generate | `foundry generate --spec PATH` [`--name NAME`] [`--dest PATH`] [`--verify none\|default\|strict`] |
| List catalog | `foundry catalog list` |
| Show unit | `foundry catalog show <id>` |
| Version | `foundry version` |

Override flags are part of the **command surface** (REQ-034), not implementation-private. Global flags (indicative): `--format text|json`, `-v/--verbose`. No `update`/`recopy`/`doctor` in v1. Avoid overloaded `init` that means generate; sample-spec command only if later needed.

### 12.2 Generated Project command surface

| Intent | Primary | Fallback |
| ------ | ------- | -------- |
| Format | `just fmt` | `cargo fmt --all` |
| Format check | `just fmt-check` | `cargo fmt --all -- --check` |
| Lint | `just lint` | `cargo clippy --all-targets -- -D warnings` |
| Test | `just test` | `cargo test` or `cargo nextest run` (+ `cargo test --doc` if nextest) |
| Full gate | `just ci` / `just check` | fmt-check + lint + test |
| Run | `just run` / `cargo run` | `cargo run -- …` |

### 12.3 GitHub

- Core CI workflow emitted for Generated Projects.
- Foundry product CI regenerates GH template snapshot from catalog.
- No Windows runners.

### 12.4 MCP / editor

- Default MCP config: **none**.
- rust-analyzer: optional environment assumption; **not** DoD.

---

## 13. User Workflows

### 13.1 Happy path (agent/owner)

1. Author Project Spec TOML (or reuse sample).
2. `foundry validate --spec ./project.toml`
3. `foundry plan --spec ./project.toml` (review text/JSON)
4. Ensure destination is **missing** or an **empty directory** (zero children; REQ-051).
5. `foundry generate --spec ./project.toml` (optional `--name` / `--dest` / `--verify` overrides)
6. `cd` dest; `just ci` / continue development with skills.

### 13.2 Profiled TUI path

Same as happy path with `profiles = ["tui"]` (and optional others). Plan must list TUI paths/deps; placed tree must include them.

### 13.3 GitHub template path

Use GitHub “Use this template” for pure-CLI onboarding. For profiles/plan/verify discipline, use Foundry generate.

### 13.4 Refuse path

`foundry generate` to an **inadmissible** destination → exit ≠ 0; no merge; destination unchanged.

Inadmissible includes: existing non-empty directory (any child, including `.git`), existing file at path, symlink destination (v1), or place algorithm failure (REQ-051). Error output SHOULD state the emptiness/admissibility reason.

---

## 14. Security and Privacy

| Rule | Requirement | REQ |
| ---- | ----------- | --- |
| Secrets in Project Spec | MUST NOT; hard fail denylisted field names (case-insensitive) | REQ-033 |
| Secrets in Generated trees | gitignore hygiene Required; no plaintext secrets committed | REQ-067, REQ-107 |
| Path escape / render injection | Every planned/rendered path MUST resolve strictly under stage root after normalization; absolute paths, `..` escape, and symlink escape MUST hard-fail plan or render | REQ-053 |
| Agent boundaries | AGENTS.md MUST state secrets, no Windows invention, no Claude-required surfaces, no MCP required | REQ-006, REQ-107 |
| MCP | MUST NOT emit default MCP kitchen sink | REQ-007 |
| Distribution | Linux+macOS only; never Windows installers as Core | REQ-003, REQ-068 |
| Personal tool threat model | Fail-closed writes; no multi-tenant isolation claims | REQ-050, REQ-051 |

---

## 15. Reliability and Operations

| Concern | Policy |
| ------- | ------ |
| Partial generate | Stage-first; dest untouched until place (REQ-050) |
| Verify failure | Default tier: do not place; stage retained; error prints stage path (REQ-050) |
| Place success | Destination is complete tree; stage cleaned (REQ-050) |
| Stage leftover on failure | Retained for inspect; path in error output (RSK-112; REQ-050) |
| Catalog drift | Template regen CI; catalog digest in `foundry version` |
| Offline catalog/render | Embedded catalog; parse/resolve/construct/render MUST work without network or remote plugin fetch (REQ-060) |
| Offline vs verify | Default/strict verify MAY require network or warm cargo cache for Generated Project deps; not an offline guarantee (REQ-120) |
| Host missing just | Document cargo fallbacks for verify |

---

## 16. Testing and Verification

### 16.1 Verify tiers (on generate)

| Tier | When | Commands |
| ---- | ---- | -------- |
| `none` | Explicit opt-out | Document only; still emit justfile/CI |
| `default` | Generate default | Primary gate: `just ci`/`just check` **or** cargo fallbacks (fmt-check + clippy -D warnings + cargo test) |
| `strict` | Opt-in | default + nextest if installed (+ doctest caveat) |

Run on **staged tree before place**. Do **not** require cargo-deny, Miri, or shear as Core verify.

### 16.2 Foundry product tests (implementation expectation)

- Golden plan JSON for minimal CLI and TUI profile (SPK-100).
- Override equality: plan/generate Construct match with same `--dest`/`--name`/`--verify` (REQ-034/040).
- Exclusive place / emptiness matrix: missing, empty dir, non-empty (incl. `.git`), file-at-path, symlink (SPK-101; REQ-051/150).
- Stage lifecycle: fail retains stage + path in stderr; success cleans stage (REQ-050).
- Path jail fixtures: `..`, absolute path, symlink escape hard-fail (REQ-053).
- Profile permutation invariance: `["hooks","tui"]` vs `["tui","hooks"]` equal digests (REQ-063).
- Emit matrix forbidden-path tests (SPK-102).
- Default verify tier smoke (SPK-103).

### 16.3 Acceptance scenarios (product ready)

See REQ-150..155 and REC-215 dispositions.

---

## 17. CI and Release

### Generated Projects

- Required workflow: fmt + clippy (`-D warnings`) + test on Linux.
- macOS Recommended when claiming mac support or `distribution` profile.
- Windows jobs MUST NOT appear.

### Foundry product

- Dogfood same quality gates.
- Release: cargo install path + GH binaries (cargo-dist evaluation at implementation).
- Release process MUST include catalog digest and template snapshot regen.

---

## 18. Migration

**N/A for v1.** Greenfield personal foundry; no merge/update of existing projects; no migration of foreign scaffolds. Future update/recopy is out of v1 (Rejected / Deferred).

---

## 19. Performance Expectations

| Operation | Expectation |
| --------- | ----------- |
| validate / plan | Interactive-agent latency; pure; no dest I/O |
| generate default verify | Dominated by cargo fmt/clippy/test of small template; acceptable minutes not hours |
| Binary size | Embed catalog; acceptable growth for personal tool |
| No SLA | Personal tool; no multi-tenant latency SLOs |

---

## 20. Internal Contracts

1. **Pure Construct:** Given identical normalized inputs + catalog digest, plan bytes (semantic content) for validate/plan/generate MUST match.
2. **Emit matrix contract:** Composition is left-fold of catalog units; pure CLI forbids TUI units.
3. **Surface separation:** Foundry product skills MUST NEVER appear in Generated Project emit.
4. **Command surface contract:** AGENTS.md / skills / justfile / CI MUST name the same primary gate strings.
5. **SoT contract:** Catalog content is authority; GH template is derived.

---

## 21. Dependency Bill of Materials

### Foundry product (indicative)

clap; toml; serde/serde_json; embed crate; anyhow/thiserror as appropriate; standard cargo test tooling.

### Generated Core (indicative Required/Default)

clap (derive); anyhow; (thiserror when needed); rustfmt/clippy via toolchain; just (host); tracing + tracing-subscriber (default full CLI).

### Profile-conditional

ratatui, crossterm (`tui`); pre-commit (`hooks`); fnox/age scaffolding (`secrets`); cargo-dist (`distribution`).

### Licensing posture

Prefer permissive (MIT/Apache-2.0 dual or equivalent) for Core deps at pin time (REC-019).

Exact version pins are implementation-time with catalog pin file; not frozen as magic numbers here.

---

## 22. Normative Requirements

Requirement template fields follow `program/templates/requirement.md`. Priority: **Must** | **Should** | **May**.

### 22.1 Product identity and locks

#### REQ-001 — Hybrid product shape

- **Priority:** Must
- **Applies to:** Foundry product + Generated Projects + template surface
- **Implementation phase:** PHASE-01
- **Source decisions:** L1; REC-210; REC-213
- **Verification:** Inspection of product surfaces and docs
- **Risk linkage:** None

##### Requirement

The product MUST provide all three hybrid facets: (1) Foundry generator CLI, (2) strong default Core for Generated Projects, (3) GitHub template surface derived from the catalog. Template-only without generator MUST NOT be the sole product.

##### Rationale

Blueprint L1 hybrid model.

##### Acceptance Evidence

README / architecture docs list all three facets; template regen from catalog exists.

##### Exceptions

None.

#### REQ-002 — Foundry implemented in Rust

- **Priority:** Must · **Applies to:** Foundry product · **Phase:** PHASE-01 · **Sources:** L2 · **Verification:** Cargo project language · **Risk:** None

##### Requirement

The Foundry product MUST be implemented in Rust.

##### Rationale

Dogfood (L2).

##### Acceptance Evidence

Product crate(s) are Rust.

##### Exceptions

None.

#### REQ-003 — Host OS macOS and Linux only

- **Priority:** Must · **Applies to:** Foundry product + Generated Projects · **Phase:** PHASE-01 · **Sources:** L3 · **Verification:** CI matrix inspection; no Windows jobs · **Risk:** RSK-007

##### Requirement

Foundry and Generated Project Core MUST target macOS and Linux only. They MUST NOT support Windows hosts, Windows CI runners, or Windows installers as Core.

##### Rationale

L3 never Windows.

##### Acceptance Evidence

No windows-* workflows; docs state hosts.

##### Exceptions

None.

#### REQ-004 — CLI primary; TUI optional

- **Priority:** Must · **Applies to:** Generation model · **Phase:** PHASE-02 · **Sources:** L4; REC-004; REC-206 · **Verification:** SPK-102 matrix · **Risk:** RSK-003, RSK-103

##### Requirement

CLI MUST be the only v1 archetype. TUI MUST be an optional capability profile. Pure CLI generates MUST NOT include TUI dependencies, TUI source trees, or TUI skills.

##### Rationale

L4; pure CLI cleanliness.

##### Acceptance Evidence

Forbidden-path tests on pure CLI plan/tree.

##### Exceptions

None.

#### REQ-005 — Closed curated sets

- **Priority:** Must · **Applies to:** Catalog, profiles, skills, MCP · **Phase:** PHASE-02 · **Sources:** L7; INV-1; REC-015; REC-103 · **Verification:** Catalog unit list equality tests · **Risk:** RSK-104, RSK-050

##### Requirement

Profiles, Core membership, and skill catalogs MUST be closed sets. Membership changes require REC/DEC-level change, not open plugins.

##### Rationale

Decision fatigue and kitchen sinks forbidden.

##### Acceptance Evidence

Catalog list matches declared closed set.

##### Exceptions

Maintainer-only external catalog override for development (not end-user marketplace).

#### REQ-006 — No Claude-specific design target

- **Priority:** Must · **Applies to:** All emits · **Phase:** PHASE-04 · **Sources:** L7; REC-100; REC-211 · **Verification:** Forbidden path tests · **Risk:** RSK-053, RSK-106

##### Requirement

Core emit MUST NOT require or generate `CLAUDE.md`, `.claude/`, or Claude-specific required surfaces.

##### Rationale

Portable standards only (L7).

##### Acceptance Evidence

CI/template tests assert absence.

##### Exceptions

None.

#### REQ-007 — MCP default none

- **Priority:** Must · **Applies to:** Generated Projects + Foundry product defaults · **Phase:** PHASE-04 · **Sources:** L7; REC-106; REC-211 · **Verification:** Emit matrix · **Risk:** RSK-051

##### Requirement

Default generate and product templates MUST NOT commit MCP server kitchen-sink configuration. MCP is opt-in only with explicit owner need.

##### Rationale

L7 closed agent tooling.

##### Acceptance Evidence

No default `.mcp.json` / equivalent Core emit.

##### Exceptions

Explicit future DEC for a single closed optional tool with justification.

#### REQ-008 — Agents as primary implementers

- **Priority:** Must · **Applies to:** Docs and skills · **Phase:** PHASE-04 · **Sources:** L9; REC-102; REC-108 · **Verification:** Operability scenarios · **Risk:** None

##### Requirement

Generated and Foundry surfaces MUST document commands, DoD, and skills so agents can operate without oral tradition.

##### Rationale

L9.

##### Acceptance Evidence

AGENTS.md + skills present; scenarios in REQ-150+.

##### Exceptions

None.

#### REQ-009 — Single-crate Generated Projects v1

- **Priority:** Must · **Applies to:** Generated Projects · **Phase:** PHASE-02 · **Sources:** Blueprint non-goals; REC-003 · **Verification:** Layout emit · **Risk:** None

##### Requirement

v1 generate MUST produce a single-crate application layout. Multi-crate workspace and lib-only first-class targets MUST NOT be supported.

##### Rationale

Scope control.

##### Acceptance Evidence

Sample generates are single package.

##### Exceptions

None.

#### REQ-010 — Research repo is not product implementation

- **Priority:** Must · **Applies to:** Program governance · **Phase:** N/A · **Sources:** L12 · **Verification:** Scope inspection · **Risk:** None

##### Requirement

This research program repository MUST NOT be treated as the Foundry product implementation beyond optional evidence spikes.

##### Rationale

L12.

##### Acceptance Evidence

Implementation occurs in product repo/branch after revised plan.

##### Exceptions

Bounded spikes only.

---

### 22.2 Foundry CLI and pipeline

#### REQ-020 — Lifecycle commands

- **Priority:** Must · **Applies to:** Foundry CLI · **Phase:** PHASE-01 · **Sources:** REC-200; REC-207 · **Verification:** CLI help + integration tests · **Risk:** None

##### Requirement

Foundry MUST implement: `validate`, `plan`, `generate`, `catalog list`, `catalog show`, and `version` as specified in §12.1. `generate` MUST be the only command that places a project tree.

##### Rationale

Agent-scriptable lifecycle.

##### Acceptance Evidence

Command tests; no silent alternate verbs required.

##### Exceptions

None for v1. `update`/`recopy`/`doctor` out of v1.

#### REQ-021 — Non-interactive first

- **Priority:** Must · **Applies to:** Foundry CLI · **Phase:** PHASE-01 · **Sources:** REC-200; REC-201 · **Verification:** Tests without TTY prompts · **Risk:** None

##### Requirement

v1 validate/plan/generate MUST NOT require interactive prompts.

##### Rationale

Agent-primary operators.

##### Acceptance Evidence

CI runs non-interactive.

##### Exceptions

None.

#### REQ-022 — validate and plan write-free

- **Priority:** Must · **Applies to:** `validate`, `plan` · **Phase:** PHASE-01 · **Sources:** REC-200 · **Verification:** FS sandbox tests · **Risk:** RSK-100

##### Requirement

`validate` and `plan` MUST NOT mutate the destination project tree and MUST NOT require network by default.

##### Rationale

Safe preview.

##### Acceptance Evidence

Tests assert no dest writes.

##### Exceptions

Writing plan to `--out` path is allowed and is not destination place.

#### REQ-023 — Exit codes

- **Priority:** Must · **Applies to:** Foundry CLI · **Phase:** PHASE-01 · **Sources:** REC-200 · **Verification:** CLI tests · **Risk:** None

##### Requirement

Successful validate/plan/generate MUST exit 0. Spec errors, resolve errors, verify failure (default tier), and refuse-non-empty MUST exit non-zero.

##### Rationale

Agent scripting.

##### Acceptance Evidence

Table-driven exit code tests.

##### Exceptions

None.

#### REQ-024 — clap derive for Foundry CLI

- **Priority:** Must · **Applies to:** Foundry product · **Phase:** PHASE-01 · **Sources:** REC-008; REC-207 · **Verification:** Dependency + derive usage · **Risk:** None

##### Requirement

Foundry CLI MUST use clap with derive API (dogfood).

##### Rationale

Core CLI standard.

##### Acceptance Evidence

Cargo.toml + source inspection.

##### Exceptions

None.

#### REQ-025 — version reports catalog digest

- **Priority:** Must · **Applies to:** `foundry version` · **Phase:** PHASE-02 · **Sources:** REC-204; REC-207 · **Verification:** CLI output test · **Risk:** RSK-102

##### Requirement

`foundry version` MUST report foundry version and embedded catalog content digest.

##### Rationale

Reproducibility and drift detection.

##### Acceptance Evidence

Output contains both fields.

##### Exceptions

None.

---

### 22.3 Project Spec, plan, writes

#### REQ-030 — TOML Project Spec schema 1

- **Priority:** Must · **Applies to:** Spec input · **Phase:** PHASE-01 · **Sources:** REC-201 · **Verification:** Parser tests · **Risk:** RSK-110

##### Requirement

Project intent MUST be a versioned TOML Project Spec with `schema = 1` fields per §11.1. Unsupported schema, unknown keys, unknown profiles, and missing required fields MUST hard fail.

##### Rationale

Replayable, Cargo-native intent.

##### Acceptance Evidence

Invalid-spec suite.

##### Exceptions

None.

#### REQ-031 — Explicit --spec

- **Priority:** Must · **Applies to:** validate/plan/generate · **Phase:** PHASE-01 · **Sources:** REC-201 · **Verification:** CLI tests · **Risk:** None

##### Requirement

Commands MUST accept explicit `--spec PATH` and MUST support `--spec -` for stdin.

##### Rationale

Agent piping and clarity.

##### Acceptance Evidence

CLI tests.

##### Exceptions

A single positional PATH MAY be accepted if equivalent to `--spec` and documented.

#### REQ-032 — Archetype cli only

- **Priority:** Must · **Applies to:** Spec resolve · **Phase:** PHASE-01 · **Sources:** REC-201; REC-004 · **Verification:** Resolve tests · **Risk:** None

##### Requirement

v1 `archetype` MUST be exactly `cli`. Other archetypes MUST hard fail.

##### Rationale

Blueprint CLI focus.

##### Acceptance Evidence

Reject matrix.

##### Exceptions

Future archetypes require Blueprint amendment + schema bump.

#### REQ-033 — Secrets forbidden in spec

- **Priority:** Must · **Applies to:** Project Spec · **Phase:** PHASE-01 · **Sources:** REC-201; REC-012 · **Verification:** Parser tests · **Risk:** RSK-110

##### Requirement

Project Spec MUST NOT carry secret material (values or secret-bearing keys). Implementation MUST hard-fail when any top-level or nested **field name** matches the denylist (case-insensitive):

`password`, `secret`, `token`, `api_key`, `private_key`, `access_key`, `client_secret`

Implementation MUST document the denylist. Full secret-content scanning is out of v1 scope.

##### Rationale

Agent/doc leakage risk; operational testability (FND-008).

##### Acceptance Evidence

Parser tests for each denylist name (various casing); docs list denylist.

##### Exceptions

None. Expanding denylist is allowed without schema bump if documented and tested.


#### REQ-034 — CLI field overrides and Construct inclusion

- **Priority:** Must · **Applies to:** validate/plan/generate · **Phase:** PHASE-01 · **Sources:** REC-201; FND-001 · **Verification:** CLI equality tests · **Risk:** RSK-101

##### Requirement

validate, plan, and generate MUST accept optional CLI overrides `--name`, `--dest`, and `--verify` as specified in §12.1. When present, a CLI flag MUST override the corresponding Project Spec TOML field. Overrides MUST be applied when producing **normalized effective inputs before** the pure Construct function. For identical effective inputs, plan and generate MUST produce equal planned file sets (REQ-040). The override set and precedence MUST be part of the documented command surface (not implementation-private).

##### Rationale

Plan-as-contract remains enforceable when operators use common name/dest/verify overrides (FND-001).

##### Acceptance Evidence

CLI tests: plan with `--dest A` equals generate Construct with same flags; flag wins over TOML when both set; unknown override flags for non-allowed fields remain rejected.

##### Exceptions

Profile membership has no CLI override in v1. Future profile flags require explicit REQ + order rules (see FND-004 residual).

#### REQ-040 — Plan-as-contract

- **Priority:** Must · **Applies to:** plan/generate · **Phase:** PHASE-01 · **Sources:** REC-202 · **Verification:** SPK-100 golden plans · **Risk:** RSK-101

##### Requirement

validate, plan, and generate MUST share one pure construction function for a given set of **effective inputs** (Project Spec after CLI override application per REQ-034, then resolve normalization including canonical profile order per REQ-063). Generate MUST NOT silently diverge from Construct used for plan when effective inputs match.

##### Rationale

Dry-run trust for agents; override-aware reconstructability (FND-001).

##### Acceptance Evidence

Golden plan equality tests including override pairs.

##### Exceptions

Non-binding warnings may differ only if documented; planned file set MUST NOT differ.

#### REQ-041 — Plan elements complete

- **Priority:** Must · **Applies to:** `foundry plan` · **Phase:** PHASE-01 · **Sources:** REC-202 · **Verification:** JSON schema/fixture tests · **Risk:** OQ-200

##### Requirement

Plans MUST include all elements in §11.2 (version, catalog digest, normalized spec, resolved composition, planned files with digests, dep deltas, AI-native paths, verify mode, destination policy, plan_sha256, warnings).

##### Rationale

Reviewability.

##### Acceptance Evidence

Fixture plans contain fields/elements.

##### Exceptions

Exact JSON key spelling MAY refine under OQ-200 with tests updated.

#### REQ-042 — Plan formats

- **Priority:** Must · **Applies to:** `foundry plan` · **Phase:** PHASE-01 · **Sources:** REC-202 · **Verification:** CLI tests · **Risk:** None

##### Requirement

Plan MUST emit human text by default and machine JSON via `--format json` and/or `--out`.

##### Rationale

Agents need JSON; humans need text.

##### Acceptance Evidence

Both modes tested.

##### Exceptions

None.

#### REQ-043 — Plan performs zero destination writes

- **Priority:** Must · **Applies to:** `foundry plan` · **Phase:** PHASE-01 · **Sources:** REC-202 · **Verification:** FS tests · **Risk:** RSK-100

##### Requirement

Plan MUST NOT place a project at destination.

##### Rationale

Dry-run safety.

##### Acceptance Evidence

FS tests.

##### Exceptions

`--out` plan file writes are not destination place.

#### REQ-050 — Sibling stage then exclusive place (lifecycle)

- **Priority:** Must · **Applies to:** `foundry generate` · **Phase:** PHASE-01 · **Sources:** REC-203; FND-005 · **Verification:** SPK-101 · **Risk:** RSK-100, RSK-112

##### Requirement

Generate MUST render to a sibling staging directory (or equivalent temp stage under dest parent), then exclusively place to destination only after successful verify (unless `--verify none`). Lifecycle:

| Outcome | Destination | Stage |
| ------- | ----------- | ----- |
| Render failure | Untouched | Retained for inspect |
| Verify failure (default/strict) | Untouched; no place | Retained for inspect |
| Place success | Complete tree | **Cleaned** (removed or renamed away; no duplicate leftover tree required) |
| Place failure | Unchanged or fail-closed partial policy documented | Retained; exit ≠ 0 |

On **any** failure after stage creation, the error output MUST print the stage path so agents can inspect. Destination MUST remain untouched on render/verify failure.

##### Rationale

Aligns with architecture REC-203 write table; prevents disk fill and undocumentable inspect paths (FND-005; RSK-112).

##### Acceptance Evidence

Failure injection leaves dest clean and prints stage path; success leaves no duplicate stage tree beside dest.

##### Exceptions

`--verify none` skips verify but still follows place success cleanup. Optional future `--clean-stage` on failure is not required for v1 Default.

#### REQ-051 — Destination admissibility, emptiness, and place algorithm

- **Priority:** Must · **Applies to:** `foundry generate` · **Phase:** PHASE-01 · **Sources:** REC-203; Blueprint non-goal; FND-002 · **Verification:** SPK-101 · **Risk:** RSK-100

##### Requirement

**Admissibility (Must):**

| Destination state | Policy |
| ----------------- | ------ |
| Path missing | Admissible; create on place |
| Existing directory with **zero** children after `read_dir` | Admissible (empty) |
| Existing directory with any child (including `.git`, `.DS_Store`, etc.) | **Refuse** — non-empty |
| Existing file at path | **Refuse** |
| Symlink at path (any) | **Refuse** in v1 |

**Emptiness predicate:** a directory is empty if and only if `read_dir` yields zero entries. Dotfiles and `.git` count as children (non-empty).

**Refuse behavior:** on inadmissible destination, generate MUST fail without merge or overwrite; destination MUST remain unchanged; exit ≠ 0.

**Place algorithm (Must Default):**

1. Prefer **atomic rename** of stage → destination when stage and destination are on the **same filesystem**.
2. If same-FS rename is not possible (cross-device / EXDEV): **fail closed** with a clear error (do not silently partial-copy in v1 Default). Document that operators should place on the same filesystem as the stage parent, or that a future DEC may add copy+verify+swap.
3. Never merge into an existing non-empty tree.

##### Rationale

Blueprint forbids merge-into-non-empty default; testable emptiness removes implementer lottery (FND-002).

##### Acceptance Evidence

Table-driven SPK-101: missing; empty dir; dir with `.git`; file-at-path; symlink; same-FS success; cross-device fail-closed message.

##### Exceptions

Force-overwrite / merge only via future explicit DEC Exception — not v1 Default. Cross-device copy+swap only via future DEC.

#### REQ-052 — No merge/update existing projects v1

- **Priority:** Must · **Applies to:** Foundry product · **Phase:** PHASE-01 · **Sources:** REC-203; REC-213 · **Verification:** Command surface absence · **Risk:** None

##### Requirement

v1 MUST NOT implement merge/update/recopy into existing projects as a product feature.

##### Rationale

Scope and safety.

##### Acceptance Evidence

No such commands.

##### Exceptions

None.

---


#### REQ-053 — Stage-root path jail

- **Priority:** Must · **Applies to:** plan construct + stage render · **Phase:** PHASE-01 · **Sources:** REC-203; RSK-109; FND-003 · **Verification:** Path fixture tests · **Risk:** RSK-109

##### Requirement

Every planned and rendered filesystem path MUST resolve **strictly under the stage root** after normalization. Absolute paths, `..` components that escape stage root, and symlink resolution that escapes stage root MUST hard-fail at plan or render (exit ≠ 0) with no write outside stage root. Destination place MUST only publish the staged tree as a whole (exclusive place), not individual paths outside dest.

##### Rationale

Closed catalog lowers likelihood but does not remove template bugs or maintainer override mistakes (FND-003; RSK-109).

##### Acceptance Evidence

Fixtures for `../` escape, absolute path, and symlink escape: plan or render fails; no file created outside stage root.

##### Exceptions

None for Generated Project paths. Maintainer catalog-dev tools outside product generate remain out of band.

### 22.4 Catalog, profiles, TUI emit

#### REQ-060 — Closed embedded catalog

- **Priority:** Must · **Applies to:** Foundry catalog · **Phase:** PHASE-02 · **Sources:** REC-204 · **Verification:** Offline generate test · **Risk:** RSK-105, RSK-102

##### Requirement

Catalog MUST be closed and embedded in the foundry binary. Parse, resolve, Construct, and render MUST work **offline** without remote plugin or catalog fetch. This offline guarantee applies to **catalog and render**, not to default/strict verify (see REQ-120).

##### Rationale

Personal offline tool; no marketplace.

##### Acceptance Evidence

Network-blocked generate succeeds for sample.

##### Exceptions

Maintainer-only external override for catalog development.

#### REQ-061 — Custom planner-led engine

- **Priority:** Must · **Applies to:** Foundry engine · **Phase:** PHASE-02 · **Sources:** REC-204; REC-213 · **Verification:** Architecture/code inspection · **Risk:** RSK-105

##### Requirement

Product engine MUST be a custom planner-led renderer. It MUST NOT shell to cargo-generate or Copier as the engine.

##### Rationale

Enforce invariants (AI-native, profiles).

##### Acceptance Evidence

No engine dependency on those tools.

##### Exceptions

None.

#### REQ-062 — Catalog unit set

- **Priority:** Must · **Applies to:** Catalog · **Phase:** PHASE-02 · **Sources:** REC-205; REC-015 · **Verification:** `catalog list` tests · **Risk:** RSK-104

##### Requirement

Catalog MUST expose units: `core`, archetype `cli`, profiles `tui`, `hooks`, `secrets`, `distribution` as in §11.3. Unknown profile ids MUST hard fail at resolve.

##### Rationale

Stable closed membership.

##### Acceptance Evidence

list/show + resolve fail tests.

##### Exceptions

None without schema/Blueprint change.

#### REQ-063 — Emit matrix composition

- **Priority:** Must · **Applies to:** Plan construction · **Phase:** PHASE-02 · **Sources:** REC-205; REC-015; FND-004 · **Verification:** SPK-102; permutation digests · **Risk:** RSK-104

##### Requirement

Composition MUST be deterministic left-fold: `core` → `archetype:cli` → **canonically ordered** selected profiles. Canonical profile order (independent of TOML array order) is: `tui`, then `hooks`, then `secrets`, then `distribution`. Specs that differ only in profile permutation MUST produce identical ordered composition and planned file sets. Emit MUST realize membership in §7.2 / architecture emit matrix (toolchain, layout, clap, justfile, GHA, gitignore hygiene always; profile files conditional).

##### Rationale

Architecture freeze of REC-015; removes profile order lottery (FND-004).

##### Acceptance Evidence

Plan inventories for Core-only and each profile; two specs differing only in profile array order produce identical plan file digests.

##### Exceptions

None.

#### REQ-064 — Core always emits

- **Priority:** Must · **Applies to:** Generated Projects · **Phase:** PHASE-02 · **Sources:** REC-001..003,005,008,013,015; REC-205 · **Verification:** Plan/tree fixtures · **Risk:** None

##### Requirement

Every generate MUST emit at least: `rust-toolchain.toml`; edition 2024 Cargo package; thin main+lib layout; clap-based CLI skeleton; justfile; `.gitignore` with secret hygiene; GitHub Actions CI workflow; README/teach slots as catalog defines.

##### Rationale

Strong Core invariant.

##### Acceptance Evidence

Minimal CLI fixture.

##### Exceptions

None.

#### REQ-065 — TUI generate-time inclusion

- **Priority:** Must · **Applies to:** profile `tui` · **Phase:** PHASE-02 · **Sources:** REC-206; REC-009; OQ-003 · **Verification:** SPK-102 · **Risk:** RSK-103

##### Requirement

When `tui` profile selected, generate MUST include TUI modules, ratatui+crossterm deps, and `add-tui-screen` skill. When not selected, plan and tree MUST contain none of those. Feature-flag-only TUI that always ships TUI sources MUST NOT be the primary mechanism.

##### Rationale

Resolved OQ-003; pure CLI cleanliness.

##### Acceptance Evidence

Positive and negative inventory tests.

##### Exceptions

Optional later hybrid *inside* TUI projects only (OQ-108) without polluting pure CLI.

#### REQ-066 — hooks profile

- **Priority:** Should · **Applies to:** profile `hooks` · **Phase:** PHASE-02 · **Sources:** REC-011; REC-205 · **Verification:** Emit fixture · **Risk:** RSK-004

##### Requirement

When `hooks` selected, generate SHOULD emit pre-commit (Default) configuration wiring fmt/clippy gates. hk remains Watchlist alternate, not Default.

##### Rationale

Optional local hooks.

##### Acceptance Evidence

hooks fixture.

##### Exceptions

Owner may choose hk later via DEC.

#### REQ-067 — secrets profile

- **Priority:** Should · **Applies to:** profile `secrets` · **Phase:** PHASE-02 · **Sources:** REC-012; REC-205 · **Verification:** Emit fixture · **Risk:** RSK-005

##### Requirement

When `secrets` selected, generate SHOULD emit fnox + age scaffolding and docs. Secret gitignore hygiene MUST still apply on pure Core without the profile.

##### Rationale

Optional secrets; always hygiene.

##### Acceptance Evidence

Core gitignore always; secrets fixture when profiled.

##### Exceptions

None.

#### REQ-068 — distribution profile

- **Priority:** Should · **Applies to:** profile `distribution` · **Phase:** PHASE-03 · **Sources:** REC-014; L6 · **Verification:** Emit fixture · **Risk:** RSK-006

##### Requirement

When `distribution` selected, generate SHOULD emit cargo-dist config and release workflow targeting Linux+macOS only. Distribution MUST NOT be Core for all projects.

##### Rationale

L6 optional distribution.

##### Acceptance Evidence

Profile fixture; no Windows targets.

##### Exceptions

None.

#### REQ-069 — rust-version floor policy

- **Priority:** Should · **Applies to:** Generated Cargo.toml · **Phase:** PHASE-02 · **Sources:** REC-002; OQ-100 · **Verification:** Fixture inspect · **Risk:** RSK-001

##### Requirement

Generated projects SHOULD stamp `package.rust-version` to floor **1.85** (or higher floor if edition requirements rise). Generate MUST NOT require stamping “then-current stable minor” as policy.

##### Rationale

Synthesis closes OQ-100 with stable floor.

##### Acceptance Evidence

Fixture Cargo.toml.

##### Exceptions

Owner DEC may raise floor later.

#### REQ-070 — Default full CLI includes tracing

- **Priority:** Should · **Applies to:** Default full CLI template · **Phase:** PHASE-02 · **Sources:** REC-018; OQ-102 · **Verification:** Fixture deps · **Risk:** None

##### Requirement

Default full CLI template SHOULD include `tracing` and `tracing-subscriber`. Ultra-minimal hello-world samples MAY omit tracing if explicitly labeled minimal and not the default path.

##### Rationale

Synthesis closes OQ-102.

##### Acceptance Evidence

Default fixture deps.

##### Exceptions

Documented minimal sample only.

---

### 22.5 Generated Project stack (G1 freeze)

#### REQ-080 — Toolchain pin file

- **Priority:** Must · **Applies to:** Generated Core · **Phase:** PHASE-02 · **Sources:** REC-001 · **Verification:** File content tests · **Risk:** RSK-001

##### Requirement

Generate MUST emit `rust-toolchain.toml` with `channel = "stable"`, components including `rustfmt` and `clippy`, and `profile = "minimal"` (or equivalent documented).

##### Rationale

Reproducible toolchain.

##### Acceptance Evidence

Fixture equality.

##### Exceptions

None.

#### REQ-081 — Edition 2024

- **Priority:** Must · **Applies to:** Generated Core · **Phase:** PHASE-02 · **Sources:** REC-002 · **Verification:** Cargo.toml · **Risk:** None

##### Requirement

Generated `Cargo.toml` MUST set `edition = "2024"`.

##### Rationale

Matches modern cargo new.

##### Acceptance Evidence

Fixture.

##### Exceptions

None.

#### REQ-082 — rustfmt and clippy Required

- **Priority:** Must · **Applies to:** Generated Core + CI · **Phase:** PHASE-02 · **Sources:** REC-005 · **Verification:** CI + justfile · **Risk:** RSK-008

##### Requirement

Generated projects MUST include rustfmt and clippy in developer/CI gates with clippy deny-warnings in CI.

##### Rationale

Quality floor.

##### Acceptance Evidence

Workflow + just recipes.

##### Exceptions

None.

#### REQ-083 — cargo test Required; nextest optional

- **Priority:** Must · **Applies to:** Generated Core · **Phase:** PHASE-02 · **Sources:** REC-006; OQ-101 · **Verification:** justfile/CI · **Risk:** RSK-002

##### Requirement

Primary test runner MUST be `cargo test`. nextest MAY be documented/used when installed; default CI templates MUST NOT require nextest install.

##### Rationale

Zero-install hard gate; SPK-001 cost.

##### Acceptance Evidence

CI uses cargo test.

##### Exceptions

strict verify tier may use nextest if present.

#### REQ-084 — anyhow / thiserror policy

- **Priority:** Should · **Applies to:** Generated Core · **Phase:** PHASE-02 · **Sources:** REC-007 · **Verification:** deps · **Risk:** None

##### Requirement

Default app CLI SHOULD depend on anyhow. thiserror SHOULD be used when exposing typed library errors.

##### Rationale

Ecosystem default errors.

##### Acceptance Evidence

Default fixture deps.

##### Exceptions

None.

#### REQ-085 — clap derive Required for CLI

- **Priority:** Must · **Applies to:** Generated CLI · **Phase:** PHASE-02 · **Sources:** REC-008 · **Verification:** source + deps · **Risk:** None

##### Requirement

Generated CLI archetype MUST use clap with derive API.

##### Rationale

Required CLI framework.

##### Acceptance Evidence

Fixture source.

##### Exceptions

None.

#### REQ-086 — just Default task runner

- **Priority:** Should · **Applies to:** Generated Core · **Phase:** PHASE-02 · **Sources:** REC-010; REC-016 · **Verification:** justfile present · **Risk:** RSK-009

##### Requirement

Generate SHOULD emit a justfile with fmt/lint/test/ci (or check) recipes matching §12.2. Docs MUST include cargo fallbacks when just missing.

##### Rationale

Agent/human single surface.

##### Acceptance Evidence

justfile + AGENTS.md fallbacks.

##### Exceptions

None.

#### REQ-087 — GitHub Actions Core CI

- **Priority:** Must · **Applies to:** Generated Core · **Phase:** PHASE-03 · **Sources:** REC-013; L5 · **Verification:** workflow file · **Risk:** RSK-007

##### Requirement

Generate MUST emit GitHub Actions CI running format check, clippy deny-warnings, and tests on Linux. Windows jobs MUST NOT be emitted. macOS SHOULD be included when distribution profile or mac binary claims apply.

##### Rationale

L5; OQ-103 closed.

##### Acceptance Evidence

Workflow fixture.

##### Exceptions

None for Linux Required.

#### REQ-088 — Command surface documentation

- **Priority:** Must · **Applies to:** Generated docs/agent surface · **Phase:** PHASE-04 · **Sources:** REC-016; REC-108 · **Verification:** string match tests · **Risk:** RSK-052

##### Requirement

AGENTS.md, skills DoD, justfile, and CI MUST present the same primary gate (`just ci` / `just check`) and cargo fallbacks per §12.2.

##### Rationale

No oral tradition; no CI≠local.

##### Acceptance Evidence

Consistency tests.

##### Exceptions

None.

#### REQ-089 — Licensing posture for Core deps

- **Priority:** Should · **Applies to:** Catalog pins · **Phase:** PHASE-02 · **Sources:** REC-019 · **Verification:** license review at pin · **Risk:** None

##### Requirement

Core dependency pins SHOULD prefer permissive licenses (MIT/Apache-2.0 or equivalent). Non-permissive Core deps require explicit Exception documentation.

##### Rationale

Open-sourceable personal tool.

##### Acceptance Evidence

Pin review checklist.

##### Exceptions

Documented Exception only.

---

### 22.6 AI-native emit and skills

#### REQ-100 — Generated AGENTS.md + skills layout

- **Priority:** Must · **Applies to:** Generated Projects · **Phase:** PHASE-04 · **Sources:** REC-100; REC-211 · **Verification:** emit tests · **Risk:** RSK-106

##### Requirement

Every Generated Project MUST include root `AGENTS.md` and `.agents/skills/<name>/SKILL.md` layout per agentskills conventions.

##### Rationale

L7 portable surface.

##### Acceptance Evidence

Tree fixture.

##### Exceptions

None.

#### REQ-101 — AGENTS.md content contract

- **Priority:** Must · **Applies to:** Generated AGENTS.md · **Phase:** PHASE-04 · **Sources:** REC-102 · **Verification:** content checklist · **Risk:** None

##### Requirement

AGENTS.md MUST include: project summary; authority pointers; canonical commands (REQ-088); definition of done; layout map; boundaries (secrets, no Windows, no Claude-required surfaces, no MCP required); skill index; short teach-as-you-go one-liners. It MUST NOT be an encyclopedia of all crate docs.

##### Rationale

Agent operability.

##### Acceptance Evidence

Template review checklist.

##### Exceptions

None.

#### REQ-102 — Generated Core skills

- **Priority:** Must · **Applies to:** Pure CLI generate · **Phase:** PHASE-04 · **Sources:** REC-103; REC-211 · **Verification:** path tests · **Risk:** RSK-050

##### Requirement

Pure CLI generate MUST emit exactly Core skills: `quality-gates` and `add-subcommand`. It MUST NOT emit `add-tui-screen`.

##### Rationale

Closed Core skill set.

##### Acceptance Evidence

Inventory tests.

##### Exceptions

None.

#### REQ-103 — TUI skill delta

- **Priority:** Must · **Applies to:** profile `tui` · **Phase:** PHASE-04 · **Sources:** REC-104; REC-206 · **Verification:** inventory tests · **Risk:** RSK-056

##### Requirement

When `tui` profile selected, generate MUST also emit `add-tui-screen`. When not selected, MUST NOT.

##### Rationale

Profile-gated skills.

##### Acceptance Evidence

Matrix tests.

##### Exceptions

None.

#### REQ-104 — Foundry product skill catalog

- **Priority:** Must · **Applies to:** Foundry product repo · **Phase:** PHASE-04 · **Sources:** REC-212; REC-101; OQ-051 · **Verification:** skill paths · **Risk:** RSK-111

##### Requirement

Foundry product MUST use closed skill ids: `plan-generate`, `catalog-inspect`, `foundry-quality-gates`. These MUST NEVER be emitted into Generated Projects. Research-program skills MUST NOT be emitted into Generated Projects.

##### Rationale

Surface separation; OQ-051 resolved.

##### Acceptance Evidence

Product tree + forbidden emit tests.

##### Exceptions

Bodies MAY land in implementation; ids frozen now.

#### REQ-105 — Definition of done embeds command surface

- **Priority:** Must · **Applies to:** Generated DoD · **Phase:** PHASE-04 · **Sources:** REC-108; REC-016 · **Verification:** AGENTS/skills text · **Risk:** RSK-052

##### Requirement

DoD MUST require passing the primary gate before claiming complete work, aligned with REQ-088.

##### Rationale

Agent completion honesty.

##### Acceptance Evidence

Text + scenario tests.

##### Exceptions

None.

#### REQ-106 — Multi-product portability baseline

- **Priority:** Must · **Applies to:** Emit policy · **Phase:** PHASE-04 · **Sources:** REC-109 · **Verification:** no vendor-required Core · **Risk:** RSK-053

##### Requirement

Baseline emit MUST be standards-only (`AGENTS.md` + `.agents/skills`). Optional product adapters (e.g. Cursor rules) MUST NOT be default Core (see OQ-109 residual).

##### Rationale

Portability across agents.

##### Acceptance Evidence

Emit matrix.

##### Exceptions

Optional adapter only with owner need.

#### REQ-107 — Repo boundary rules in AGENTS.md

- **Priority:** Must · **Applies to:** Generated AGENTS.md · **Phase:** PHASE-04 · **Sources:** REC-110 · **Verification:** content checklist · **Risk:** RSK-055

##### Requirement

AGENTS.md MUST state boundaries: never commit secrets; do not invent Windows support; do not add Claude-required surfaces; do not require MCP; stay within single-crate v1 scope.

##### Rationale

Safety for agent-primary workflow.

##### Acceptance Evidence

Checklist.

##### Exceptions

None.

#### REQ-108 — rust-analyzer not DoD

- **Priority:** Must · **Applies to:** DoD docs · **Phase:** PHASE-04 · **Sources:** REC-107 · **Verification:** doc inspection · **Risk:** None

##### Requirement

Definition of done MUST use CLI gates, not LSP/rust-analyzer diagnostics as required completion criteria.

##### Rationale

Environment-optional editor tooling.

##### Acceptance Evidence

DoD text.

##### Exceptions

None.

---

### 22.7 Verify, template, teach-as-you-go, acceptance

#### REQ-120 — Default verify tier on generate

- **Priority:** Must · **Applies to:** `foundry generate` · **Phase:** PHASE-03 · **Sources:** REC-209 · **Verification:** SPK-103 · **Risk:** RSK-107

##### Requirement

Generate default verify mode MUST be `default` (primary gate on staged tree before place). `--verify none` MAY skip with documentation. `--verify strict` MAY add nextest-if-installed. Failure of default/strict MUST not place. Default and strict verify **MAY require network or a warm cargo cache** to fetch Generated Project dependencies; this is **not** an offline guarantee. Catalog/render offline requirements remain REQ-060.

##### Rationale

Trustable trees; clean dest.

##### Acceptance Evidence

Verify fail leaves dest absent/untouched.

##### Exceptions

none tier documented Exception path.

#### REQ-121 — Verify tools limited to Core surface

- **Priority:** Must · **Applies to:** verify module · **Phase:** PHASE-03 · **Sources:** REC-209; REC-016 · **Verification:** code/docs · **Risk:** None

##### Requirement

Required verify MUST NOT depend on cargo-deny, Miri, or shear as Core tools.

##### Rationale

Ecosystem Core alignment.

##### Acceptance Evidence

Verify implementation list.

##### Exceptions

None.

#### REQ-122 — GitHub template is catalog snapshot

- **Priority:** Must · **Applies to:** Hybrid template surface · **Phase:** PHASE-03 · **Sources:** REC-210; L1 · **Verification:** CI regen job · **Risk:** RSK-102

##### Requirement

Catalog MUST be source of truth. GitHub template MUST be a generated snapshot of default pure-CLI Core. Dual-editing template and catalog as co-equal SoT MUST NOT occur. Full generate remains path for profiles.

##### Rationale

Drift control.

##### Acceptance Evidence

CI regenerates template; policy docs.

##### Exceptions

None.

#### REQ-123 — Teach-as-you-go for Core and architecture defaults

- **Priority:** Must · **Applies to:** Generated + Foundry docs · **Phase:** PHASE-04 · **Sources:** REC-017; REC-112; REC-214 · **Verification:** content slots present · **Risk:** None

##### Requirement

Non-obvious defaults MUST ship short “why” notes covering at least: pipeline, TOML spec, plan-as-contract, refuse non-empty, embedded catalog, TUI generate-time, verify-before-place, template snapshot, AGENTS/skills without Claude/MCP, and Core crate rationales (concise).

##### Rationale

Owner less Rust depth; Blueprint success criterion.

##### Acceptance Evidence

Doc slots in catalog templates.

##### Exceptions

None.

#### REQ-130 — Module boundaries Default

- **Priority:** Should · **Applies to:** Foundry product · **Phase:** PHASE-01 · **Sources:** REC-208 · **Verification:** module map inspection · **Risk:** None

##### Requirement

Foundry product SHOULD use single crate with modules in §10.1. Micro-crate explosion MUST NOT be default v1.

##### Rationale

Owner mental load.

##### Acceptance Evidence

Module tree.

##### Exceptions

Split core/cli if second consumer appears.

#### REQ-150 — Acceptance: validate/plan/generate pure CLI

- **Priority:** Must · **Applies to:** Product acceptance · **Phase:** PHASE-05 · **Sources:** REC-215; REC-113 · **Verification:** automated scenarios · **Risk:** None

##### Requirement

Before claiming template freeze: validate+plan succeed on sample pure-CLI spec; generate into **missing or empty** dest yields runnable CLI; generate refuses non-empty (including dir with `.git`), file-at-path, and symlink dest; plan file digests match placed tree for matching effective inputs (including override pairs); path-jail fixtures fail closed; no ratatui/no add-tui-screen on pure CLI; primary gate passes post-generate when network/cache available; no CLAUDE.md; no default MCP.

##### Rationale

Architecture acceptance scenarios.

##### Acceptance Evidence

CI scenario suite.

##### Exceptions

None.

#### REQ-151 — Acceptance: TUI profile matrix

- **Priority:** Must · **Applies to:** Product acceptance · **Phase:** PHASE-05 · **Sources:** REC-215 · **Verification:** automated scenarios · **Risk:** RSK-103

##### Requirement

TUI profile sample MUST generate with TUI entry, ratatui/crossterm present, add-tui-screen present, and still pass primary gate and forbid Claude/MCP defaults.

##### Rationale

Profile path proof.

##### Acceptance Evidence

TUI scenario suite.

##### Exceptions

None.

#### REQ-152 — Acceptance: agent operability (recommended)

- **Priority:** Should · **Applies to:** Product acceptance · **Phase:** PHASE-05 · **Sources:** REC-113; SPK-050/051 · **Verification:** SPK or manual agent trial · **Risk:** None

##### Requirement

Agent SHOULD successfully add a subcommand via skill on pure CLI and add a TUI screen only when TUI profile is present.

##### Rationale

Agent-primary success criteria.

##### Acceptance Evidence

SPK-050/051 or equivalent.

##### Exceptions

Recommended not hard-blocking if automated agent matrix unavailable; document residual.

#### REQ-160 — Prior-art transfer table authoritative for synthesis

- **Priority:** Must · **Applies to:** Spec narrative · **Phase:** N/A · **Sources:** REC-213; L8 · **Verification:** §28 dispositions · **Risk:** None

##### Requirement

Implementation and docs MUST respect Adopt/Adapt/Reject in REC-213 (summarized §2). Go/Python stacks MUST NOT be cited as sole proof of Rust Core choices.

##### Rationale

Balanced transfer.

##### Acceptance Evidence

No blind-copy Core justifications.

##### Exceptions

None.

---

## 23. Traceability

| REQ | Sources | Phase |
| --- | ------- | ----- |
| REQ-001 | L1; REC-210; REC-213 | PHASE-01 |
| REQ-002 | L2 | PHASE-01 |
| REQ-003 | L3 | PHASE-01 |
| REQ-004 | L4; REC-004; REC-206 | PHASE-02 |
| REQ-005 | L7; REC-015; REC-103 | PHASE-02 |
| REQ-006 | L7; REC-100; REC-211 | PHASE-04 |
| REQ-007 | L7; REC-106; REC-211 | PHASE-04 |
| REQ-008 | L9; REC-102; REC-108 | PHASE-04 |
| REQ-009 | Blueprint; REC-003 | PHASE-02 |
| REQ-010 | L12 | N/A |
| REQ-020 | REC-200; REC-207 | PHASE-01 |
| REQ-021 | REC-200; REC-201 | PHASE-01 |
| REQ-022 | REC-200 | PHASE-01 |
| REQ-023 | REC-200 | PHASE-01 |
| REQ-024 | REC-008; REC-207 | PHASE-01 |
| REQ-025 | REC-204; REC-207 | PHASE-02 |
| REQ-030 | REC-201 | PHASE-01 |
| REQ-031 | REC-201 | PHASE-01 |
| REQ-032 | REC-201; REC-004 | PHASE-01 |
| REQ-033 | REC-201; REC-012; FND-008 | PHASE-01 |
| REQ-034 | REC-201; FND-001 | PHASE-01 |
| REQ-040 | REC-202; FND-001 | PHASE-01 |
| REQ-041 | REC-202 | PHASE-01 |
| REQ-042 | REC-202 | PHASE-01 |
| REQ-043 | REC-202 | PHASE-01 |
| REQ-050 | REC-203; FND-005 | PHASE-01 |
| REQ-051 | REC-203; FND-002 | PHASE-01 |
| REQ-052 | REC-203; REC-213 | PHASE-01 |
| REQ-053 | REC-203; RSK-109; FND-003 | PHASE-01 |
| REQ-060 | REC-204; FND-006 | PHASE-02 |
| REQ-061 | REC-204; REC-213 | PHASE-02 |
| REQ-062 | REC-205; REC-015 | PHASE-02 |
| REQ-063 | REC-205; REC-015; FND-004 | PHASE-02 |
| REQ-064 | REC-001..003,005,008,013,015; REC-205 | PHASE-02 |
| REQ-065 | REC-206; REC-009 | PHASE-02 |
| REQ-066 | REC-011; REC-205 | PHASE-02 |
| REQ-067 | REC-012; REC-205 | PHASE-02 |
| REQ-068 | REC-014; L6 | PHASE-03 |
| REQ-069 | REC-002; OQ-100 | PHASE-02 |
| REQ-070 | REC-018; OQ-102 | PHASE-02 |
| REQ-080 | REC-001 | PHASE-02 |
| REQ-081 | REC-002 | PHASE-02 |
| REQ-082 | REC-005 | PHASE-02 |
| REQ-083 | REC-006; OQ-101 | PHASE-02 |
| REQ-084 | REC-007 | PHASE-02 |
| REQ-085 | REC-008 | PHASE-02 |
| REQ-086 | REC-010; REC-016 | PHASE-02 |
| REQ-087 | REC-013; L5; OQ-103 | PHASE-03 |
| REQ-088 | REC-016; REC-108 | PHASE-04 |
| REQ-089 | REC-019 | PHASE-02 |
| REQ-100 | REC-100; REC-211 | PHASE-04 |
| REQ-101 | REC-102 | PHASE-04 |
| REQ-102 | REC-103; REC-211 | PHASE-04 |
| REQ-103 | REC-104; REC-206 | PHASE-04 |
| REQ-104 | REC-212; REC-101 | PHASE-04 |
| REQ-105 | REC-108; REC-016 | PHASE-04 |
| REQ-106 | REC-109 | PHASE-04 |
| REQ-107 | REC-110 | PHASE-04 |
| REQ-108 | REC-107 | PHASE-04 |
| REQ-120 | REC-209; FND-006 | PHASE-03 |
| REQ-121 | REC-209; REC-016 | PHASE-03 |
| REQ-122 | REC-210; L1 | PHASE-03 |
| REQ-123 | REC-017; REC-112; REC-214 | PHASE-04 |
| REQ-130 | REC-208 | PHASE-01 |
| REQ-150 | REC-215; REC-113 | PHASE-05 |
| REQ-151 | REC-215 | PHASE-05 |
| REQ-152 | REC-113; SPK-050/051 | PHASE-05 |
| REQ-160 | REC-213; L8 | N/A |

### High-level phases (not delivery plan)

| Phase | Focus |
| ----- | ----- |
| PHASE-01 | Foundry CLI skeleton, spec/plan/generate, write semantics, module map |
| PHASE-02 | Embedded catalog, emit matrix, Core templates, TUI/hooks/secrets units |
| PHASE-03 | Verify tiers, GHA, distribution profile, GH template regen |
| PHASE-04 | AI-native templates, skills bodies, teach-as-you-go slots, product skills |
| PHASE-05 | Acceptance scenario suite; freeze gates |

---

## 24. Risk Register

| ID | Risk | Mitigation (spec) | Residual |
| -- | ---- | ----------------- | -------- |
| RSK-001 | Stable fmt/clippy drift | Toolchain pin; CI | Monitor channel |
| RSK-002 | nextest doctest gaps | cargo test Required; strict docs | Document |
| RSK-003 / RSK-103 | TUI leakage into pure CLI | REQ-004, REQ-065, SPK-102 | Template bugs |
| RSK-004 | Hooks vs CI divergence | hooks profile wires same gates | Owner discipline |
| RSK-005 | Secret mishandling | gitignore; secrets profile docs | Agent mistakes |
| RSK-006 | cargo-dist pre-1.0 churn | Optional profile only | Pin carefully |
| RSK-007 | macOS CI cost/skip | Linux Required; mac Recommended when needed | Coverage gap |
| RSK-008 | Clippy noise for agents | -D warnings + teach notes | Tuning |
| RSK-009 | just missing on hosts | cargo fallbacks documented | Friction |
| RSK-050 | Skill catalog bloat | Closed sets REQ-005/102/104 | Pressure to expand |
| RSK-051 | MCP creep | REQ-007 | Social pressure |
| RSK-052 | DoD drift from CI | REQ-088/105 | Template drift |
| RSK-053 | Overfit one agent product | REQ-006/106 | Adapter temptation |
| RSK-055 | Secrets in agent docs | REQ-033/107 | Examples |
| RSK-056 | TUI skill on pure CLI | REQ-102/103 | Emit bugs |
| RSK-100 | Destructive overwrite | REQ-050/051 (emptiness + place) | Override DEC later |
| RSK-101 | Plan/apply divergence | REQ-040, REQ-034 | Implementer error |
| RSK-102 | Catalog/template drift | REQ-122; digest | CI failure |
| RSK-104 | Profile explosion | Closed set | Demand pressure |
| RSK-105 | Over-complex engine | REQ-061; single crate Default | Complexity |
| RSK-106 | Agent-surface emit bugs | Forbidden path tests | — |
| RSK-107 | Verify host gaps | cargo fallbacks | just missing |
| RSK-109 | Path escape | REQ-053 stage root jail + fixtures | Impl bugs |
| RSK-110 | Secrets in spec | REQ-033 denylist | Residual non-denylist keys |
| RSK-111 | Product skills in Generated | REQ-104 | — |
| RSK-112 | Stage dir leftovers | REQ-050 success clean / fail retain + path | Disk if inspect abandoned |

---

## 25. Open Questions

| ID | Question | Blocking? | Synthesis disposition |
| -- | -------- | --------- | --------------------- |
| OQ-100 | rust-version stamp policy | No | **Closed provisional:** floor 1.85 (REQ-069) |
| OQ-101 | nextest in default CI | No | **Closed provisional:** cargo test in default CI (REQ-083) |
| OQ-102 | Logging in hello-world | No | **Closed provisional:** tracing on default full CLI (REQ-070) |
| OQ-103 | macOS CI default | No | **Closed provisional:** Linux Required; mac Recommended when needed (REQ-087) |
| OQ-104 | Foundry product rustc pin | No | Carry to implementation; dogfood Generated policy |
| OQ-105 | Hooks tool if profile | No | **Closed provisional:** pre-commit Default (REQ-066); hk Watchlist |
| OQ-106 | GH template regen cadence | No | Implementation: on catalog change + release |
| OQ-107 | Saved plan file apply | No | **Deferred** v1; Construct always rebuilds |
| OQ-108 | Cargo feature hybrid inside TUI | No | Optional later; pure CLI unaffected |
| OQ-109 | Optional Cursor rules | No | Not default Core (REQ-106) |
| OQ-110 | Secrets skill when secrets profile | No | Deferred skill; profile scaffolding first |
| OQ-111 | Change-completion report contract | No | Deferred; not v1 Core DoD text |
| OQ-112 | Nested AGENTS.md monorepos | No | N/A v1 single-crate |
| OQ-003 | TUI mechanism | Was blocking | **Resolved** REC-206 / REQ-065 |
| OQ-051 | Foundry skill ids | Was blocking | **Resolved** REC-212 / REQ-104 |
| **OQ-200** | Exact plan JSON field names | No | Freeze elements §11.2; names refine via SPK-100 |
| **OQ-201** | Embed crate choice | No | Implementation (`include_dir` vs rust-embed) |
| **OQ-202** | Foundry skill body prose | No | Implementation before product ship |

Resolved ecosystem OQ-001..007 are carried via architecture OQ-100..105 as above; OQ-003/051 resolved upstream.

---

## 26. Deferred Work

| Item | Why deferred | Owner stage |
| ---- | ------------ | ----------- |
| Saved plan-file apply without rebuild | Complexity; OQ-107 | Post-v1 / DEC |
| Foundry product skill bodies | Ids frozen; prose later | Implementation PHASE-04 |
| Exact JSON plan key freeze | SPK-100 | Implementation |
| Embed crate selection | Non-load-bearing | Implementation |
| Cursor rules profile | Optional | Owner need |
| Secrets profile skill | Optional | Later REC/DEC |
| update/recopy/merge existing | Out of v1 | Future |
| Multi-crate / lib-only generate | Blueprint non-goal | Blueprint amendment |
| cargo-dist pin dry-run | Profile optional | Implementation |
| SPK-100..104 execution | Recommended gates | Implementation |

---

## 27. Rejected Work

| Item | Why rejected | Sources |
| ---- | ------------ | ------- |
| Windows support | L3 | Blueprint |
| Remote/plugin marketplace catalogs | Non-goal; closed sets | REC-204; REC-213 |
| Merge-into-non-empty default | Blueprint non-goal | REC-203 |
| cargo-generate/Copier as product engine | Weak plan/invariants | REC-204; REC-213 |
| Feature-flag-only TUI as primary | Dead files on pure CLI | REC-206 |
| Dual full template flavors as primary | Drift | REC-206 |
| Claude-specific Core surfaces | L7 | REC-100; REC-211 |
| Default MCP on | L7 | REC-106 |
| cargo-deny/Miri/shear as Required verify | Not ecosystem Core | REC-209 |
| Forced TUI archetype | L4 | REC-004 |
| Research skills in Generated Projects | Surface bleed | REC-101; REC-212 |
| Beads / Go-only stacks as Generated Core | Wrong stack | REC-213 |
| Framework zoo Core | Non-goals | Blueprint |
| Kitchen-sink init/update/recopy v1 | Scope | REC-207 |

---

## 28. Recommendation Disposition Ledger

Every substantive REC receives exactly one disposition.

### 28.1 Ecosystem REC-001..019

| REC | Disposition | Notes / surviving REQs |
| --- | ----------- | ---------------------- |
| REC-001 | **Accepted** | REQ-080, REQ-064 |
| REC-002 | **Accepted with modification** | Edition Required; rust-version floor 1.85 provisional (REQ-069, REQ-081) |
| REC-003 | **Accepted** | REQ-009, REQ-064 |
| REC-004 | **Accepted** | REQ-004, REQ-032, REQ-065 |
| REC-005 | **Accepted** | REQ-082 |
| REC-006 | **Accepted** | REQ-083; nextest not default CI hard gate |
| REC-007 | **Accepted** | REQ-084 |
| REC-008 | **Accepted** | REQ-024, REQ-085 |
| REC-009 | **Accepted** | REQ-065 (profile only) |
| REC-010 | **Accepted** | REQ-086 |
| REC-011 | **Accepted** | REQ-066 |
| REC-012 | **Accepted** | REQ-067; hygiene Core |
| REC-013 | **Accepted** | REQ-087 |
| REC-014 | **Accepted** | REQ-068 |
| REC-015 | **Merged** | Into REC-205 / REQ-062..064 emit matrix; membership retained |
| REC-016 | **Accepted** | REQ-088, REQ-105, REQ-120 |
| REC-017 | **Accepted** | REQ-123 |
| REC-018 | **Accepted with modification** | Default full CLI; minimal sample may omit (REQ-070) |
| REC-019 | **Accepted** | REQ-089 |

### 28.2 AI-native REC-100..113

| REC | Disposition | Notes / surviving REQs |
| --- | ----------- | ---------------------- |
| REC-100 | **Accepted** | REQ-100, REQ-006 |
| REC-101 | **Accepted** | REQ-104 separation |
| REC-102 | **Accepted** | REQ-101 |
| REC-103 | **Accepted** | REQ-102 |
| REC-104 | **Accepted** | REQ-103 |
| REC-105 | **Merged** | Into REC-212 / REQ-104 (ids closed) |
| REC-106 | **Accepted** | REQ-007 |
| REC-107 | **Accepted** | REQ-108 |
| REC-108 | **Accepted** | REQ-105, REQ-088 |
| REC-109 | **Accepted** | REQ-106 |
| REC-110 | **Accepted** | REQ-107 |
| REC-111 | **Merged** | Into REC-213 / REQ-160 transfer posture |
| REC-112 | **Accepted** | REQ-123 |
| REC-113 | **Accepted** | REQ-150..152 |

### 28.3 Architecture REC-200..215

| REC | Disposition | Notes / surviving REQs |
| --- | ----------- | ---------------------- |
| REC-200 | **Accepted** | REQ-020..023 |
| REC-201 | **Accepted** | REQ-030..034 |
| REC-202 | **Accepted** | REQ-040..043 |
| REC-203 | **Accepted** | REQ-050..053 |
| REC-204 | **Accepted** | REQ-060..061, REQ-025 |
| REC-205 | **Accepted** | REQ-062..064, REQ-066..068 |
| REC-206 | **Accepted** | REQ-004, REQ-065 (OQ-003 resolved) |
| REC-207 | **Accepted** | REQ-020, REQ-024, REQ-025 |
| REC-208 | **Accepted** | REQ-130 |
| REC-209 | **Accepted** | REQ-120, REQ-121 |
| REC-210 | **Accepted** | REQ-001, REQ-122 |
| REC-211 | **Accepted** | REQ-100..103, REQ-006..007 |
| REC-212 | **Accepted** | REQ-104 (OQ-051 resolved) |
| REC-213 | **Accepted** | REQ-160; §2/§27 tables |
| REC-214 | **Accepted** | REQ-123 |
| REC-215 | **Accepted** | REQ-150..152 |

**Disposition counts:** Accepted 44; Accepted with modification 2; Merged 3; Deferred 0 (as REC dispositions); Rejected 0; Superseded 0; N/A 0. **Total 49/49.** No silent REC loss.

---

## 29. Definition of Done

### Generated Project work (agent/human)

1. Changes complete for the task.
2. Primary gate passes: `just ci` / `just check` or cargo fallbacks (REQ-088).
3. No secrets committed; boundaries respected (REQ-107).
4. Skills used when applicable (`quality-gates`, `add-subcommand`, `add-tui-screen` if TUI).

### Foundry generate success

1. Spec validated; plan constructible.
2. Stage verify tier passes (unless `--verify none`).
3. Exclusive place succeeds to empty/missing dest.
4. Acceptance scenarios REQ-150/151 green for shipped templates.

### Specification stage (this artifact)

1. All RECs dispositioned (§28).
2. All FND-001..008 dispositioned (Finding Disposition Ledger).
3. Standalone REQs cover architecture freeze areas including write/CLI/security hardenings.
4. Artifact status: **Accepted — implementation authority** (stage accept still human gate).

---

## 30. Updated Implementation Handoff

### Downstream stage

`implementation-plan` → `docs/plans/01-implementation-plan.md` (after this revised
spec is human-accepted and `accepted_commit` recorded).

### What planners must treat as load-bearing

1. **Write safety:** REQ-050 lifecycle, REQ-051 emptiness/place, REQ-053 path jail.
2. **Plan-as-contract:** REQ-034 overrides + REQ-040 effective inputs + REQ-063 profile order.
3. **Catalog offline ≠ verify offline:** REQ-060 vs REQ-120.
4. **Emit matrix:** REQ-062..068; TUI zero-leakage REQ-004/065/103.
5. **AI-native:** REQ-100..108; MCP none; no Claude Core.
6. **Locks/non-goals:** L1–L9/L12; no Windows; no merge default; no remote catalogs.
7. **REC ledger:** §28 remains complete (49/49); do not silently drop RECs when phasing.

### Recommended implementation spikes (still recommended, not done)

| SPK | Purpose |
| --- | ------- |
| SPK-100 | Golden plan JSON freeze (OQ-200 names) |
| SPK-101 | Emptiness/place matrix + stage lifecycle (REQ-050/051) |
| SPK-102 | TUI pure-CLI separation |
| SPK-103 | Default verify tier cost/behavior (network/cache) |
| SPK-104 | Template snapshot regen digest |

### Residual non-blocking gaps

- Exact plan JSON field names (OQ-200) — elements frozen.
- Foundry skill body prose (ids frozen).
- Cross-device place remains fail-closed Default (copy+swap needs DEC).
- Secret denylist is name-based, not content entropy scanning.

### Second adversarial review?

**Not recommended by default** under standard rigor: High findings integrated with
tightened REQs only; no force-overwrite, merge default, Windows, MCP-on, or remote
catalog introduced. Re-open risk-triggered review only if implementation plan or
later DEC adds major new machinery.

### Out of scope for implementation-plan stage

- Re-picking clap/ratatui/edition without DEC / new evidence.
- Product implementation inside this research repo beyond optional spikes (L12).
- Expanding to Windows or marketplace.

---

## 31. Completion Checklist

- [x] Baseline `01-…` and review `01-…` consumed in full for revision
- [x] Finding Disposition Ledger covers FND-001..FND-008 with exactly one enum each
- [x] High findings FND-001..003 **Accepted** and integrated (overrides, emptiness/place, path jail)
- [x] Medium/Low findings FND-004..008 dispositioned and integrated
- [x] Integrated Correction Ledger + Preserved Strengths present
- [x] Every Must in security/write/pipeline tables maps to a REQ
- [x] New REQs only from unused ids (REQ-034, REQ-053); stable REQs retained where subject unchanged
- [x] No silent REC loss (49/49 in §28); REC-201/203 notes updated for new REQs
- [x] Locks L1–L9/L12 and non-goals intact; refuse non-empty default preserved
- [x] TUI optional; pure CLI free of TUI deps/files/skills
- [x] MCP default none; no Claude design target
- [x] Standalone revised specification (no chat dependence)
- [x] Status: **Accepted — implementation authority**
- [x] Updated implementation handoff toward `implementation-plan`
- [x] Allowed file scope: revised specification path (baseline not rewritten as deliverable)
- [x] Independent validation (`research-validate`) — see `docs/validations/02-definitive-specification-revised-validation.md`
- [x] Human approval of spec-revision stage
- [x] `accepted_commit` recorded in `research-program.toml`

---

*End of revised definitive specification v0.2 — rust-foundry.*
