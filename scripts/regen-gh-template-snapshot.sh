#!/usr/bin/env bash
# Regenerate pure-CLI GH template snapshot from embedded catalog SoT (MS-015.1 / SPK-104).
# Single source of truth: catalog/units/cli/templates/*
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/docs/freeze/gh-template"
mkdir -p "$OUT/.github/workflows"
cp "$ROOT/catalog/units/cli/templates/ci.yml" "$OUT/.github/workflows/ci.yml"
cp "$ROOT/catalog/units/cli/templates/justfile" "$OUT/justfile.snippet"
# Refresh catalog digest freeze
cargo run -q --manifest-path "$ROOT/Cargo.toml" -- version | awk '/catalog_digest:/{print $2}' > "$ROOT/docs/freeze/catalog-digest.txt"
echo "Regenerated $OUT and catalog-digest.txt"
echo "Run: cargo test --test spk104_gh_template"
