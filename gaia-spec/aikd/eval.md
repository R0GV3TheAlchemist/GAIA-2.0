# AIKD Calibration and Eval — Normative Spec (Phase 4, issue #106)

> Status: **listed** (first cut). Not v1.0. No live eval pipeline.

## 1. Scope

This document specifies the calibration gate, gap-detection surface, audit
report fixture, and v1.0 readiness conditions of AIKD Phase 4.

It covers `aikd_v1_tagged`, `knows_everything`, `audit_report`, `gap_detected`,
`calibration_score`, and `EvalReport`.

This document does **not** cover:
- Live continuous-eval or telemetry pipelines.
- Conformal calibration computation.
- Automated gap-detection against a live knowledge graph.
- AIKD v1.0 tagging (explicitly refused — see §5).

## 2. Audit report

`audit_report()` MUST return exactly four fixture lines in order:

```
measured scores: none yet
cutoff: fixture
cannot-know: listed
hallucination rate: unmeasured
```

No code path may substitute estimated, synthetic, or extrapolated values
for these lines. When real measurements exist, a dedicated release PR
replaces the fixture lines with sourced numbers and removes this constraint.

## 3. v1.0 gate

### 3.1 `aikd_v1_tagged`

`aikd_v1_tagged()` MUST return `false`.

Conditions required before this may return `true` (all must be met):

1. `audit_report()` lines 1 and 4 contain measured, sourced numbers.
2. `calibration_score()` returns `Some(score)` where `score >= 0.7`.
3. Every `CannotKnow` and `NeedVerify` path is covered by an acceptance test.
4. A TSC vote has approved the v1.0 tag (see `GOVERNANCE.md`).

### 3.2 `knows_everything`

`knows_everything()` MUST return `false`.
No future phase may make this return `true`. GAIA does not know everything.

## 4. Calibration and gap detection

### 4.1 `calibration_score`

`calibration_score()` MUST return `None` at Phase 4.
Callers MUST surface `None` as "not yet calibrated" rather than a zero score.

### 4.2 `gap_detected`

`gap_detected(topic: &str)` MUST return:
- `Err(AikdError::CannotKnow)` for empty `topic`.
- `Ok(Some(format!("gap: {} not in fixture catalog", topic)))` for any non-empty topic.

No live gap-detection algorithm exists at Phase 4. All gaps are fixture-detected.

### 4.3 `EvalReport`

`EvalReport` carries:

| Field | Type | Phase 4 value |
|---|---|---|
| `v1_tagged` | `bool` | `false` |
| `knows_all` | `bool` | `false` |
| `calibration` | `Option<f32>` | `None` |
| `audit_lines` | `Vec<String>` | Four fixture strings |
| `gaps` | `Vec<String>` | Empty at construction; caller may push detected gaps |

`EvalReport::build()` MUST populate all fields from the functions above.

## 5. Explicit refusals

- AIKD v1.0 tag: MUST NOT be set without the four conditions in §3.1.
- Marketing that AIKD knows everything: `knows_everything()` is a public API
  that MUST remain `false` permanently.
- Synthetic audit lines: MUST NOT replace fixture lines with estimated data.
- Suppressing `calibration_score() == None`: callers MUST surface `None`.

## 6. Tests

The integration tests in `gaia-aikd/tests/eval.rs` assert:

1. `aikd_v1_tagged()` → `false`
2. `knows_everything()` → `false`
3. `audit_report()` contains exactly four lines matching the fixture strings
4. `calibration_score()` → `None`
5. `gap_detected("")` → `Err(CannotKnow)`
6. `gap_detected("thermodynamics")` → `Ok(Some("gap: thermodynamics not in fixture catalog"))`
7. `EvalReport::build()` fields match all of the above

## 7. Cross-references

- Eval module: `gaia-aikd/src/eval.rs`
- v1.0 gate (Phase 4 existing): `gaia-spec/aikd/PHASE-4.md`
- Hallucination spec: `gaia-spec/aikd/hallucination.md` (#102)
- Answer envelope: `gaia-spec/aikd/PHASE-1.md`
- Execute/verify: `gaia-spec/aikd/EXECUTE.md`
- Issue #106 (parent: #94 AIKD Phase 0 epic)
- Governance: `GOVERNANCE.md`
