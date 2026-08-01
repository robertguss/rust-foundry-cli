# foundry-quality-gates (product skill)

Before close:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Matches Generated justfile / GHA freeze fixtures (REQ-088).
