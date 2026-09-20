# AIKD Phase 2 — specialized domains

Adapters are labels + cite-or-refuse. Not practice licenses.
Crate already on main: `gaia-aikd::{Adapter, Executed, gaia_certifies_usmle_or_bar}`.
Issue this slice: #525 / #96. Children #103 / #104 stay open.

## Adapters

`Adapter::{Math, Code, Science, Vision, MedicalRef, LegalRef}`.
Each has `eval_slice()` — a *name*, not a published bench run.
`is_practice_license()` is false on every variant.

Medical/legal `answer(cited, uncertainty)` requires both flags or `MissingCitation`.

`Executed::code(false, _)` and `Executed::proof(false)` stay Tier 4 with an error. That is the #103 stub. Not a sandbox product.

Science / EO live models stay on #48. Not this slice.
`gaia_certifies_usmle_or_bar()` stays false.

## Refuse

- practice license from a pack
- unverified proof as Tier 1
- live code runner / live Earth product ingest
