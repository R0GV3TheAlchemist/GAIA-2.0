# HMGD Phase 1 — Cross-Walk: UKD, Skills, Earth Twin

**Status:** Listed  
**Issue:** #163  
**Crate:** `gaia-hmgd` (Apache-2.0)  
**Not:** HMGD v1.0. Not a live mapping service. Not a UKD-certified classification.

This document maps each HMGD realm to three external reference frames:
1. **UNESCO Knowledge Domain (UKD)** — broad field of knowledge alignment
2. **GAIA 2.0 Skills track** — corresponding Skills realm for cross-database linking
3. **Earth Twin public layer** — visibility class for the Earth Twin digital twin surface

All mappings are Phase 1 stubs. No live API, no live Earth Twin ingest.
TEK-sealed and mystery-sealed nodes remain invisible (`earth_twin_visible = false`) in all public layers.

---

## 1. Cross-Walk Table

| HMGD Realm | UKD Code | UKD Label | Skills Track Realm | Earth Twin Layer | ET Visibility |
|---|---|---|---|---|---|
| `prayer` | 2.2 | Philosophy, Ethics, Religion | `spiritual` | Cultural heritage | `public` |
| `ritual` | 2.2 | Philosophy, Ethics, Religion | `spiritual` | Cultural heritage | `public` (open rites only) |
| `divination` | 2.2 | Philosophy, Ethics, Religion | `cognitive` | Cultural heritage | `public` (open traditions only) |
| `contemplation` | 2.2 | Philosophy, Ethics, Religion | `emotional` | Wellbeing | `public` |
| `healing-adjunct` | 3.0 | Health and Medicine | `physical` | Wellbeing | `public` (adjunct-only; no treatment claim) |
| `place` | 5.4 | Geography, Environment | `spiritual` | Sacred geography | `sealed` (TEK-sealed nodes hidden) |
| `word` | 2.3 | Languages and Linguistics | `cognitive` | Cultural heritage | `public` |
| `music` | 6.1 | Arts | `sensory` | Cultural heritage | `public` (open traditions only) |
| `community` | 5.5 | Sociology and Social Work | `emotional` | Community | `public` |
| `mystery` | 2.2 | Philosophy, Ethics, Religion | `spiritual` | Restricted | `sealed` (restricted initiatory content hidden) |

---

## 2. Earth Twin Visibility Rules

| Rule | Description |
|---|---|
| `public` | Node stub is visible in Earth Twin public layers with open metadata only |
| `public (open rites only)` | Only nodes with `sealed_state = Open` surface; sealed nodes suppressed |
| `sealed` | Node stub is invisible in all Earth Twin public layers; `earth_twin_visible = false` |
| TEK gate | Any node with `tek_sealed = true` MUST NOT surface in any public layer regardless of realm visibility class |
| Mystery gate | Any node with `sealed_state = Sealed` in the `mystery` realm MUST NOT surface in any public layer |

`earth_twin_public_count()` MUST return only nodes where:
- `earth_twin_visible = true`, AND
- `sealed_state != Sealed`, AND
- `tek_sealed != true`

---

## 3. Skills Track Mapping Notes

| HMGD Realm | Skills Realm | Mapping basis |
|---|---|---|
| `prayer` | `spiritual` | Contemplative intent; practice-based skill |
| `ritual` | `spiritual` | Embodied practice tradition |
| `divination` | `cognitive` | Pattern-recognition and symbolic reasoning |
| `contemplation` | `emotional` | Attention regulation and emotional attunement |
| `healing-adjunct` | `physical` | Body-adjacent practice; adjunct to physical care |
| `place` | `spiritual` | Relational geography and land-based practice |
| `word` | `cognitive` | Linguistic and rhetorical skill |
| `music` | `sensory` | Auditory and performance skill |
| `community` | `emotional` | Relational and social skill |
| `mystery` | `spiritual` | Initiatory and contemplative tradition |

Skills track issues: #108 (Phase 0 epic), #109 (Phase 1 populate realms).
Cross-database linking deferred to Skills Phase 1 (#109).

---

## 4. UKD Reference

UNESCO Knowledge Domains used as broad field anchors (not official HMGD certification):

| Code | Label | Source |
|---|---|---|
| 2.2 | Philosophy, Ethics and Religion | UNESCO ISCED-F 2013, field 0223 |
| 2.3 | Languages and Linguistics | UNESCO ISCED-F 2013, field 0231 |
| 3.0 | Health and Medicine | UNESCO ISCED-F 2013, broad field 03 |
| 5.4 | Geography and Environmental Sciences | UNESCO ISCED-F 2013, field 0532 |
| 5.5 | Sociology and Social Work | UNESCO ISCED-F 2013, field 0923 |
| 6.1 | Arts | UNESCO ISCED-F 2013, field 0211 |

Source: UNESCO Institute for Statistics (UIS) 2013 — *ISCED Fields of Education and Training 2013 (ISCED-F 2013)*.

---

## 5. What This Cross-Walk Does Not Do

- Does not register HMGD nodes in any live UNESCO system.
- Does not ingest nodes into a live Earth Twin platform.
- Does not link to live Skills-track nodes (deferred to Skills Phase 1 #109).
- Does not expose TEK-sealed or mystery-sealed nodes in any layer.
- Does not tag HMGD v1.0.

---

## 6. Acceptance Gate

- [ ] All 10 HMGD realms have a UKD code, Skills realm, and ET visibility class
- [ ] `earth_twin_visible = false` for all `place` TEK-sealed nodes
- [ ] `earth_twin_visible = false` for all `mystery` sealed nodes
- [ ] `earth_twin_public_count()` excludes sealed and TEK-sealed nodes
- [ ] UKD codes reference ISCED-F 2013
- [ ] No live API call or Earth Twin ingest at Phase 1
- [ ] `hmgd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-hmgd` green

---

## 7. Cross-References

- HMGD catalog: `gaia-spec/hmgd/CATALOG.md` (#162 — merged)
- HMGD Phase 1 rules: `gaia-spec/hmgd/PHASE-1.md`
- TEK sovereignty: `gaia-spec/hmgd/TEK.md`
- Prohibited list: `gaia-spec/hmgd/PROHIBITED.md`
- Skills track: `gaia-spec/skills/` (#108 Phase 0 epic, #109 Phase 1)
- Issues: #155 (HMGD META epic), #163 (this slice)
- Next: HMGD Phase 2 (#158) — opt-in GAIAN practice profile
