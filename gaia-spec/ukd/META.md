# UKD META — listed bind

Multiple epistemologies. Knowledge is a commons. TEK empty until grant (#88).
Crate already on main: `gaia-ukd`.
Issue this slice: #507 / #77. Children #78–#92 stay open.

## Rules already on main

- `REALMS` has 12 names including `traditional-knowledge`.
- Node ids use `ukd:` — not `skill:` and not magic ids.
- `get_node` returns seeded cited fixtures only.
- `federated_wikidata(write_tek=true)` → `TekSealed`.
- `tek_export(false)` → `NoAgreement`.
- `ukd_v1_tagged()` → false.
- Do not merge Skill Nodes or Magic Nodes into Knowledge Nodes.
- No uncited LLM nodes (`UkdError::Uncited` exists on contribute).

## Refuse

- live Wikidata write / Neo4j product
- TEK scrape
- UKD v1.0 tag
