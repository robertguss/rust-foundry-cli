# AGENTS.md

Project: {{name}}

## Quality gates (primary)

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`

## Skills

- `.agents/skills/quality-gates/`
- `.agents/skills/add-subcommand/`

## Forbidden

- No `CLAUDE.md` / `.claude/` defaults
- No default MCP kitchen-sink config
- No secret material in Project Specs
