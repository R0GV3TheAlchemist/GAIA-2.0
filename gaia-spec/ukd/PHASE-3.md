# UKD Phase 3 — cited synthesis

Every generated claim has sources. New triples wait in review.
Crate already on main: `gaia-ukd::{Claim, ReviewQueue, NodeDraft, GraphView}`.
Issue this slice: #551 / #81.

## What exists

`Claim::synthesize(_, [])` → `Uncited`.
`ReviewQueue::auto_publish` → `Uncited` (not the canonical graph).
`NodeDraft::save` also requires sources.
`GraphView::query_canonical` fails until `accept`.

No live arXiv watch. No silent hallucination path.

## Refuse

- uncited synthesis
- auto-publish to canonical graph
- UKD v1.0
