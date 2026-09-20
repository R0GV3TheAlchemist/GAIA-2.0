# ESCO / O*NET lookup

Query by id. Do not block on a 19k merge.
Crate already on main: `gaia-skills::SkillGraph`.
Issue this slice: #505 / #114.

## What exists

- `SkillGraph::seed()` holds `active_listening()` (esco/onet `None`).
- `by_esco` / `by_onet` return `UnknownSkill` when the id is absent.
- `unmapped()` returns nodes with both ids empty.

Label `Skill` is this crate. UKD `KnowledgeNode` stays on the UKD side. Do not reuse UKD ids as skill ids.

License on the seed node is `CC0-1.0`. Provenance for a future official crosswalk is not an ingest job here.

## Refuse

- bulk ESCO / O*NET import
- Neo4j / live graph store
- drop unmapped GAIA skills
