# Human Skills META — listed bind

Doing, not only knowing. Distinct from UKD (#77) and AIKD (#93).
Crate already on main: `gaia-skills`.
Issue this slice: #489 / #107. Children #108–#120 stay open.

## Rules already on main

- Local-default profiles. `global_profile_dump()` → false.
- No ambient scoring: `Session::start(false)` → `AmbientDenied`.
- No child EI inference. `hidden_profile_api()` → false.
- No practice licenses: `Badge::mint(..., clinical: true)` → `ClinicalCert`.
- TEK via #88: `tek_skill(false)` → `NoGrant`.
- `skills_v1_tagged()` → false.
- Do not score humans on the AISD maturity scale.

## Overlays (tags only)

`wef_top10`, `wef_2025_essay`, `digcomp_areas`, `bessi_domains` are name lists.
`wef_resolves(tag)` prefixes `skill:`. Not a live overlay ingest.

## Refuse

- ambient scoring / child marketplace rank / child EI
- clinical cert as a practice license
- skills v1.0 / global dump / TEK scrape
