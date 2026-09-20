# UKD Phase 0 — taxonomy + local graph stubs

Name the graph. Do not stand up Neo4j.
Crate already on main: `gaia-ukd::{list_realms, get_node, federated_wikidata, GraphApi}`.
Schema already listed under #513 / #83. Issue this slice: #545 / #78.

## What exists

`list_realms()` — 12 names.
`get_node("ukd:math:linear-algebra")` is the seeded fixture. Unknown id → `UnknownNode`.
`federated_wikidata(true)` → `TekSealed` (cannot write TEK from a federated query).
`GraphApi` is in-process. Not a SPARQL endpoint.

JSON-LD, MediaWiki, and a 15k-topic dump are **not** imported here.

## Refuse

- Neo4j / Apache AGE product
- live SPARQL write
- UKD v1.0
