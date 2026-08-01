# SPK-104 / MS-015.1 — GH template snapshot regen (catalog SoT)

## Source of truth

`catalog/units/cli/templates/` (embedded at compile time).

## Frozen snapshot

`docs/freeze/gh-template/` — pure-CLI GitHub template surface checked into CI.

| Snapshot path | Catalog template |
| ------------- | ---------------- |
| `docs/freeze/gh-template/.github/workflows/ci.yml` | `catalog/units/cli/templates/ci.yml` |
| `docs/freeze/catalog-digest.txt` | live `foundry version` catalog digest |

## Regen path (local / CI)

```bash
./scripts/regen-gh-template-snapshot.sh
cargo test --test spk104_gh_template
git add docs/freeze/
```

## CI gate

`cargo test --test spk104_gh_template` fails if catalog and snapshot diverge
(no dual-edit SoT).
