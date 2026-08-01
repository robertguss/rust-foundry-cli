# Foundry User Guide

Welcome to **Foundry**, a command-line tool for generating opinionated Rust CLI projects from a small TOML file.

This guide is for people who want to *use* Foundry to create and scaffold projects. It does not describe how Foundry is built or how its internal catalog works.

## What Foundry does

Foundry reads a **Project Spec** (a short TOML file) and writes a complete, ready-to-build Rust CLI project into a destination directory.

The generated project includes:

- A working `clap`-based CLI in `src/main.rs`
- `Cargo.toml` and `Cargo.lock`
- `justfile` with common quality commands
- GitHub Actions CI
- `AGENTS.md` and `.agents/skills/` for AI/agent workflows
- Optional profiles such as `tui`, `hooks`, `secrets`, and `distribution`

## Workflow at a glance

1. Write a Project Spec TOML file.
2. `foundry validate --spec project.toml`
3. `foundry plan --spec project.toml`
4. `foundry generate --spec project.toml`
5. Work inside the generated directory.

## Guide contents

- [Installation](install.md)
- [Quickstart](quickstart.md)
- [Project Spec reference](project-spec.md)
- [Command reference](commands.md)
- [Catalog and profiles](catalog.md)
- [Using a generated project](generated-project.md)
- [Troubleshooting and error codes](troubleshooting.md)

## Requirements

- macOS or Linux (Windows is not supported)
- Rust 1.85 or newer
- `cargo` and a working C toolchain for native dependencies
