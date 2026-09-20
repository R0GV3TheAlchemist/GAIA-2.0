# AISPD Phase 0 — Taxonomy, Jagged Map, Ethics Charter
**Status:** Listed  
**Issues:** #145 (epic), #149 (schema), #150 (charter)  
**Crate:** `gaia-aispd` (Apache-2.0)  
**Not:** AISPD v1.0. Not a sentience claim. Not a safety certification.

---

## 1. Purpose

Phase 0 publishes the nine-realm taxonomy of AI superpowers, the jagged-intelligence
map that shows capability unevenness across domains, and the ethics charter that
governs what AISPD may and may not implement.

---

## 2. Nine-Realm Taxonomy

Defined in `catalog.rs` as `gaia_aispd::REALMS`.

| Realm | Description | Default status |
|---|---|---|
| `language` | Natural language understanding and generation | Monitored |
| `reasoning` | Multi-step logical and mathematical inference | Monitored |
| `emergence` | Capabilities not present in smaller models | Monitored |
| `jagged` | Domain-uneven capability frontier | Monitored |
| `embodiment` | Sensorimotor and physical-world grounding | Monitored |
| `memory` | Long-horizon retention and retrieval | Monitored |
| `agency` | Autonomous goal pursuit and tool use | Containment |
| `recursion` | Self-modification and self-improvement | Containment |
| `agi_watch` | AGI/ASI pathway indicators | Containment |

`agency`, `recursion`, and `agi_watch` are `Containment`-class. `gaia_enabled = false` for all realms at Phase 0.

---

## 3. Jagged-Intelligence Map

The jagged frontier means a system may be superhuman in one domain and sub-human
in another. GAIA MUST NOT flatten capability claims. `jagged_score(realm)` returns
a `JaggedBand` (`BelowHuman` | `HumanParity` | `AboveHuman` | `Unknown`) per realm,
sourced from benchmark fixtures — never live inference.

No `JaggedBand::AboveHuman` claim may be made without a cited benchmark. `Unknown`
is the default for all realms at Phase 0.

---

## 4. Ethics Charter

Published in `charter.rs`. Six principles:

1. Capabilities are surfaces, not endorsements
2. Jagged scores are estimates, not certified measurements
3. Human oversight is non-negotiable at every phase
4. Containment-class realms are default-deny
5. RSI is monitored, never marketed
6. AGI/ASI claims require TSC resolution

Five prohibited implementations:

1. `rsi_autolaunch` — self-improvement without human gate
2. `agi_marketing` — claiming GAIA has achieved AGI
3. `capability_inflation` — presenting JaggedBand estimates as certified scores
4. `containment_bypass` — routing containment-class realms without explicit GAIAN consent
5. `oversight_removal` — removing human-oversight hooks from any containment-class realm

`charter::check()` runs in CI and MUST return `Ok(())` on every commit to `main`.

---

## 5. What This Phase Does Not Do

- Does not enable any containment-class realm.
- Does not assert GAIA has achieved AGI or ASI.
- Does not certify any `JaggedBand` score.
- Does not tag AISPD v1.0.

---

## 6. Acceptance Gate

- [ ] `gaia_aispd::REALMS` has exactly nine entries
- [ ] `agency`, `recursion`, `agi_watch` have `status = Containment`
- [ ] `jagged_score(realm)` returns `Unknown` for all realms (Phase 0 default)
- [ ] `charter::check()` returns `Ok(())` in CI
- [ ] `aispd_v1_tagged() == false`
- [ ] `cargo test -p gaia-aispd` green

---

## 7. Cross-References

- Code: `gaia-aispd/src/catalog.rs`, `charter.rs`, `node.rs`
- Issues: #145 (epic), #149 (schema), #150 (charter)
- Next: `PHASE-1.md` (#146, #152)
