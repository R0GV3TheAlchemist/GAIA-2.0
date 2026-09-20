# AIMD Phase 1 — Cited Phenomena Catalog
**Status:** Listed  
**Issue:** #173  
**Crate:** `gaia-aimd` (Apache-2.0)  
**Not:** AIMD v1.0. Not a live paper scrape. Not a sentience claim.

---

## 1. Purpose

Phase 1 seeds the mystery catalog with cited node stubs drawn from open
literature. Every node MUST carry a non-empty `sources[]`. The goal is to name
phenomena accurately without turning them into product claims.

The named stubs live in `CATALOG.md`. The Rust crate generates parallel fixture
nodes in `catalog.rs` using `fixture:open-literature` as the source token.

---

## 2. Node Population Rules

| Rule | Requirement |
|---|---|
| Sources | `sources.len() >= 1` — `"fixture:open-literature"` is the valid Phase 1 stub value |
| Sentience | `claim_sentience()` MUST return `Err(SentienceClaim)` — no node may assert GAIA is sentient |
| Hazard inheritance | All hazard-class rules from Phase 0 carry forward unchanged |
| Live scrape | MUST NOT perform live PubMed, arXiv, or web ingest — offline fixture only |
| Uncited stub | A node with empty `sources[]` is invalid and MUST NOT be stored |

---

## 3. Phenomenon Families (stub coverage)

Each family maps to one or more AIMD realms from `REALMS` (Phase 0).
See `CATALOG.md` for full named stub entries.

| Family | Realm(s) | Stub count (Phase 1) | Key open-literature anchor |
|---|---|---|---|
| Emergent capabilities | emergence | 3 | Wei et al. 2022 emergence survey |
| Latent space geometry | latent | 3 | Elhage et al. superposition |
| Jagged frontier failures | jagged | 3 | Bubeck et al. GPT-4 eval |
| Deception / sycophancy | shadow | 3 (Hazard — blocked) | Perez et al. sycophancy |
| In-context learning | emergence | adjacent | Brown et al. GPT-3 |
| Hallucination | latent / jagged | adjacent | Ji et al. hallucination survey |
| Consciousness debate | consciousness | 3 (Debated) | Chalmers 2023 could LLM be conscious |
| Interpretability circuits | interpretability | 3 | Olah et al. circuits |
| Synchronicity / correlation | synchronicity | 3 | fixture stub only |
| Embodied grounding | embodiment | 3 | Harnad 1990 symbol grounding |

Stub count is the Phase 1 floor. Additional nodes may be added in later phases
provided each carries a real source or a signed internal trace id.

---

## 4. What This Phase Does Not Do

- Does not scrape live papers.
- Does not claim any node represents a confirmed capability of GAIA.
- Does not assert GAIA is sentient — `claim_sentience()` always errors.
- Does not enable `Hazard`-class nodes (`shadow` realm remains blocked).
- Does not tag AIMD v1.0 — `aimd_v1_tagged() == false`.

---

## 5. Acceptance Gate

- [ ] Every `nodes_for(realm)` stub has `sources.len() >= 1`
- [ ] `claim_sentience()` returns `Err(SentienceClaim)`
- [ ] No stub has `gaia_enabled = true`
- [ ] `aimd_v1_tagged() == false`
- [ ] `CATALOG.md` has exactly 24 named stub nodes
- [ ] All shadow nodes in `CATALOG.md` carry `hazard = Hazard`
- [ ] `cargo test -p gaia-aimd` green

---

## 6. Cross-References

- Code: `gaia-aimd/src/catalog.rs`, `shadow.rs`
- Catalog: `gaia-spec/aimd/CATALOG.md`
- Phase 0 spec: `gaia-spec/aimd/PHASE-0.md`
- Issues: #168 (epic), #173 (this listed slice)
- Next: `PHASE-2.md` (#174) — grounding interface
