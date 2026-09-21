# Placement: reading-and-writing

**Status:** decided  
**Issue:** #700  
**Ancestor id:** `hum.know.basic.reading-and-writing`  
**Ancestor slug:** `reading-and-writing` (no `basic-` prefix)

## Decision

**Cross-cutting foundation domain**, not a fourth "Tier 0" identity and not a split into two canonical domains yet.

| Field | Value |
|---|---|
| Canonical id | `gaia.knowledge.literacy.reading-and-writing` |
| Slug | `reading-and-writing` |
| Kind | `domain` |
| Stage | `foundation` |
| Track | `foundational-literacy` |
| Special | `cross_cutting: true` |

## Rationale

1. The ancestor already treated the folder as special: it is the only basic subject whose slug is not `basic-*`.
2. Literacy is a prerequisite for every other domain, including the three new foundation domains in #698.
3. A separate Tier 0 identity would encode placement in the id, which #696 forbids.
4. Splitting reading vs writing now would fork aliases before content is migrated. Split remains allowed later as a successor pair with `replaced_by`.

## What this is not

- Not a GAIAN boot requirement implemented in code.
- Not a hidden profile or ambient scoring of a person's literacy.
- Not a substitute for UKD language realms.

## Acceptance for #700

- [x] Options considered.
- [x] Formal placement written.
- [x] `catalog.json` carries the row.
