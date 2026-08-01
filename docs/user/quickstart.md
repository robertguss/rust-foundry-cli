# Quickstart

This page walks through creating your first Rust CLI project with Foundry.

## 1. Get a sample Project Spec

Foundry can emit a valid starter spec:

```bash
foundry sample-spec --name my-app > my-app.toml
```

This creates a TOML file with the minimum required fields.

## 2. Inspect the spec

Open `my-app.toml`. It looks like this:

```toml
schema = 1
name = "my-app"
description = "Sample pure-CLI project"
archetype = "cli"
destination = "./my-app"
profiles = []
# verify = "default"
```

The `destination` is where Foundry will write the generated project. It must be missing or empty before you generate.

## 3. Validate the spec

Validate checks that the spec is well-formed without writing anything:

```bash
foundry validate --spec my-app.toml
```

If validation passes, you will see a summary.

## 4. Preview the Generation Plan

The plan shows what Foundry will write, before it writes anything:

```bash
foundry plan --spec my-app.toml
```

Review the output. It lists the files, dependencies, and any warnings.

For machine-readable output, use:

```bash
foundry plan --spec my-app.toml --format json --out plan.json
```

## 5. Generate the project

Make sure `my-app` (the destination) does not yet exist, or is an empty directory. Then run:

```bash
foundry generate --spec my-app.toml
```

Foundry builds the project in a staging area, runs the default verification tier, and then places the result in `my-app`.

## 6. Build and test the generated project

```bash
cd my-app
cargo build
cargo test
```

You can also run the generated CLI:

```bash
cargo run -- --help
```

## Next steps

- Read the [Project Spec reference](project-spec.md) to learn about profiles and overrides.
- Read [Using a generated project](generated-project.md) for day-to-day development.
- Read [Troubleshooting](troubleshooting.md) if anything goes wrong.
