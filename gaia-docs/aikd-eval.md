# AIKD calibration and eval first cut (#106)

Working notes. Not a live eval pipeline. Not AIKD v1.0.

- `aikd_v1_tagged()` → false. There is no AIKD v1.0 tag.
- `knows_everything()` → false. AIKD does not know everything.
- `audit_report()` emits four fixture lines:
    1. `measured scores: none yet`
    2. `cutoff: fixture`
    3. `cannot-know: listed`
    4. `hallucination rate: unmeasured`
- `gap_detected(topic)` → `Some("gap: <topic> not in fixture catalog")` always at this phase.
  No live gap-detection algorithm exists.
- `calibration_score()` → `None`. No calibration has been run.
- `EvalReport` carries fixture strings only. Future phases replace these with measured numbers.
- Prohibited: replacing fixture lines with synthetic or estimated numbers, tagging AIKD v1.0,
  claiming AIKD is calibrated, or suppressing `calibration_score() == None`.

See `gaia-aikd/src/eval.rs` for the implementation.
See `gaia-aikd/tests/eval.rs` for the acceptance tests.
See `gaia-spec/aikd/eval.md` for the normative spec.
