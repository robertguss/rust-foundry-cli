# plan-generate (product skill)

Foundry product skill — **not** emitted into Generated Projects.

```text
foundry plan --spec PATH
foundry generate --spec PATH
```

Primary gate strings (must match freeze fixtures):

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
