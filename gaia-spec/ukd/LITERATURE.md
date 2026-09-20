# UKD literature + levels — fixtures

One paper. One skill. Same concept at two levels. Local state.
Crate already on main: `gaia-ukd::{LicensedNode, present, KnowledgeState, KnowledgeLevel}`.
Issue this slice: #563 / #89.

## What exists

`LicensedNode::paper()` / `skill()` — nonempty license.
`present(concept, Sprout)` vs `present(concept, Master)` — different strings.
`KnowledgeState::local()` — `sync == false`. `mark_learned` moves a step into `known`.

PubMed / arXiv / DOAJ / WikiHow are **not** connectors here.

## Refuse

- live literature ingest
- cloud-sync on by default
