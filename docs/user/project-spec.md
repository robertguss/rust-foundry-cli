# Project Spec reference

A **Project Spec** is the TOML file that tells Foundry what to generate. It is the only input you need for `validate`, `plan`, and `generate`.

## Example

```toml
# Minimal Project Spec for schema 1
schema = 1
name = "my-app"
description = "My first Foundry CLI"
archetype = "cli"
destination = "./my-app"
profiles = ["tui", "hooks"]
verify = "default"
```

## Fields

| Field | Required? | Description |
| ----- | --------- | ----------- |
| `schema` | Yes | Schema version. Only `1` is supported in this release. |
| `name` | Yes | The crate and project name. Should be a valid Rust crate name. |
| `description` | No | A short human-readable description. |
| `archetype` | Yes | Project archetype. Only `cli` is supported in v1. |
| `destination` | Yes | Directory where Foundry places the project. It should be empty or not yet exist. |
| `profiles` | Yes | Array of profile ids from the catalog. May be `[]`. |
| `verify` | No | Verification tier: `none`, `default`, or `strict`. Defaults to `default`. |

### Notes

- Unknown top-level keys cause `validate` (and therefore `plan`/`generate`) to fail.
- Do not put secret values in the spec. Field names such as `password`, `secret`, `token`, `api_key`, `private_key`, `access_key`, and `client_secret` are rejected.
- You can read the spec from a file with `--spec PATH` or from stdin with `--spec -`.

## Profiles

Profiles add optional capabilities to the generated project. Add them to the `profiles` array:

```toml
profiles = ["tui", "hooks"]
```

The order in the array does not matter. Foundry applies profiles in a canonical order regardless of how you list them.

See [Catalog and profiles](catalog.md) for what each profile adds.

## Verification modes

| Mode | What it does |
| ---- | ------------ |
| `none` | Do not run any post-stage verification. |
| `default` | Run the primary quality gate (format, clippy, test). This is the default. |
| `strict` | Run a more extensive verification tier when available. |

For most users, `default` is the right choice. Use `none` only when you want to inspect a partial or experimental generation.

## CLI overrides

You can override three spec fields from the command line:

```bash
foundry validate --spec my-app.toml --name other-name --dest ./other-dest --verify strict
```

| Flag | Overrides | Commands that accept it |
| ---- | --------- | ----------------------- |
| `--name NAME` | `name` | `validate`, `plan`, `generate` |
| `--dest PATH` | `destination` | `validate`, `plan`, `generate` |
| `--verify none\|default\|strict` | `verify` | `validate`, `plan`, `generate` |

When a CLI flag is provided, it takes precedence over the value in the TOML file. This is useful in scripts and CI where you want to reuse the same spec with a different destination.

## Tips

- Keep the destination directory name similar to the project `name` to avoid confusion.
- Use `profiles = []` for the smallest possible pure-CLI project.
- Check the plan with `foundry plan` before generating. It lets you catch mistakes without writing files.
