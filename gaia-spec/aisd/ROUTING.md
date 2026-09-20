# AISD routing + v1.0 gate

Orchestrator *would* consume AISD. This tree does not ship a live orchestrator.
Crate already on main: `gaia-aisd::{dispatch, capability_copy, release_gaps, aisd_v1_tagged, ask_aisd}`.
Issue this slice: #487 / #131.

## Dispatch log

`dispatch(skill_id, maturity)` requires a skill id. Missing id → `InsufficientMaturity`.
`CallLog` records skill id + maturity at call time. That is the agent-log acceptance. Not a running agent.

## Copy / GAIAN card

`capability_copy` refuses the string "autonomous AGI" (case-insensitive) → `Unmeasured`.
GAIAN cards stay generated from *measured* cards only. Unmeasured stays a limitation string (`AiSkill::unmeasured`).
No self-awarded superhuman. Proposal queue is not an API on main — do not invent one.

## Release notes / gaps

`release_gaps()` includes swe-pro-engineering, calibration, and `no AISD v1.0`.
`aisd_v1_tagged()` stays false. Taxonomy / tools / ban list exist as listed files; that is not a v1.0 release.

`ask_aisd` refuses claimed L5+ when actual ≤ L2.

## Refuse

- AISD v1.0 tag
- "autonomous AGI" UI copy
- tool call without skill id
- live orchestrator
