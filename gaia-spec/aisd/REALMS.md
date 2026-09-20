# AISD 13 realms + explicit gaps

Honest gap map is part of the product. Not a UI.
Crate already on main: `gaia-aisd::{REALMS, REALM_COUNT, realm_stubs, gap_nodes, protein_structure}`.
Issue this slice: #481 / #128.

## Realms (navigable as names)

`REALMS` / `REALM_COUNT` = 13. `realm_stubs()` returns `{realm}:skill-stub` for each.
That is navigation in this tree. No live catalog UI.

## Seed gaps (`gap_nodes`)

long-horizon-cot, swe-pro-engineering, calibration, causal, theory-of-mind, continual-learning, genuine-novelty, dexterous-embodiment.
Queryable as that array. UI copy guideline: show the gap *name*, do not claim it is closed.

## Limitations on L4+

`AiSkill.limitations` exists. `protein_structure()` and `AiSkill::unmeasured` already carry limitation strings. Unmeasured cannot be L5 (`assign_maturity`).

## Refuse

- live router / trainer / forecast ingest
- closed gap as a shipped skill
- AISD v1.0
