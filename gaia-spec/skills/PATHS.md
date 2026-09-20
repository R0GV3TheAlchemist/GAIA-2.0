# Skill paths + UKD links

Practice is required; a textbook alone is not enough.
Crate already on main: `gaia-skills::{novice_public_speaking, develop}`.
Issue this slice: #493 / #116.

## Practice steps

`develop(goal)` emits steps with `kind: practice` and open resource URLs already in the fixture (Gutenberg, OCW).
`novice_public_speaking()` is a three-step fixture with `skill:` ids and open resources.

## UKD gaps

`requires_knowledge` uses `ukd:` ids (not skill ids).
`develop(...).ukd_gaps` surfaces missing knowledge as UKD gaps.
`used_network` stays false on the solo path.

Time-to-competence bands are **estimates, not promises**. This slice does not add a band field.

## Refuse

- reading-only path
- live OER scrape
- competence-time guarantee
