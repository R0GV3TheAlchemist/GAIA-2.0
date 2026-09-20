# AIKD Phase 1 — answer envelope

Layer + tier + citations. Not a live model stack.
Crate already on main: `gaia-aikd::{Answer, QueryHit, Generation, Layer, Tier}`.
Issue this slice: #539 / #95. Children #101/#102 stay open.

## Envelope

`Answer::emit`:
- T5 → `NeedVerify`
- T1/T2 with empty citations → `MissingCitation`
- otherwise uncertainty is the fixture `0.5` (not a conformal product)

`QueryHit::offline` sets `used_network == false`. Empty question → `CannotKnow`.
`Generation::from_fixture` labels Temporal/Factual from fixture strings. T5 without `verify:` prefix → `NeedVerify`.

## Refuse

- live local runtime
- live retrieve store
- live hallucination detector product
