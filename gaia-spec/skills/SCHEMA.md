# Skill Node schema

Skills must not reuse UKD concept ids.
Crate already on main: `gaia-skills::{SkillNode, active_listening, REALMS, realm_stubs}`.
Issue this slice: #491 / #113.

## Fields on `SkillNode` (what exists)

id, realm, dreyfus, esco, onet, license.
Other blueprint names (dimensions, transferability, wef_category, cultural_variations, ai_replaceability) are **not** struct fields on main — do not invent them here.

## Example

`active_listening()`:
- id `skill:active-listening` (not a UKD id)
- realm `communication`
- dreyfus `novice`
- esco / onet empty (`None`) — unmapped skills still exist
- license `CC0-1.0`

## Realms

`REALMS` has 12 names including `digital`.
`realm_stubs()` returns one domain stub per realm.

Relation types (prerequisites, enables, related, requires_knowledge) are names only. No live graph store in this slice.

## Refuse

- live ESCO / O*NET ingest (#114 stays separate)
- UKD concept id reused as a skill id
- skills v1.0
