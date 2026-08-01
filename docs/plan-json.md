# Plan JSON field names (frozen at MS-004 / SPK-100)

After **MS-004**, plan JSON field names are **stable**. Additive renames require
tests and conscious migration (do not drive-by rename in generate work).

OQ-200 residual: **closed** by this document + committed goldens under
`tests/snapshots/`.

## Success object (`foundry plan --format json`)

| Field | Type | Notes |
| ----- | ---- | ----- |
| `ok` | bool | Always `true` on success |
| `foundry_version` | string | Package version |
| `catalog_digest` | string | Closed catalog digest (stub until MS-007) |
| `plan_sha256` | string | 64 hex; integrity over semantic content |
| `verify` | string | `none` \| `default` \| `strict` |
| `destination_policy` | string | `missing` \| `empty_admissible` \| `refuse:…` |
| `normalized_spec` | object | See below |
| `composition` | object | See below |
| `planned_files` | array | `{ path, mode, content_digest }` |
| `dependency_deltas` | array | `{ name, version_req, features, dev }` |
| `ai_native_paths` | array of string | e.g. `AGENTS.md` |
| `warnings` | array of string | Non-binding |

### `normalized_spec`

| Field | Type |
| ----- | ---- |
| `schema` | number |
| `name` | string |
| `description` | string or null |
| `archetype` | string |
| `destination` | string |
| `profiles` | array of string |
| `verify` | string |
| `source` | string |

### `composition`

| Field | Type |
| ----- | ---- |
| `archetype` | string |
| `ordered_profiles` | array of string (canonical order) |
| `unit_ids` | array of string (`core` + archetype + profiles) |

### `planned_files[]`

| Field | Type |
| ----- | ---- |
| `path` | string (relative, jail-safe) |
| `mode` | string (`file` \| `executable` \| `directory`) |
| `content_digest` | string (64 hex SHA-256 of planned bytes) |

## Error object

```json
{
  "ok": false,
  "error": {
    "code": "plan.path_jail",
    "message": "…"
  }
}
```

See also [`error-codes.md`](error-codes.md).

## Snapshot redaction policy

For **insta** goldens only (`tests/plan_snapshots.rs`):

| Live field | Snapshot placeholder |
| ---------- | -------------------- |
| `foundry_version` | `<FOUNDRY_VERSION>` |
| `catalog_digest` | `<CATALOG_DIGEST>` |
| `plan_sha256` | `<PLAN_SHA256>` |

`content_digest` values on planned files are **not** redacted (they are the
emit contract). After intentional plan/catalog changes: `cargo insta review`.

## Update path

```bash
cargo test --test plan_snapshots
cargo insta review   # accept/reject diffs
# or: cargo insta accept
```
