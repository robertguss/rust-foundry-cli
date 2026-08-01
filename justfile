# Foundry product convenience gates (dogfood surface expands later).

default:
    @just --list

# Primary local quality gate (align with CI).
ci: fmt lint test

fmt:
    cargo fmt --check

lint:
    cargo clippy --all-targets -- -D warnings

test:
    cargo test

build:
    cargo build

version:
    cargo run --quiet -- version
