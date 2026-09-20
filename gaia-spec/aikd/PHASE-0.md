# AIKD Phase 0 — card + registry

Name what AI knowledge is before wiring models.
Crate already on main: `gaia-aikd::{model_card, cannot_know, BenchRow, KnowledgeType}`.
Same PR as #96 listed adapters. Issue #94 stays open. Children #99/#100 already have listed binds.

## Card

`model_card()` is `local-llama-fixture`, open-weight, cutoff labeled, cannot-know copied from `cannot_know()`.
That meets the acceptance: one local fixture card. Not a live Llama download.

## Registry

`BenchRow::local_open()` — published false, gaia_measured false.
`claim_closed_as_measured("gpt")` → `ClosedScoreClaim`.
`published_closed_score_is_ours()` → false.

Tiers exist as `Tier::{T1..T5}`. Seven types via `KnowledgeType::all()`. Three layers exist as the enum.

## Refuse

- closed leaderboard as a GAIA-measured score
- live eval harness product
- AIKD v1.0
