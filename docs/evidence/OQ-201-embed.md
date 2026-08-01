# OQ-201 — Catalog embed mechanism decision

**Decision:** use the **`include_dir`** crate (`include_dir!` macro) to embed the
`catalog/` authoring tree at compile time.

**Alternatives considered:**

| Option | Pros | Cons |
| ------ | ---- | ---- |
| `include_dir` | Whole-tree embed; simple API; offline | Extra dep |
| `rust-embed` | Popular | Heavier feature surface |
| Hand-rolled `include_str!` | Zero deps | Brittle as catalog grows |

**Non-load-bearing:** choice does not change REQ-060 offline catalog semantics
or plan/construct signatures. Digest is SHA-256 over canonical unit paths+bytes.

**Date:** 2026-08-01
