# UKD ingest — fixtures

Not a Wikidata clone. Not a 15k-topic dump.
Crate already on main: `gaia-ukd::Ingested`.
Issue this slice: #557 / #85.

## What exists

`Ingested::physics_subject()` — subject physics, Q413, Wikipedia sitelink, CC-BY-SA license.
`Ingested::okg_prerequisite()` — linear-algebra, relation `PREREQUISITE_OF`.
`admit()` fails if license is empty (`Uncited`).

That meets the *shape* of the acceptance. It does not pull SPARQL.

## Refuse

- 16B triple clone
- empty license field
- live SPARQL write
