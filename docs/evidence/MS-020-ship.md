# MS-020 ship freeze

## Release checklist
- [x] catalog digest frozen: docs/freeze/catalog-digest.txt
- [x] plan goldens: tests/snapshots/plan/
- [x] command surface freeze: docs/freeze/command-surface-*.txt
- [x] GHA freeze: docs/freeze/gha-core-ci.yml
- [x] install: `cargo install --path .` / `cargo run --`

## Owner ship decision
**Ship-ready for v0.1 scaffold** with residuals:
- Full default verify warm-cache CI cost (SPK-103)
- cargo-dist release automation is scaffold-only
- REQ-152 agent trial: residual — not run in this delivery; document for owners

## Residual ledger
| Item | Disposition |
| ---- | ----------- |
| REQ-152 agent operability trial | Residual — optional Should; owners run manually |
| cargo-dist full release | Scaffold only |
| Windows | Forbidden forever |
