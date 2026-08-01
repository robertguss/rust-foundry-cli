# Implementation authority

This product repository implements **rust-foundry**. Research and
methodology live in a separate repository; **do not** treat research chat
history or this README as product law.

## Authoritative artifacts (in this repo)

| Role | Document | Source (research repo) | Source commit |
| ---- | -------- | ---------------------- | ------------- |
| **Product law** | [`02-definitive-specification-revised.md`](02-definitive-specification-revised.md) | `docs/specifications/02-definitive-specification-revised.md` | `bf8b0f4d9ac4d5e8276100c9bb3af3438afc4443` |
| **Delivery sequence** | [`02-implementation-plan-revised.md`](02-implementation-plan-revised.md) | `docs/plans/02-implementation-plan-revised.md` | `9d74790259d1d942e850817e2d997ff8b5490aa4` |
| Locks / non-goals (context) | [`00-program-blueprint.md`](00-program-blueprint.md) | `docs/00-program-blueprint.md` | `e2a16cfab1097956731e147ed405ff3c4035281e` |

Research repository: https://github.com/robertguss/rust-foundry  
(local sibling: `../rust-foundry`)

## Precedence

1. Accepted `DEC-###` (if any are added under `decisions/` in this product)
2. Blueprint locks and non-goals
3. **Revised definitive specification** (product law)
4. **Revised implementation plan** (delivery sequence / phases)
5. This product's `AGENTS.md` and README (workflow only; never override REQs)

## Refreshing authority copies

If the research program amends the revised specification or plan:

1. Update the source commits above.
2. Replace the copied Markdown files in `docs/`.
3. Note residual product impact in the commit message.

Do not invent REQs or reverse locks in this product without a formal DEC and
upstream research authority change.
