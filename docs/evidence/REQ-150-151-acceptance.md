# REQ-150 / REQ-151 acceptance automation

Named scenario paths = **`cargo test` filter names** (CI job equivalents).

## REQ-150 pure-CLI — `tests/req150_acceptance.rs`

| Job / path name | Scenario |
| --------------- | -------- |
| `req150_validate_and_plan_sample` | validate + plan on sample pure-CLI spec |
| `req150_generate_missing_dest` | generate into missing dest |
| `req150_generate_empty_dest` | generate into empty dir |
| `req150_refuse_nonempty` | refuse non-empty (incl `.git`) |
| `req150_refuse_file_at_path` | refuse file-at-path dest |
| `req150_plan_digests_match_tree` | plan content digests match placed tree |
| `req150_path_jail` | absolute / `..` paths hard-fail |
| `req150_no_tui_leakage` | no ratatui / add-tui-screen on pure CLI |
| `req150_no_claude_mcp` | no CLAUDE.md / `.claude/` / default MCP |

CI invocation:

```bash
cargo test --test req150_acceptance
# or named:
cargo test --test req150_acceptance req150_refuse_nonempty
```

## REQ-151 TUI — `tests/profiles_tui_spk102.rs`

| Job / path name | Scenario |
| --------------- | -------- |
| `tui_profile_includes_tui_paths` | TUI sample paths when profile selected |
| `side_by_side_inventory` | pure CLI ⊂ TUI inventory |
| `generate_pure_cli_no_ratatui_files` | pure CLI leakage gate |

## Spot-check (MS-019.3)

Override equality, refuse non-empty, path jail remain green via:

- `tests/plan_integration.rs`
- `tests/spk101_matrix.rs`
- `tests/path_jail_denylist.rs`
- `tests/req150_acceptance.rs`
