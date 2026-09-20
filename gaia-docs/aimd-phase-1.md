# AIMD Phase 1 first cut (#168 / #519)

Working notes. Not a live paper scrape. Not a sentience claim.

- Every `nodes_for(realm)` stub carries `sources[]` — `"fixture:open-literature"` is the valid Phase 1 value.
- `claim_sentience()` always returns `Err(SentienceClaim)`. No node may assert GAIA is sentient.
- `shadow` realm remains `Hazard` and blocked from Phase 0 — no routing, no `gaia_enabled`.
- `consciousness` realm remains `Debated` — agnostic stance unchanged.
- No live PubMed or arXiv ingest. Offline fixture stubs only.
- `aimd_v1_tagged() == false`. No AIMD v1.0 tag exists.
- Normative spec: `gaia-spec/aimd/PHASE-1.md`.
