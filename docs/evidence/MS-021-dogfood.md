# MS-021 — Pure-CLI Core catalog generate dogfood gate

## Evidence

### 1. Offline catalog/render generate (MS-021.1)

- Catalog content is **compile-time embedded** (`include_dir`); generate does not
  fetch remote catalog (REQ-060).
- Command: `foundry generate --spec … --verify none` materializes pure-CLI tree
  without network for catalog/render.
- Note: `--verify none` is used so this is **not** a claim that default verify
  is offline (REQ-120 remains PHASE-03).

### 2. SPK-100 golden for Core catalog plan (MS-021.2)

- Goldens under `tests/snapshots/plan/` cover pure-CLI plan after Core embed
  (`minimal_cli_plan_json` / `minimal_cli_plan_text`).
- `tests/plan_snapshots.rs` + `tests/core_emit.rs` gate drift.

### 3. cargo test smoke on generated project (MS-021.3)

- Generated tree includes `Cargo.toml`, `src/main.rs`, clap binary.
- `cargo test` on generated project succeeds (0 tests is green compile smoke).
- Logs captured under agent scratch during dogfood run; reproducible via:

```bash
foundry generate --spec examples/minimal-cli.toml --dest /tmp/dogfood --verify none
# or override dest; then:
cd /tmp/dogfood && cargo test
```

## Residual

- Default verify network/cache expectations remain PHASE-03 / SPK-103.
- Generated project currently has no unit tests beyond binary compile — acceptable
  for MS-021 smoke (not full quality gate).
