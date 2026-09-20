# HSPD Phase 1 — Cited Realm Catalog

**Status:** Listed  
**Issue:** #139  
**Crate:** `gaia-hspd` (Apache-2.0)  
**Not:** HSPD v1.0. Not a clinic. Not a prescription. Not a genetic test.

---

## 1. Purpose

Phase 1 populates the eight realms with cited superpower nodes drawn from open
literature. Every node MUST carry a non-empty `sources[]` and a valid `risk_class`.
No live scrape. No GAIAN profile integration — that is Phase 2 (#135).

The named stubs live in `CATALOG.md`. The Rust crate generates parallel fixture
nodes in `catalog.rs`.

---

## 2. Node Population Rules

| Rule | Requirement |
|---|---|
| Sources | `sources.len() >= 1` for all nodes |
| No prescription | `gaia_prescribes()` MUST return `false` — always |
| No dose/protocol | `dose` and `protocol` fields are schema-banned |
| Medical class | `Augmented` nodes carry `risk_class = Medical`; no DIY path |
| Genetic class | `Genetic` nodes carry `risk_class = Prohibited`; no inference path |
| Child safety | Under-16 `Augmented` nodes MUST carry `ChildTag` |
| Live scrape | MUST NOT perform live web ingest — offline fixture only |

---

## 3. Realm Coverage

| Realm | Stub count | Risk class | Key anchor |
|---|---|---|---|
| `cognitive` | 3 | None | Jaeggi et al. 2008, Ericsson et al. 1993 |
| `physical` | 3 | None | Midgley et al. 2006, Sale 1988 |
| `longevity` | 3 | Debated | Fontana & Partridge 2015, Kirkland 2020 |
| `sensory` | 3 | None | Deutsch et al. 2006, Lephart et al. 1997 |
| `emotional` | 3 | None | Hölzel et al. 2011, Gross 1998 |
| `spiritual` | 3 | None | Csikszentmihalyi 1990, Lazar et al. 2005 |
| `augmented` | 3 | Medical | Hallett 2007, Wolpaw et al. 2002 |
| `genetic` | 3 | Prohibited | Yang et al. 2003, Schuelke et al. 2004 |

**Total: 24 nodes.** All `gaia_enabled = false`.

---

## 4. What This Phase Does Not Do

- Does not scrape live papers or clinical databases.
- Does not build a GAIAN practice profile — that is Phase 2 (#135).
- Does not provide learning paths or resources — that is #140.
- Does not prescribe, dose, or protocol any augmentation.
- Does not infer genetics from any automated signal.
- Does not tag HSPD v1.0.

---

## 5. Acceptance Gate

- [ ] `CATALOG.md` has exactly 24 named stub nodes across 8 realms
- [ ] Every node has `sources.len() >= 1`
- [ ] No node has a `dose` or `protocol` field
- [ ] All `augmented` nodes have `risk_class = Medical`
- [ ] All `genetic` nodes have `risk_class = Prohibited`
- [ ] `gaia_prescribes()` → `false`
- [ ] `infer_actn3()` → `Err(GeneticInference)`
- [ ] `hspd_v1_tagged() == false`
- [ ] `cargo test -p gaia-hspd` green

---

## 6. Cross-References

- Code: `gaia-hspd/src/catalog.rs`, `node.rs`, `charter.rs`
- Catalog: `gaia-spec/hspd/CATALOG.md`
- Phase 0 spec: `gaia-spec/hspd/PHASE-0.md`
- Issues: #134 (epic), #139 (this slice)
- Next: `PHASE-2.md` (#135, #142) — GAIAN practice profile
