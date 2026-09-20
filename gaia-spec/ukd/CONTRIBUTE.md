# UKD contribution flow

LLM helps. The canonical graph stays curated.
Crate already on main: `gaia-ukd::{NodeDraft, GraphView}`.
Cited synthesis already listed under Phase 3. Issue this slice: #565 / #90.

## What exists

`NodeDraft::save(_, [], _)` → `Uncited`.
`GraphView::propose` does not make `query_canonical` succeed.
`accept` moves the edge; then query works.
No live arXiv frontier watch.

## Refuse

- uncited model text as a node
- proposed edge treated as canonical
