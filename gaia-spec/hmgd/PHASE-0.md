# HMGD Phase 0 — schema + evidence

Contract before cataloging anyone's practice.
Crate already on main: `gaia-hmgd::{REALMS, parse_node, EvidenceClass}`.
Charter already listed under #161 and META #531.
Issue this slice: #533 / #156.

## Realms

`REALMS` has 10 names. That is the taxonomy data.

## Node fields that exist

`MagicNode`: id, evidence (`EvidenceClass`), sources.
Issue text also named `sovereignty_state`. That field is **not** on the struct. Sealed collections use `sealed_rite()` → `Sealed` instead.

`parse_node`:
- contains `recipe` or `dose` → `RecipeForbidden`
- contains `curse` → `CurseForbidden`
- otherwise unknown → `MissingEvidence`

## Refuse

- dose / recipe / curse product fields
- HMGD v1.0
