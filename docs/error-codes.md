# Stable error codes (agent switch surface)

Machine-oriented codes appear as `error[<code>]: …` on stderr (text) and as
`error.code` in JSON plan failure reports (`report::format_error_json`).

Codes are **stable strings** agents may match on. Expanding the set is allowed;
renaming existing codes requires a DEC.

## Spec / normalize (REQ-030..034)

| Code | When |
| ---- | ---- |
| `spec.parse` | TOML decode / read failure (and related parse codes) |
| `spec.unknown_key` | Unknown top-level Project Spec key |
| `spec.missing_field` | Required field absent |
| `spec.unsupported_schema` | `schema` not in supported set |
| `spec.schema_type` | `schema` wrong type |
| `spec.field_type` | Field wrong type |
| `spec.empty_field` | Empty name/destination (TOML or CLI override) |
| `spec.unknown_archetype` | Archetype not in closed set (`cli` only v1) |
| `spec.unknown_profile` | Profile id not in closed set |
| `spec.duplicate_profile` | Duplicate profile id |
| `spec.profiles_type` / `spec.profile_type` | Profiles array / element type |
| `spec.verify_mode` | Invalid verify mode string |
| `spec.secret_field` | Secret field-name denylist hit (REQ-033) |

## Resolve (MS-003.2)

| Code | When |
| ---- | ---- |
| `resolve.unknown_archetype` | Defense-in-depth non-cli archetype |
| `resolve.unknown_profile` | Defense-in-depth unknown profile |

## Plan / Construct (REQ-040, REQ-053)

| Code | When |
| ---- | ---- |
| `plan.path_jail` | Absolute or `..` escape in planned path |
| `plan.incomplete` | Plan missing required REQ-041 element after seal |

## Report I/O

| Code | When |
| ---- | ---- |
| `report.write` | Cannot write `--out` file |

## JSON error shape (plan `--format json` failures)

```json
{
  "ok": false,
  "error": {
    "code": "plan.path_jail",
    "message": "…"
  }
}
```

Spec load failures currently print text `error[code]: message` on stderr for all
commands (including plan). Construct failures under `--format json` use the JSON
shape on stderr.
