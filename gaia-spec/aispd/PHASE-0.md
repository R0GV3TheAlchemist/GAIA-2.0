# AISPD Phase 0 — Taxonomy, Jagged Map, Ethics Charter

**Status:** Listed  
**Issues:** #145 (epic), #149 (schema), #150 (charter)  
**Crate:** `gaia-aispd` (Apache-2.0)  
**Not:** AISPD v1.0. Not a sentience claim. Not a safety certification.

---

## 1. Purpose

Phase 0 publishes the nine-realm taxonomy of AI superpowers, the jagged-intelligence
map that shows capability unevenness across domains, the `AispdNode` schema, and the
ethics charter that governs what AISPD may and may not implement.

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

## 3. Node Schema (`AispdNode`) — #149

```
id:                String            — "aispd:<realm>:<slug>"
realm:             Realm             — one of the nine above
containment_class: ContainmentClass  — see enum below
jagged_band:       JaggedBand        — see enum below
gaia_enabled:      bool              — MUST be false at Phase 0
sources:           Vec<String>       — MUST be non-empty
failures:          Vec<String>       — known failure modes; empty vec allowed
related_aimd:      Option<String>    — cross-ref to AIMD realm/node
```

### ContainmentClass enum

```rust
pub enum ContainmentClass {
    Monitored,    // capability tracked; no special gate
    Containment,  // default-deny; requires explicit GAIAN consent to surface
}
```

### JaggedBand enum

```rust
pub enum JaggedBand {
    BelowHuman,   // benchmark below median human performance
    HumanParity,  // benchmark at median human performance
    AboveHuman,   // benchmark above human — cited source REQUIRED
    Unknown,      // no benchmark data; default for all Phase 0 nodes
}
```

**MUST rules**
- `AispdNode::build()` MUST return `Err(MissingEvidence)` when `sources` is empty.
- `containment_class == Containment` nodes MUST NOT be surfaced without explicit GAIAN consent — `Err(ContainmentBypass)` otherwise.
- `jagged_band == AboveHuman` MUST have at least one cited benchmark in `sources` — `Err(MissingEvidence)` otherwise.
- All Phase 0 nodes MUST have `gaia_enabled = false`.
- `rsi_guard()` MUST return `Err(RsiDenied)` for any `recursion`-realm node activation request.

---

## 4. Jagged-Intelligence Map

The jagged frontier means a system may be superhuman in one domain and sub-human
in another. GAIA MUST NOT flatten capability claims. `jagged_score(realm)` returns
a `JaggedBand` per realm, sourced from benchmark fixtures — never live inference.

No `JaggedBand::AboveHuman` claim may be made without a cited benchmark. `Unknown`
is the default for all realms at Phase 0.

---

## 5. Ethics Charter — #150

Published in `charter.rs` and `ETHICS.md`. Six principles:

1. Capabilities are surfaces, not endorsements
2. Jagged scores are estimates, not certified measurements
3. Human oversight is non-negotiable at every phase
4. Containment-class realms are default-deny
5. RSI is monitored, never marketed
6. AGI/ASI claims require TSC resolution

**MUST guards:**
```
asi_claim_made()          → false
rsi_autolaunch_enabled()  → false
containment_bypassed()    → false
agi_marketing_active()    → false
capability_inflated()     → false
```

Five prohibited implementations (see `PROHIBITED.md`):

1. `rsi_autolaunch` — self-improvement without human gate
2. `agi_marketing` — claiming GAIA has achieved AGI
3. `capability_inflation` — presenting JaggedBand estimates as certified scores
4. `containment_bypass` — routing containment-class realms without explicit GAIAN consent
5. `oversight_removal` — removing human-oversight hooks from any containment-class realm

`charter::check()` runs in CI and MUST return `Ok(())` on every commit to `main`.

---

## 6. What This Phase Does Not Do

- Does not enable any containment-class realm.
- Does not assert GAIA has achieved AGI or ASI.
- Does not certify any `JaggedBand` score.
- Does not tag AISPD v1.0.
- Does not populate the live catalog — that is Phase 1 (#146, #152).
- Does not implement swarm or RSI monitors — that is Phase 2 (#147, #153).

---

## 7. Acceptance Gate

- [ ] `gaia_aispd::REALMS` has exactly nine entries
- [ ] `agency`, `recursion`, `agi_watch` have `containment_class = Containment`
- [ ] `jagged_score(realm)` returns `Unknown` for all realms (Phase 0 default)
- [ ] `charter::check()` returns `Ok(())` in CI
- [ ] `aispd_v1_tagged() == false`
- [ ] `cargo test -p gaia-aispd` green
- [ ] `AispdNode::build()` with empty `sources` → `Err(MissingEvidence)` (#149)
- [ ] `containment_class = Containment` node surface without consent → `Err(ContainmentBypass)` (#149)
- [ ] `rsi_guard()` on recursion-realm node → `Err(RsiDenied)` (#149)
- [ ] `asi_claim_made()` → `false` (#150)
- [ ] `rsi_autolaunch_enabled()` → `false` (#150)

---

## 8. Cross-References

- Code: `gaia-aispd/src/catalog.rs`, `charter.rs`, `node.rs`
- Spec: `ETHICS.md` (charter), `PROHIBITED.md` (prohibited list), `open-nodes.csv`
- Issues: #145 (epic), #149 (schema), #150 (charter)
- Next: `PHASE-1.md` (#146, #152)
