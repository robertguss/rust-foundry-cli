# Troubleshooting and error codes

This page lists common problems, their error codes, and how to fix them.

## Stable error codes

Foundry prints error messages in the form `error[<code>]: ...`. The codes are stable so scripts and agents can match them.

### Spec errors

| Code | What it means | How to fix |
| ---- | ------------- | ---------- |
| `spec.parse` | The TOML file could not be read or decoded. | Check the file path and ensure the file is valid TOML. |
| `spec.unknown_key` | The spec contains a top-level key Foundry does not recognize. | Remove the unknown key. Only valid Project Spec fields are allowed. |
| `spec.missing_field` | A required field is missing. | Add the missing field. See [Project Spec reference](project-spec.md). |
| `spec.unsupported_schema` | The `schema` value is not supported. | Use `schema = 1` for this release. |
| `spec.schema_type` / `spec.field_type` | A field has the wrong type. | Check that each field is a string, number, or array as expected. |
| `spec.empty_field` | `name` or `destination` is empty. | Provide non-empty values. |
| `spec.unknown_archetype` | The `archetype` value is not supported. | Use `archetype = "cli"` in v1. |
| `spec.unknown_profile` | A profile id in `profiles` is not in the catalog. | Use only known profile ids. Run `foundry catalog list` to see them. |
| `spec.duplicate_profile` | The same profile id appears more than once. | Remove the duplicate. |
| `spec.verify_mode` | The `verify` value is invalid. | Use `none`, `default`, or `strict`. |
| `spec.secret_field` | A forbidden secret-related field name was used. | Do not name fields `password`, `secret`, `token`, `api_key`, `private_key`, `access_key`, or `client_secret`. |

### Resolve errors

| Code | What it means | How to fix |
| ---- | ------------- | ---------- |
| `resolve.unknown_archetype` | The archetype could not be resolved. | Use `archetype = "cli"`. |
| `resolve.unknown_profile` | A profile could not be resolved. | Check the profile id against `foundry catalog list`. |

### Plan and generate errors

| Code | What it means | How to fix |
| ---- | ------------- | ---------- |
| `plan.path_jail` | A planned file path would escape the project root. | Check your `destination` and `name` for absolute paths, `..`, or symlink escapes. |
| `plan.incomplete` | The plan is missing a required element. | This is an internal error; please report it. |

### Report I/O errors

| Code | What it means | How to fix |
| ---- | ------------- | ---------- |
| `report.write` | Foundry could not write the `--out` file for a plan. | Check that the output directory exists and is writable. |

## Common problems

### `foundry generate` says the destination is not empty

Foundry refuses to place a project into a directory that already contains files. This is the default behavior and cannot be overridden in v1.

**Fix one of the following:**

- Remove the destination directory: `rm -rf ./my-app`
- Use a different destination: `--dest ./my-app-2`
- Move the old directory out of the way

### `foundry plan` shows a warning about `destination_policy=missing`

This is a non-binding warning. It means the destination path does not exist yet, which is fine for `plan` because `plan` does not write files. `generate` will re-check the destination before placing anything.

### Verification fails during `generate`

If `verify` is `default` or `strict`, Foundry runs the generated project's quality gate after staging. If it fails, Foundry does not place the files and prints the staging path.

**Common causes:**

- Generated code does not compile on your system (check `rustc` version).
- Clippy denies a warning that appears in your environment.
- A test fails due to a system dependency.

**Fix:**

1. Run the same commands manually in the staging directory.
2. Fix the underlying issue, or use `--verify none` to skip verification for a one-off inspection.

### Unknown profile id

Run `foundry catalog list` to see valid profile ids. In v1 the valid ids are `tui`, `hooks`, `secrets`, and `distribution`.

### `cargo` is not found

Foundry does not install Rust for you. Make sure `cargo` and `rustc` are on your `PATH` and meet the minimum version in the [Installation guide](install.md).

## Getting more help

- Run `foundry --help` or `foundry <COMMAND> --help` for command-specific help.
- Check the [Command reference](commands.md) on this site.
- Inspect the plan with `foundry plan --format json` to see the exact inputs and outputs before generating.
