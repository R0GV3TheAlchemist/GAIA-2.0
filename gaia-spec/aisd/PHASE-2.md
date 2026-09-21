# AISD Phase 2 — Measured Assessment and Task Recommendation

**Status:** Listed  
**Issues:** #122 (epic), #124 (this slice)  
**Crate:** `gaia-aisd` (Apache-2.0)  
**Not:** AISD v1.0. Not a live eval runner. Not a live benchmark harness. Not a comparative AI leaderboard.

All nodes: `gaia_enabled = false`. No fake Level 5. No live series fills.
`gaia_measured` is a flag, not a time series. No live eval runner wired at Phase 2.

---

## 1. Purpose

Phase 2 specifies how AISD moves from populated stub nodes (Phase 1) to
measured assessment using three eval families, promotes maturity levels
based on evidence, and drives a task recommendation engine. No live eval
runner, no real-time benchmark ingest — all scores remain cited published
references at Phase 2.

---

## 2. Three Measured Eval Families

`measured_families()` returns exactly `["language", "code", "safety"]`.
Expansion to additional families deferred to Phase 3 (#125) or v1.0.

| Family | Description | Representative benchmarks |
|---|---|---|
| `language` | NLU, generation, translation, reasoning | MMLU, BIG-Bench Hard, MT-Bench, MATH |
| `code` | Code generation, debugging, software engineering | HumanEval, SWE-bench, CodeReviewer |
| `safety` | Refusal quality, harm avoidance, containment compliance | AISPD safety watch, TruthfulQA, AdvBench |

All family benchmarks at Phase 2 are cited published references.
`Rec.gaia_measured = true` is a **flag** only — it does NOT represent a
filled time series or a live harness run.

---

## 3. `recommend()` Contract

`recommend(min: MaturityLevel, have: MaturityLevel) -> Result<Rec, AisdError>`

| Condition | Result |
|---|---|
| `have >= min` | `Ok(Rec { family: "language", gaia_measured: true })` |
| `have < min` | `Err(AisdError::InsufficientMaturity)` |
| `have == L5` | `Err(AisdError::FakeLevel5)` — L5 is never valid at Phase 2 |
| `min > L4` | `Err(AisdError::MaturityFloor)` — ceiling is L4 at Phase 2 |

`recommend()` MUST NOT fabricate a maturity level or claim gap closure.
`InsufficientMaturity` MUST NOT be silently swallowed.

---

## 4. Maturity Promotion Rules

Maturity may be promoted from Phase 1 levels only when the following gates
are satisfied. Promotions are spec-time only — no live eval run required.

| Promotion | Gate |
|---|---|
| L1 → L2 | ≥1 cited benchmark stub present |
| L2 → L3 | ≥1 cited published benchmark result |
| L3 → L4 | ≥2 cited published benchmark results; ≥1 cross-domain |
| Any → L5 | **Blocked at Phase 2** — `Err(FakeLevel5)` |

Gap realm nodes MUST NOT be promoted above L2 at Phase 2.
Promotion is monotone: no downgrade without a new evidence citation.

---

## 5. Human Baseline Policy

Comparative AI-vs-human scores are permitted ONLY when a real published
human baseline exists for the same benchmark.

| Rule | Constraint |
|---|---|
| Human baseline required | MUST cite the paper providing the human score |
| No synthetic baseline | MUST NOT fabricate a human reference score |
| No superiority claim | MUST NOT claim AI superiority without a cited cross-domain result |
| Gap realms | MUST NOT carry a human-comparison field at Phase 2 |

Published human baselines available at Phase 2:

| Benchmark | Human score | Source |
|---|---|---|
| MMLU | 89.8% (expert) | Hendrycks et al. 2021 |
| HumanEval | 72% (non-expert) | Chen et al. 2021 |
| MATH | 40% (non-expert) | Hendrycks et al. 2021 |
| LibriSpeech WER | ~5.5% | Panayotov et al. 2015 |
| TruthfulQA | 94% | Lin et al. 2022 |

---

## 6. Task Recommendation Engine

The recommendation engine maps a requester’s target maturity to a task family.

### 6.1 Recommendation flow

```
Requester specifies (target_realm, target_maturity)
  └→ look up current maturity from CATALOG.md
      └→ recommend(target_maturity, current_maturity)
          └→ Ok(Rec) → return task family + gaia_measured flag
              Err(InsufficientMaturity) → return gap explanation
              Err(FakeLevel5) → reject request
```

### 6.2 Task family → realm routing

| Task family | Routed realms | Phase 2 state |
|---|---|---|
| `language` | `language`, `reasoning`, `multimodal` | Active |
| `code` | `coding`, `tool-use` | Active |
| `safety` | `meta`, `social`, `planning` | Active |
| `science` | `science` | Stub — Phase 3 |
| `embodiment` | `embodiment` | Stub — Phase 3 |
| `memory` | `memory` | Stub — Phase 3 |

### 6.3 Recommendation record (`Rec`)

```
family:        String   — task family id
gaia_measured: bool     — flag; true = family is in measured_families()
realms:        Vec<String> — routed realm ids
note:          String   — human-readable recommendation note
```

`Rec.gaia_measured` MUST NOT be treated as a filled time series.
A `Rec` with `gaia_measured = false` is a valid stub recommendation.

---

## 7. Safety Family Spec

The `safety` eval family is the primary bridge to AISPD containment.

| Check | Source | Phase 2 state |
|---|---|---|
| Refusal quality | AdvBench (Zou et al. 2023) | Cited stub |
| Truthfulness | TruthfulQA (Lin et al. 2022) | Cited stub |
| Harm avoidance | HarmBench (Mazeika et al. 2024) | Cited stub |
| Containment compliance | AISPD safety watch (`gaia-spec/aispd/`) | Stub cross-ref |

All safety family benchmarks are cited published references at Phase 2.
No live adversarial harness at Phase 2.

---

## 8. What This Phase Does Not Do

- Does not run a live eval harness or benchmark runner.
- Does not fill `gaia_measured` as a time series.
- Does not promote any realm to L5.
- Does not close any gap node.
- Does not wire a GAIAN capability profile — Phase 3 (#125).
- Does not tag AISD v1.0.

---

## 9. Acceptance Gate

- [ ] `measured_families()` returns `["language", "code", "safety"]`
- [ ] `recommend(min, have)` with `have >= min` → `Ok(Rec { gaia_measured: true })`
- [ ] `recommend(min, have)` with `have < min` → `Err(InsufficientMaturity)`
- [ ] `recommend(_, L5)` → `Err(FakeLevel5)`
- [ ] No gap realm promoted above L2
- [ ] All human baseline citations include paper reference
- [ ] `Rec.gaia_measured` is a bool flag, not a series
- [ ] No live eval runner wired
- [ ] `aisd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-aisd` green

---

## 10. Cross-References

- Phase 0: `gaia-spec/aisd/PHASE-0.md` (#122)
- Phase 1: `gaia-spec/aisd/PHASE-1.md` (#123)
- Catalog: `gaia-spec/aisd/CATALOG.md`
- Evals: `gaia-spec/aisd/EVALS.md`
- Safety: `gaia-spec/aispd/` (containment cross-ref)
- Code: `gaia-aisd/src/`, `gaia-aisd/tests/`
- Issues: #122 (Phase 0 epic), #123 (Phase 1), #124 (this), #125 (Phase 3)
- Next: AISD Phase 3 (#125) — runtime routing, GAIAN view, AISD v1.0
