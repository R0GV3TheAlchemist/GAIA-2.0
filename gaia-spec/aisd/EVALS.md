# AISD measured evals + recommender

Capability × quality × reliability. Prefer fixtures we can rerun later.
Crate already on main: `gaia-aisd::{recommend, measured_families}`.
Issue this slice: #485 / #130.

## Three families (names)

`measured_families()` → language, code, safety.
`measured-families.csv` holds empty series columns. That is not three filled time series. Do not treat the bool `Rec.gaia_measured` as a history.

Language / code / safety evals cited from #102 / #103 / #106 stay **citations**. This slice does not wire a runner.

## Recommender

`recommend(min, have)`:
- `have >= min` → `Ok` with family `language` and `gaia_measured: true` as a *flag*
- otherwise → `InsufficientMaturity` (no fake Level 5)

Comparative AI vs human only when a real baseline exists. None is stored here.

## Refuse

- fake Level 5
- published bench as filled `gaia_measured` series
- live eval runner
