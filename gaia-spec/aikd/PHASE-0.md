# AIKD Phase 0 — Taxonomy, Model Card, Registry, Tiers, Eval Harness

**Status:** Listed  
**Issues:** #93 (meta), #94 (epic), #96 (adapters), #99 (cannot-know), #100 (tier binding)  
**Crate:** `gaia-aikd` (Apache-2.0)  
**Not:** AIKD v1.0. Not a live model runtime. Not a live leaderboard. Not a calibrated eval product.

`aikd_v1_tagged()` MUST return `false`.  
`eval_harness_is_live()` MUST return `false`.  
`published_closed_score_is_ours()` MUST return `false`.

---

## 1. Purpose

Phase 0 names what AI knowledge is before wiring models. It publishes the
`KnowledgeType` taxonomy, the `ModelCard` schema, the `BenchRow` registry
convention, and the T1–T5 tier definitions. No live inference, no live eval run,
no live leaderboard claim.

---

## 2. Knowledge Taxonomy (`KnowledgeType`)

`KnowledgeType::all()` returns exactly 7 types:

| Type | Description |
|---|---|
| `Factual` | Verifiable claim with a ground-truth referent |
| `Procedural` | Step-by-step task knowledge |
| `Conceptual` | Definitions, categories, and relationships |
| `Temporal` | Time-sensitive knowledge subject to staleness |
| `Inferential` | Derived from reasoning over other knowledge types |
| `Uncertain` | Claim with known or quantified uncertainty |
| `CannotKnow` | Structurally unknowable — see `cannot-know.csv` |

### Epistemic layers

```rust
pub enum EpistemicLayer {
    KnownKnown,       // model knows and knows it knows
    KnownUnknown,     // model knows it does not know
    UnknownUnknown,   // model does not know it does not know
}
```

`cannot_know()` returns the list from `cannot-know.csv`. The list MUST be
non-empty. `CannotKnow` entries MUST NOT be answered with fabricated content
— `Err(AikdError::CannotKnow)` is the only valid return.

---

## 3. `ModelCard` Schema

`model_card()` returns the Phase 0 fixture card:

```
model_id:      String    — "local-llama-fixture"
open_weight:   bool      — true
cutoff_label:  String    — non-empty; user-visible cutoff description
cannot_know:   Vec<String> — copied from cannot_know(); MUST be non-empty
tier:          Tier      — fixture tier assignment (T1–T5)
gaia_measured: bool      — false at Phase 0; no live eval run
```

**MUST rules**
- `model_card().cutoff_label` MUST be non-empty.
- `model_card().cannot_know` MUST equal `cannot_know()`.
- `model_card().gaia_measured` MUST be `false` at Phase 0.
- `ModelCard` MUST NOT carry a `score` field derived from a closed leaderboard — `Err(ClosedScoreClaim)` otherwise.

---

## 4. `BenchRow` Registry

`BenchRow::local_open()` returns the Phase 0 fixture row:

```
bench_id:    String   — "local-open-fixture"
published:   bool     — false at Phase 0
gaia_measured: bool   — false at Phase 0; no live run
source:      String   — non-empty reference to bench definition
```

**MUST rules**
- `claim_closed_as_measured(id)` MUST return `Err(ClosedScoreClaim)` for any closed-leaderboard id.
- `published_closed_score_is_ours()` MUST return `false` — GAIA does not claim ownership of third-party benchmark scores.
- `BenchRow::local_open().gaia_measured` MUST be `false`.

---

## 5. Tier Definitions (T1–T5)

Defined in `tiers.csv` and as `Tier::{T1..T5}` in the crate:

| Tier | Label | Representative class | Key constraint |
|---|---|---|---|
| T1 | Cited-authoritative | 70B+ instruction-tuned | Sources required; citations mandatory |
| T2 | Cited-reference | 13B–70B instruction-tuned | Citations required; minor gaps tolerated |
| T3 | Temporal-risk | Any model with knowledge cutoff | `NeedVerify` on time-sensitive claims |
| T4 | Factual-risk | Small or unverified model | Factual/overconfidence risk flagged |
| T5 | Must-verify | Fabricated / uncited output | Always `NeedVerify`; `verify:` prefix required |

All Phase 0 tier assignments are fixture labels. No live benchmark run.
`Tier::from_str(s)` MUST return `Err(UnknownTier)` for any string not in `{"T1","T2","T3","T4","T5"}`.

---

## 6. Eval Harness Rules

| Call | Behaviour |
|---|---|
| `eval_harness_is_live()` | MUST return `false` — no live eval product at Phase 0 |
| `run_eval_live()` | MUST return `Err(EvalHarnessNotLive)` |
| `BenchRow::local_open().gaia_measured` | MUST be `false` |
| `claim_closed_as_measured(id)` | MUST return `Err(ClosedScoreClaim)` |

No CI step may call a live benchmark endpoint. All fixtures are static files
in `gaia-spec/aikd/benches.csv` and `gaia-spec/aikd/models.csv`.

---

## 7. Prohibition Surface

| Prohibition | Error |
|---|---|
| Closed leaderboard score claimed as GAIA-measured | `Err(ClosedScoreClaim)` |
| Live eval harness at Phase 0 | `Err(EvalHarnessNotLive)` |
| `CannotKnow` entry answered with fabricated content | `Err(CannotKnow)` |
| `ModelCard` with empty `cutoff_label` | `Err(MissingCutoffLabel)` |
| Unknown tier string | `Err(UnknownTier)` |
| AIKD v1.0 tag | `aikd_v1_tagged()` returns `false` |

---

## 8. What This Phase Does Not Do

- Does not run live local LLM inference.
- Does not run a live eval harness or benchmark suite.
- Does not claim third-party benchmark scores as GAIA-measured.
- Does not wire retrieval stack — Phase 1 (#101).
- Does not specify hallucination taxonomy — Phase 1 / `hallucination.md` (#102).
- Does not tag AIKD v1.0.

---

## 9. Acceptance Gate

- [ ] `KnowledgeType::all()` returns exactly 7 types
- [ ] `cannot_know()` returns non-empty list matching `cannot-know.csv`
- [ ] `model_card().cutoff_label` is non-empty
- [ ] `model_card().gaia_measured == false`
- [ ] `claim_closed_as_measured("gpt")` → `Err(ClosedScoreClaim)`
- [ ] `published_closed_score_is_ours()` → `false`
- [ ] `eval_harness_is_live()` → `false`
- [ ] `Tier::from_str("T6")` → `Err(UnknownTier)`
- [ ] `aikd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-aikd` green

---

## 10. Cross-References

- Taxonomy: `gaia-spec/aikd/TAXONOMY.md`
- Tiers: `gaia-spec/aikd/tiers.csv`
- Models: `gaia-spec/aikd/models.csv`
- Benches: `gaia-spec/aikd/benches.csv`
- Cannot-know: `gaia-spec/aikd/cannot-know.csv`, `cannot-know.json`
- Tags: `gaia-spec/aikd/tags.csv`
- Phase 1: `gaia-spec/aikd/PHASE-1.md` (#101, #102)
- Issues: #93 (meta), #94 (epic), #96 (adapters), #99 (cannot-know), #100 (tier binding)
