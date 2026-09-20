# UKD Knowledge Node schema

Taxonomy is data. Not a live graph store.
Crate already on main: `gaia-ukd::{REALMS, KnowledgeNode, get_node, list_realms}`.
Issue this slice: #513 / #83. License on these files: CC0-1.0 as the issue asks.

## Fields that exist on `KnowledgeNode`

id, realm, citation.
Blueprint names (aliases, relations, learning, traditions, license) are **not** struct fields on main. Do not invent them here.

## Seed examples that validate

- `ukd:math:linear-algebra` — realm `mathematics`, citation `fixture:open-text`
- `ukd:eng:quantum-computing` — realm `engineering`, citation `fixture:open-text`

The issue named a quantum-mechanics example. The crate seed is **quantum-computing**, not quantum-mechanics. Do not add a fake mechanics node in this slice.

## Realms

`list_realms()` / `REALMS` = 12, including `traditional-knowledge`.
Domain stubs are realm names. Subject rows stay empty except the two seeds.

SKOS / OWL alignment is a note, not an ontology file in this PR.

## Refuse

- live SPARQL / Neo4j
- uncited node
- UKD v1.0
