# AISD META — listed bind

Universal AI Skills Database: executable capability names, not stored facts.
Crate already on main: `gaia-aisd`. Taxonomy listed in #455 / #126.
Issue this slice: #477 / #121. Children #122–#131 stay open.

## Rules already on main

- GAIA scores are measured. Published Elo/MMLU stay `reference_published`.
- Level 6 is banned on high-stakes domains: `high_stakes_level6()` → `HighStakesLevel6`.
- Safety skills gate tool use (`policy::allow`). This slice does not add a runtime router.
- `aisd_v1_tagged()` → false.

## Tools vs model cards

`graphcast()` is a weather-forecast *tool*, not an LLM card (`is_llm_card == false`).
`reference_published(name, value)` leaves `gaia_measured` empty.
No live GraphCast ingest.

AISPD (#144 / #151) is the extreme tail. Do not duplicate every skill as a superpower.

## Phases (tracking only)

| Phase | Issue | listed status |
| --- | --- | --- |
| 0 Taxonomy | #126 | listed #455 |
| 0 Tools | #127 | tool card names here |
| 1–3 | #128–#131 | not v1.0 |

## Refuse

- Level 6 high-stakes
- published bench as measured GAIA score
- AISD v1.0 / live forecast ingest / trainer
