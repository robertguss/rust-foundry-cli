# SPK-103 — Default verify network / cache notes

| Mode | Behavior |
| ---- | -------- |
| `--verify none` | Skip runners; place proceeds |
| `default` / `strict` | Primary gate: `just check` if present, else cargo fmt/clippy/test |
| Cold cache | May need network to fetch Generated Project crates |
| Warm cache | Often offline after first fetch |
| Missing `just` | Cargo fallbacks (freeze fixture) |
| Catalog/render | Offline via embed (REQ-060) — **not** the same as verify offline |

Catalog offline ≠ verify offline.
