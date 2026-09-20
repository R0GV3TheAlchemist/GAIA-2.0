# HMGD Phase 3 — opt-in + v1.0 gate

Collective practice is opt-in. Not a global prayer experiment.
Crate already on main: `gaia-hmgd::{join_room, sell_closed_rite, songlines, sacred_layer, hmgd_v1_tagged}`.
Issue this slice: #537 / #159.

## Rooms

`join_room(false)` → `enrolled == false`.
`join_room(true)` → enrolled. That is the join flag. Not a live session server.
`sell_closed_rite()` → `SaleForbidden`.
`songlines()` → `Sealed`.
`sacred_layer(false)` → `NotOptIn`.

## Gate

`hmgd_v1_tagged()` stays **false**.
A future v1.0 note would name: catalog, evidence policy, TEK policy, non-claims. That note is not a tag.

## Refuse

- enroll outsiders
- monetized closed-rite pack
- HMGD v1.0 tag
