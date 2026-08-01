# Catalog and profiles

Foundry uses a **closed catalog** of project units. "Closed" means the catalog is embedded in the `foundry` binary and cannot be extended by end users in v1.

## Catalog units

| Unit id | Kind | Description | Always included? |
| ------- | ---- | ----------- | ---------------- |
| `core` | core | Toolchain, gitignore, `AGENTS.md`, and quality skill files. | Yes |
| `cli` | archetype | Pure clap-based CLI project. | Yes (only `cli` in v1) |
| `tui` | profile | `ratatui` + `crossterm` TUI modules. | Optional |
| `hooks` | profile | Pre-commit configuration. | Optional |
| `secrets` | profile | `fnox` + `age` secrets scaffolding. | Optional |
| `distribution` | profile | `cargo-dist` release configuration. | Optional |

## Core files

The `core` unit is always included. It adds:

- `.gitignore`
- `rust-toolchain.toml`
- `AGENTS.md`
- `.agents/skills/add-subcommand/SKILL.md`
- `.agents/skills/quality-gates/SKILL.md`

## CLI archetype

The `cli` archetype adds the minimum files for a working Rust CLI:

- `Cargo.toml`
- `src/main.rs`
- `README.md`
- `justfile`
- `.github/workflows/ci.yml`

The generated `src/main.rs` uses `clap` derive macros and is ready to run with `cargo run`.

## Optional profiles

### `tui`

Adds a `src/tui/mod.rs` module and an `add-tui-screen` skill file for building terminal user interfaces with `ratatui` and `crossterm`.

Requires: `core`, `cli`

### `hooks`

Adds `.pre-commit-config.yaml` for pre-commit hooks.

Requires: `core`

### `secrets`

Adds `.secrets/README.md` with a secrets-management layout.

Requires: `core`

### `distribution`

Adds `cargo-dist` release configuration files:

- `.github/workflows/release.yml`
- `dist-workspace.toml`

Requires: `core`, `cli`

## Profile ordering

When you list multiple profiles, Foundry applies them in a canonical order regardless of how you write the array:

1. `tui`
2. `hooks`
3. `secrets`
4. `distribution`

For example, these two specs produce the same project:

```toml
profiles = ["distribution", "tui"]
```

```toml
profiles = ["tui", "distribution"]
```

## Inspect the catalog

Use the catalog commands to see what is currently built into Foundry:

```bash
foundry catalog list
foundry catalog show tui
```

## Constraints

- Only the `cli` archetype is available in v1.
- A profile can only be listed once in the `profiles` array.
- Unknown profile ids cause `validate`, `plan`, and `generate` to fail.
