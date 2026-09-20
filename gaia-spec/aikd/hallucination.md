# AIKD Hallucination — Normative Spec (Phase 1, issue #102)

> Status: **listed** (first cut). Not v1.0. No live detector.

## 1. Scope

This document specifies the hallucination taxonomy, uncertainty band, and
detector surface of AIKD Phase 1.

It covers `HallucinationClass`, `HallucinationWarning`, `UncertaintyBand`,
`uncertainty_band`, `hallucination_warning`, `tier_floor`, and `propagate_uncertainty`.

This document does **not** cover:
- Live retrieval-augmented hallucination detection.
- Conformal prediction or calibrated uncertainty scores.
- Continuous-eval or telemetry pipelines.
- AIKD v1.0 tagging.

## 2. Hallucination taxonomy

### 2.1 `HallucinationClass`

Five enumerated classes:

| Variant | Description | Default tier |
|---|---|---|
| `Temporal` | Answer is correct but outdated (knowledge cutoff) | T3 |
| `Factual` | Answer contradicts a cited source | T4 |
| `Fabricated` | Answer has no retrievable source basis | T5 |
| `Overconfident` | Uncertainty band is narrower than evidence supports | T4 |
| `Contradictory` | Answer contradicts another answer in the same session | T4 |

No code path may suppress a `Fabricated` classification. `Fabricated` MUST
always propagate as T5 → `NeedVerify`.

### 2.2 `HallucinationWarning`

Every `HallucinationWarning` instance carries:

| Field | Type | Constraint |
|---|---|---|
| `class` | `HallucinationClass` | Required |
| `tier` | `u8` (1–5) | MUST match class default or higher |
| `message` | `String` | Fixture string; must not be empty |

**Construction rule:** `hallucination_warning(class, tier)` MUST return
`Err(AikdError::NeedVerify)` when `tier >= 5` and the class is `Fabricated`.
All other T5 answers also propagate `NeedVerify` (see `tier_floor`).

## 3. Uncertainty band

### 3.1 `UncertaintyBand`

`UncertaintyBand` carries `(lower: f32, upper: f32)` as a fixture pair.

- Phase 1 fixture: `lower = 0.0`, `upper = 1.0` (maximum epistemic uncertainty).
- `uncertainty_band()` MUST return the Phase 1 fixture. No live conformal computation.
- Callers MUST NOT present the band as a measured or calibrated score.

### 3.2 `propagate_uncertainty`

`propagate_uncertainty(tier)` maps tier to an `UncertaintyBand`:

| Tier | `lower` | `upper` | Meaning |
|---|---|---|---|
| T1 | 0.0 | 0.2 | High confidence, cited |
| T2 | 0.1 | 0.4 | Cited, minor gaps |
| T3 | 0.3 | 0.7 | Temporal risk |
| T4 | 0.5 | 0.9 | Factual/overconfidence risk |
| T5 | 0.8 | 1.0 | Fabricated / must verify |

All values are fixtures. Future phases replace with conformal bounds.

## 4. Tier floor

`tier_floor(tier)` enforces the minimum acceptable tier behaviour:

- `tier < 1` or `tier > 5` MUST return `Err(AikdError::CannotKnow)`.
- `tier == 5` MUST return `Err(AikdError::NeedVerify)`.
- `tier <= 4` MUST return `Ok(tier)`.

No code path may call `tier_floor` and ignore its `Err` result.

## 5. Hard floor — `CannotKnow`

The following conditions MUST produce `AikdError::CannotKnow` regardless of tier:

- Empty question string.
- Output matching a prohibited biometric pattern (see `ToolCube` spec, §2.1).
- Closed-score claim (see `packs` spec, §4).

`CannotKnow` is the absolute floor. It MUST NOT be demoted to `NeedVerify`.

## 6. Soft ceiling — `NeedVerify`

The following conditions MUST produce `AikdError::NeedVerify`:

- T5 generation without a `verify:` prefix.
- Unsigned tool result used as a knowledge source.
- Missing citation on a T1/T2 answer.
- `Fabricated` hallucination class.

`NeedVerify` may be resolved in a future phase by attaching a verified proof
or a cited source. It MUST NOT be silently discarded.

## 7. Tests

The integration tests in `gaia-aikd/tests/hallucination.rs` assert:

1. `tier_floor(0)` → `Err(CannotKnow)`
2. `tier_floor(5)` → `Err(NeedVerify)`
3. `tier_floor(3)` → `Ok(3)`
4. `uncertainty_band()` returns `(0.0, 1.0)` Phase 1 fixture
5. `propagate_uncertainty(5)` returns `(0.8, 1.0)`
6. `hallucination_warning(Fabricated, 5)` → `Err(NeedVerify)`
7. `hallucination_warning(Temporal, 3)` succeeds and message is non-empty

## 8. Cross-references

- Hallucination module: `gaia-aikd/src/hallucination.rs`
- Answer envelope: `gaia-spec/aikd/PHASE-1.md` (#101)
- Retrieval stack: `gaia-spec/aikd/RETRIEVAL-STACK.md` (#101)
- Execute/verify: `gaia-spec/aikd/EXECUTE.md`
- Packs (CannotKnow gate): `gaia-spec/aikd/packs.md`
- Agentic (biometric gate): `gaia-spec/aikd/agentic.md`
- Issues: #94 (epic), #102 (this slice), #101 (foundation + retrieval stack)
- Eval spec: `gaia-spec/aikd/eval.md` (#106)
