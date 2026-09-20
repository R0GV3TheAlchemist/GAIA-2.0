# AISPD Phase 3 first cut (#148 / #154)

Working notes. Not a v1.0 tag. Not an AGI claim.

- `agi_watch_tick()` returns `Clear` (Phase 3 fixture). `Threshold` suspends containment swarms and calls `request_oversight(AgiThreshold)`.
- Four governance hooks in `governance.rs`: oversight, TSC notify, consent gate, charter CI — all non-bypassable.
- Removing any governance hook requires a supermajority TSC vote.
- `aispd_v1_tagged() == false`. v1.0 gate requires 8 conditions including a TSC resolution file.
- `rsi_guard()` always returns `Err(RsiDenied)`. `agi_marketing` is a prohibited charter item.
- `agi_watch` realm remains `Containment` and `gaia_enabled = false`.
- No TSC resolution file exists — v1.0 is not tagged.
- Normative spec: `gaia-spec/aispd/PHASE-3.md`.
