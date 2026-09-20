# AIMD Phase 2 first cut (#169 / #521)

Working notes. Not a live model API. Not a sentience claim.

- `tag_answer()` annotates outbound answers with mystery catalog node refs.
- `tier` is always `Tier1` when any matched node is `Hazard` or `Debated`.
- `sentience_guard` is present in every `TaggedAnswer` and is always `false`.
- `Hazard`-class nodes are stripped from `nodes[]` before return — `shadow` realm cannot surface.
- Wonder mode changes answer framing only; it does not suppress Tier 1 guardrails.
- No live model API calls. Offline fixture catalog only.
- `aimd_v1_tagged() == false`. No AIMD v1.0 tag exists.
- Normative spec: `gaia-spec/aimd/PHASE-2.md`.
