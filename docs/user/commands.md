# Command reference

All Foundry commands are non-interactive. They exit `0` on success and `1` on failure.

## Global options

| Option | Description |
| ------ | ----------- |
| `-h, --help` | Show help. |
| `-V, --version` | Show version. |

## `foundry version`

Print the Foundry version and catalog digest.

```bash
foundry version
```

The catalog digest is a fingerprint of the embedded catalog. Two builds with the same digest produce the same planned files for the same spec.

## `foundry validate`

Validate a Project Spec without writing anything to disk.

```bash
foundry validate --spec PATH
```

Options:

| Option | Description |
| ------ | ----------- |
| `--spec PATH` | Path to the Project Spec, or `-` for stdin. Required. |
| `--name NAME` | Override the `name` field. |
| `--dest PATH` | Override the `destination` field. |
| `--verify none\|default\|strict` | Override the `verify` field. |

Use this command to check your spec before planning or generating.

## `foundry plan`

Emit the Generation Plan for a spec. This is a dry run that shows what `generate` would do.

```bash
foundry plan --spec PATH
```

Options:

| Option | Description |
| ------ | ----------- |
| `--spec PATH` | Path to the Project Spec, or `-` for stdin. Required. |
| `--name NAME` | Override the `name` field. |
| `--dest PATH` | Override the `destination` field. |
| `--verify none\|default\|strict` | Override the `verify` field. |
| `--format text\|json` | Output format. Default is `text`. |
| `--out FILE` | Write the plan report to FILE. The destination directory is not touched. |

The plan output includes:

- The resolved composition (archetype + ordered profiles)
- The list of files that will be generated
- The dependencies that will be added
- Any warnings

## `foundry generate`

Generate the project.

```bash
foundry generate --spec PATH
```

Options:

| Option | Description |
| ------ | ----------- |
| `--spec PATH` | Path to the Project Spec, or `-` for stdin. Required. |
| `--name NAME` | Override the `name` field. |
| `--dest PATH` | Override the `destination` field. |
| `--verify none\|default\|strict` | Override the `verify` field. |

Generation follows these steps:

1. Validate the spec.
2. Resolve the archetype and profiles.
3. Construct an immutable plan.
4. Render the plan into a temporary staging directory.
5. Run the requested verification tier.
6. Move the staged files into the destination directory.

The destination must be missing or an empty directory. Foundry refuses to merge into an existing, non-empty directory.

If generation fails after staging, Foundry prints the path of the staging directory so you can inspect it.

## `foundry catalog`

Inspect the closed embedded catalog. This works offline.

### `foundry catalog list`

List all catalog unit ids.

```bash
foundry catalog list
```

Output includes each unit id, its kind (`core`, `archetype`, `profile`), its requirements, and the catalog digest.

### `foundry catalog show <ID>`

Show details of a single catalog unit.

```bash
foundry catalog show cli
foundry catalog show tui
```

The supported ids are `core`, `cli`, `tui`, `hooks`, `secrets`, and `distribution`.

## `foundry sample-spec`

Emit a sample Project Spec to stdout.

```bash
foundry sample-spec
foundry sample-spec --name my-app --profile tui
```

Options:

| Option | Description |
| ------ | ----------- |
| `--name NAME` | Project name in the sample. Default is `example-cli`. |
| `--profile ID` | Optional profile ids to include. Can be given multiple times. |

The output is a valid TOML file. Save it and edit it to your needs:

```bash
foundry sample-spec --name my-app --profile tui --profile hooks > my-app.toml
```

## Exit codes

| Exit code | Meaning |
| --------- | ------- |
| `0` | Success. |
| `1` | Failure: bad spec, unknown profile, path error, verification failure, destination not empty, or other runtime error. |

For specific error codes and how to fix them, see [Troubleshooting](troubleshooting.md).
