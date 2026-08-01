# REQ-150 / REQ-151 acceptance automation

Named scenario paths (CI job equivalents — local `cargo test` covers):

## REQ-150 pure-CLI
- validate_plan — tests/plan_cli.rs, tests/plan_integration.rs
- generate_missing_dest — tests/generate_lifecycle.rs, tests/spk101_matrix.rs
- refuse_nonempty — tests/spk101_matrix.rs
- path_jail — tests/path_jail_denylist.rs
- no_tui_leak — tests/profiles_tui_spk102.rs
- plan_digest_equality — tests/plan_integration.rs

## REQ-151 TUI
- tui_generate_paths — tests/profiles_tui_spk102.rs tui_profile_includes_tui_paths
- no_claude_mcp — tests/phase03_05_gates.rs

## Spot-check (MS-019.3)
Override equality, refuse non-empty, path jail remain green in continuous suite.
