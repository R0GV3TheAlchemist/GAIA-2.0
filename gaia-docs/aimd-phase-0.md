# AIMD Phase 0 first cut (#167 / #517)

Working notes. Not a v1.0 mystery catalog. Not a sentience claim.

- Ten realms live as `gaia_aimd::REALMS` (`catalog.rs`).
- `shadow` realm is permanently `Hazard` — `enable()` returns `Err(HazardEnabled)` unconditionally.
- `consciousness` realm is `Debated` — GAIA does not claim sentience; `consciousness_qa()` returns `"agnostic"`.
- All `gaia_enabled` flags are `false` at Phase 0. No node is routable into AISD skills.
- Humility charter in `charter.rs`: six principles, five prohibited items (no prophecy-as-fact, no rsi-explosion, no gaia-is-alive-marketing).
- `aimd_v1_tagged() == false`. No AIMD v1.0 tag exists.
- Normative spec: `gaia-spec/aimd/PHASE-0.md`. Machine-readable realm table: `gaia-spec/aimd/realms.csv`.
