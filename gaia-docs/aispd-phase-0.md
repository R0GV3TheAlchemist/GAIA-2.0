# AISPD Phase 0 first cut (#145 / #149 / #150)

Working notes. Not a v1.0 tag. Not a sentience claim. Not a safety certification.

- Nine realms live as `gaia_aispd::REALMS` (`catalog.rs`).
- `agency`, `recursion`, and `agi_watch` are `Containment`-class. `gaia_enabled = false` for all realms.
- Jagged-intelligence map: `jagged_score(realm)` returns `Unknown` for all realms at Phase 0 — no benchmark yet.
- Ethics charter in `charter.rs`: six principles, five prohibited items (`rsi_autolaunch`, `agi_marketing`, `capability_inflation`, `containment_bypass`, `oversight_removal`).
- `charter::check()` runs in CI and MUST return `Ok(())` on every commit to `main`.
- `aispd_v1_tagged() == false`. No AISPD v1.0 tag exists.
- Normative spec: `gaia-spec/aispd/PHASE-0.md`.
