# Using a generated project

After `foundry generate` succeeds, the destination directory contains a complete Rust project. This page explains how to work with it.

## Directory layout

A generated pure-CLI project looks like this:

```text
my-app/
  .agents/
    skills/
      add-subcommand/SKILL.md
      quality-gates/SKILL.md
  .github/
    workflows/
      ci.yml
  src/
    main.rs
  Cargo.toml
  Cargo.lock
  justfile
  README.md
  rust-toolchain.toml
```

If you selected profiles, additional files appear (for example, `src/tui/mod.rs` with the `tui` profile).

## First commands

Change into the generated directory and build the project:

```bash
cd my-app
cargo build
cargo test
cargo run -- --help
```

The generated CLI uses `clap`. Run it without arguments to see the default behavior, or use its flags:

```bash
cargo run -- --message "hello"
```

## Quality gates

The generated project is set up with the same quality gates used by Foundry:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

If you have `just` installed, you can run all three at once:

```bash
just check
```

These commands are also run in CI on every push.

## Agent surface

Generated projects include an `AGENTS.md` file at the root. This file is the entry point for AI coding agents working on your project. It describes the canonical commands, layout, and boundaries.

The `.agents/skills/` directory contains reusable skill files. For example:

- `add-subcommand/SKILL.md` — how to add a new CLI subcommand
- `quality-gates/SKILL.md` — how to run and understand the quality checks

Read these files when an agent (or you) wants to extend the project.

## CI

The `.github/workflows/ci.yml` file runs the quality gate on every push and pull request. It uses GitHub Actions on Linux. Foundry does not generate Windows CI jobs.

## Development workflow

1. Edit `src/main.rs` and other source files.
2. Run `just check` or the cargo commands above.
3. Commit and push; CI runs the same checks.

## Adding new CLI subcommands

If you want to add a subcommand, read `.agents/skills/add-subcommand/SKILL.md` in the generated project. It describes the pattern used by the generated `clap` code.

## Toolchain

Generated projects include `rust-toolchain.toml` to pin the Rust version. This matches the version used by Foundry and keeps CI reproducible.
