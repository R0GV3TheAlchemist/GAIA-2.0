# AIKD Phase 1 — Foundation + Retrieval Stack

**Status:** Listed  
**Issues:** #94 (epic), #101 (foundation + retrieval), #102 (hallucination — see `hallucination.md`)  
**Crate:** `gaia-aikd` (Apache-2.0)  
**Not:** AIKD v1.0. Not a live model runtime. Not a live retrieval store. Not a calibrated uncertainty product.

---

## 1. Purpose

Phase 1 wires the answer envelope to a tiered open-LLM stack, an offline
Qdrant-backed RAG layer, and a MemOS memory layer. No live inference.
No live vector ingest. All stores are offline fixtures for Phase 1.

Hallucination taxonomy, uncertainty bands, and tier floor are specified
fully in `hallucination.md` (#102).

---

## 2. Open LLM Tiers

| Tier | Label | Representative model class | Notes |
|---|---|---|---|
| T1 | Cited-authoritative | 70B+ instruction-tuned | Sources required; citations mandatory |
| T2 | Cited-reference | 13B–70B instruction-tuned | Citations required; minor gaps tolerated |
| T3 | Temporal-risk | Any model with knowledge cutoff | `NeedVerify` on time-sensitive claims |
| T4 | Factual-risk | Small or unverified model | Factual/overconfidence risk flagged |
| T5 | Must-verify | Fabricated / uncited output | Always `NeedVerify`; `verify:` prefix required |

All tier assignments in Phase 1 are fixture labels. No live benchmark run.
`tier_floor(tier)` enforces the constraints defined in `hallucination.md §4`.

---

## 3. RAG Layer (Qdrant — Offline Fixture)

`QueryHit::offline` sets `used_network = false` for all Phase 1 results.
No live Qdrant ingest or query at Phase 1.

| Component | Phase 1 behaviour |
|---|---|
| `qdrant_client` | Offline fixture; no live HTTP to Qdrant Cloud or local instance |
| `embed_query(text)` | Returns fixture embedding vector; no live model call |
| `vector_search(query)` | Returns fixture `QueryHit` list with `used_network = false` |
| `attach_citations(hits)` | Attaches fixture citation stubs to answer |

T1/T2 answers with empty citations after `attach_citations` MUST produce
`AikdError::MissingCitation`. This is not waivable at Phase 1.

---

## 4. MemOS Memory Layer

MemOS provides long-horizon memory across sessions.

| Component | Phase 1 behaviour |
|---|---|
| `memos_store(key, value)` | Writes to offline fixture store; no live persistence |
| `memos_retrieve(key)` | Returns fixture value or `AikdError::CannotKnow` if absent |
| `memos_forget(key)` | Removes key from fixture store; emits `MemOsForgetEvent` |
| `memos_migrate(key)` | Migrates key to new id; old id preserved as alias |

`memos_forget` is the only deletion path. There is no bulk-wipe path at Phase 1.
`memos_migrate` MUST preserve the original id as an alias; it MUST NOT silently drop it.

---

## 5. Answer Envelope Rules

`Answer::emit` rules (from existing crate, confirmed for Phase 1):

- T5 output → `NeedVerify` always
- T1/T2 with empty citations → `MissingCitation`
- Uncertainty fixture: `0.5` (not a conformal product)
- Empty question → `CannotKnow`

`Generation::from_fixture` labels `Temporal` / `Factual` from fixture strings.
T5 without `verify:` prefix → `NeedVerify`.

---

## 6. What This Phase Does Not Do

- Does not run live local LLM inference.
- Does not ingest live documents into Qdrant.
- Does not run a live hallucination detector or classifier.
- Does not produce calibrated or conformal uncertainty scores.
- Does not tag AIKD v1.0.

---

## 7. Acceptance Gate

- [ ] `QueryHit::offline` sets `used_network = false`
- [ ] T1/T2 with empty citations → `AikdError::MissingCitation`
- [ ] T5 without `verify:` prefix → `AikdError::NeedVerify`
- [ ] Empty question → `AikdError::CannotKnow`
- [ ] `memos_forget` emits `MemOsForgetEvent`
- [ ] `memos_migrate` preserves original id as alias
- [ ] `tier_floor(5)` → `Err(NeedVerify)` (see `hallucination.md §4`)
- [ ] `uncertainty_band()` returns `(0.0, 1.0)` Phase 1 fixture
- [ ] `aikd_v1_tagged() == false`
- [ ] `cargo test -p gaia-aikd` green

---

## 8. Cross-References

- Hallucination: `gaia-spec/aikd/hallucination.md` (#102)
- Retrieval stack: `gaia-spec/aikd/RETRIEVAL-STACK.md`
- Execute/verify: `gaia-spec/aikd/EXECUTE.md`
- Packs: `gaia-spec/aikd/packs.md`
- Agentic: `gaia-spec/aikd/agentic.md`
- Code: `gaia-aikd/src/`, `gaia-aikd/tests/hallucination.rs`
- Issues: #94 (epic), #101 (this slice), #102 (hallucination)
- Next: Phase 2 (#103, #104) — calibration + gap detection
