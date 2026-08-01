# Installation

Foundry is currently distributed as source. You build the `foundry` binary with `cargo`.

## Requirements

- macOS or Linux. Foundry does not support Windows in v1.
- Rust toolchain 1.85 or newer.
- `git` if you are cloning the repository.

## Build from source

1. Clone the repository:

   ```bash
   git clone https://github.com/robertguss/rust-foundry-cli.git
   cd rust-foundry-cli
   ```

2. Build the release binary:

   ```bash
   cargo build --release
   ```

3. The binary is at `target/release/foundry`. You can add it to your `PATH` by copying it to a directory already on your path, for example:

   ```bash
   cp target/release/foundry ~/.local/bin/
   ```

   Or use it directly from the build tree:

   ```bash
   ./target/release/foundry --help
   ```

## Run without installing

If you do not want to copy the binary anywhere, use `cargo run` from the repository root:

```bash
cargo run -- --help
cargo run -- validate --spec examples/minimal-cli.toml
```

`--` separates `cargo run` arguments from `foundry` arguments.

## Verify the install

Run:

```bash
foundry version
```

You should see the Foundry version and the catalog digest.
