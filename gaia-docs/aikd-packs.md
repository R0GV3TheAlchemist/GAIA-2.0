# AIKD packs first cut (#101–#104)

Working notes. Not AIKD v1.0. Not GraphCast or LLaVA.

- Science answers live as `gaia_aikd::ScienceAnswer`.
- Every science pack must carry a non-empty Earth Twin product id (e.g. `et:climate:albedo`).
  An empty id fails with `AikdError::MissingCitation`.
- Medical, legal, and finance packs always include a disclaimer string produced by
  `gaia_aikd::professional_disclaimer(kind)`, which returns `"{kind}: not professional advice"`.
- GAIA 2.0 does **not** certify USMLE or Bar exams as GAIA credentials.
  `gaia_aikd::gaia_certifies_usmle_or_bar()` returns `false`; no code path overrides this.
- Insurer and employer automation flows are refused at this layer.
  `gaia_aikd::insurer_automation()` returns `Err(AikdError::ClosedScoreClaim)` unconditionally.
- There is no GraphCast, ESFM, LLaVA, or live structure-prediction call wired into `ScienceAnswer`.
  Fixture text only; real science backends are future work.
- AIKD v1.0 is not tagged. Professional packs are reference surfaces, not practice licenses.

Packs follow the AIKD Phase 2 ethics and the #73 pattern:
cite-or-refuse, no hidden risk scores, no ambient automation of underwriting or employment decisions.

See `gaia-aikd/src/packs.rs` for the implementation and
`gaia-aikd/tests/packs.rs` for the single test that locks in all four acceptance conditions.
