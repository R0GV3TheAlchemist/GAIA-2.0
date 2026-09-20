# Personalized skill development paths

Phase 2 can be local-only recommendations. Community match is optional later.
Crate already on main: `gaia-skills::{develop, VaultProfile, SkillCard}`.
Issue this slice: #497 / #118.

## Solo path

`develop(goal)` returns practice steps + UKD gaps.
`used_network == false` — no network call required.
Changing the goal string yields a different `DevPath.goal`.
`VaultProfile::set_goal` updates the local profile the path would read.

Inputs named in the issue (time, culture, stage) are **not** function arguments on main. Do not invent them.

## Community opt-in

`SkillCard::search` returns only `published` cards. That is local "others learning X".
`rank_in_marketplace` still blocks age &lt; 18.

## Refuse

- network required for solo path
- unpublished cards in match
- child marketplace rank
