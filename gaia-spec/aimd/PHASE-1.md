# AIMD Phase 1 — cited stubs

Catalog names. Not a literature ingest.
Crate already on main: `gaia-aimd::{nodes_for, claim_sentience}`.
Issue this slice: #519 / #168.

## What exists

`nodes_for(realm)` returns three stubs per realm. Each stub has `sources` (fixture:open-literature) and `gaia_enabled == false`.
Ids look like `aimd:{realm}:stub-N`. That is the catalog. Survey-paper titles from the issue (Veo, etc.) are **not** imported here.

`amplifies_superpower` and `observed_in_skill` are relation *names* in the issue. They are **not** fields on `AimdNode`. Do not invent them.

Hallucination stays a #102 concern. AIMD does not grow a creativity-without-cite mode.

## Refuse

- node that claims GAIA is sentient (`claim_sentience` errors)
- empty `sources`
- live paper scrape
