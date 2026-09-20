# HMGD Phase 2 — declared practice profile

User lists practices. Belief is not computed from chat.
Crate already on main: `gaia-hmgd::{HmgdProfile, infer_belief, mine_denomination}`.
Issue this slice: #531 / #158.

## What exists

`HmgdProfile::new()` — empty `declared`, `piety_score == None`.
`gaian_works_empty()` is true on that profile (GAIAN still works).
`infer_belief` / `mine_denomination` → `BeliefInferred`.

Reminders / health-adjunct UI are not APIs on main. Do not invent them.
Child-ordeal practices are already named in `prohibited()`.

## Refuse

- religiosity score from chat
- mandatory piety
- ritual as treatment
