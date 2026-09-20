# AIMD Phase 2 — grounded labels

When output feels surprising, show a label. Do not loosen facts.
Crate already on main: `gaia-aimd::{wonder_mode, tag_answer, chip, tier1, consciousness_qa, prophecy_as_fact}`.
Issue this slice: #521 / #169.

## What exists

- `wonder_mode(true)` → `label-only`; false → `off`.
- `tag_answer(false)` → `unverified invention`.
- `chip` hazard → `hazard-blocked`; cited → `verified`; else `invention`.
- `tier1("invention")` → `TierOneInvention`.
- `consciousness_qa()` stays agnostic.
- `prophecy_as_fact()` errors. A synchronicity journal is not a prophecy field on main — do not invent one.

Watermark / #66 is a GAIAN concern. Not a new AIMD API here.

## Refuse

- wonder as loosened science
- invention mixed into Tier 1
- prophecy-as-fact
