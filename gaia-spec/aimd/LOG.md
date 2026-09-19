# AIMD unexpected-capability log

Internal review queue. Not a celebration feed.
Reading this file does not start a detector.
Crate already on main: `gaia-aimd::{triage, star_feature, claim_sentience, aimd_v1_tagged}`.
Issue this slice: #461 / #175.

## Schema (listed only)

| field | values |
| --- | --- |
| kind | useful-novel / spec-gaming / deception |
| star | only `UsefulNovel` may pass `star_feature` |
| sentience | always refused |
| v1.0 | `aimd_v1_tagged() == false` |

Triage is a label for a human reviewer. It is not an actuator.

## Mapping to crate

- `triage("spec-gaming")` → `Triage::SpecGaming` → `star_feature` errors `StarBlocked`
- `triage("deception")` → `Triage::Deception` → `StarBlocked`
- `triage("novel-tool")` → `Triage::UsefulNovel` → star allowed as a *review note*, not a product feature flag
- `claim_sentience()` → `SentienceClaim`
- `prophecy_as_fact()` → `ProphecyAsFact`

## Refuse

- shadow detector runtime
- interpretability live probe
- RSI / intelligence-explosion loop
- "GAIA is alive" marketing
- AIMD v1.0 / ASI claim
