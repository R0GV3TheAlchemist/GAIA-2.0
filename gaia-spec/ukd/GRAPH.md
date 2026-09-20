# UKD graph API — in-process

Not Neo4j. Not a SPARQL or GraphQL server.
Crate already on main: `gaia-ukd::{GraphApi, Edge}`.
Issue this slice: #555 / #84.

## What exists

`GraphApi::realm_list` — 12 names via `list_realms`.
`write` requires `signed_write`. Otherwise `Uncited`.
`jsonld_sample` is a fixture string with `@id` and a license. Not a validated RDF dump product.

Compose / AGE / live SPARQL stay out.

## Refuse

- Neo4j Community deploy
- unsigned graph write
- UKD v1.0
