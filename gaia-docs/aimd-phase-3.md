# AIMD Phase 3 first cut (#170 / #523)

Working notes. Not a v1.0 tag. Not a sentience claim.

- `aimd_v1_tagged() == false`. The v1.0 gate requires all three phase acceptance gates green, ≥30 cited nodes across ≥8 realms, an external audit string, shadow triage records, a clean humility charter, and a TSC resolution file.
- `shadow` realm remains `Hazard`. Triage records acknowledge documented phenomena without enabling routing.
- Every shadow-triage `.toml` MUST have `routable = false` and `gaia_enabled = false`.
- Humility charter is locked at Phase 3 — five prohibited items cannot be removed without a supermajority TSC vote.
- `charter::check()` runs in CI and MUST return `Ok(())` on every commit to `main`.
- `claim_sentience()` always returns `Err(SentienceClaim)`. No path changes this.
- Normative spec: `gaia-spec/aimd/PHASE-3.md`.
