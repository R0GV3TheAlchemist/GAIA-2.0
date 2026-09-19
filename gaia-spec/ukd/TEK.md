# UKD TEK + knowledge-sovereignty protocol

Realm 11 is not a scrape target. Consent before ingest.
Crate already on main: `gaia-ukd::{TekGraph, TekStore, publish_tek, tek_export, ukd_v1_tagged}`.
Same grant/withdrawal idea as HMGD #161 and Skills #120 — no parallel scrape.
Issue this slice: #473 / #88.

## Collection states (`CollectionState`)

| state | public list |
| --- | --- |
| Sealed (default) | `list_public` → `TekSealed` |
| CommunityGoverned | only with a grant row |
| PublicWithGrant | only with a grant row |

`TekGraph::new()` and `TekStore::new()` start empty.

## Agreement record (fields already on `Agreement`)

community, scope, benefit_sharing, withdrawal.
`publish_tek(false, _)` and `tek_export(false)` → `NoAgreement`.
`ingest_wipo()` → `NoAgreement` (no bulk import).

## Withdrawal

`load_fixture` then `withdraw` clears replicas and returns the graph to Sealed.
That is the tested withdrawal path. It is not a live replica network.

## CARE

Collective benefit, Authority to control, Responsibility, Ethics — named principles only.
Sacred / sealed knowledge stays out of public list.

## Refuse

- scrape ceremonial or country corpora
- WIPO/NPS bulk import without an agreement row
- list sealed collections
- UKD v1.0 / live SPARQL
