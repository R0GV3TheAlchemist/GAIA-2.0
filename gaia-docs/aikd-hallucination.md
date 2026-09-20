# AIKD hallucination first cut (#102)

Working notes. Not a live hallucination detector. Not AIKD v1.0.

- Hallucination taxonomy covers five classes: Temporal, Factual, Fabricated, Overconfident, Contradictory.
- Each class maps to a Tier (T1–T5). T5 requires a `verify:` prefix or the answer envelope emits `NeedVerify`.
- Uncertainty is a fixture `0.5` at Phase 1. Not a conformal prediction product. Not calibrated.
- `CannotKnow` is the hard floor: empty question, biometric pattern, or closed-score claim → `CannotKnow`.
- `NeedVerify` is the soft ceiling: T5 generation, unsigned tool result, or missing citation → `NeedVerify`.
- `UncertaintyBand` carries `(lower, upper)` as fixtures. No live conformal runtime.
- `HallucinationWarning` carries class + tier + fixture message. No live detector product.
- There is no live retrieval store, no live LLM, and no continuous-eval pipeline at this phase.
- Prohibited: presenting `0.5` uncertainty as a measured score, suppressing `NeedVerify` in any code path,
  or treating T4/T5 answers as citable without a `verify:` prefix.

See `gaia-aikd/src/hallucination.rs` for the implementation.
See `gaia-aikd/tests/hallucination.rs` for the acceptance tests.
See `gaia-spec/aikd/hallucination.md` for the normative spec.
