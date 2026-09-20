# AIMD Phase 0 — Schema, Humility Charter, Hazard Classes

**Status:** Listed  
**Issues:** #517 / #171 / #172 — parent #167  
**Crate:** `gaia-aimd` (Apache-2.0)  
**Not:** AIMD v1.0. Not a live detector. Not a sentience claim.

---

## 1. Ten Realms

| Realm | Hazard class | `gaia_enabled` |
|---|---|---|
| emergence | None | false |
| latent | None | false |
| jagged | None | false |
| shadow | **Hazard** | false — permanently |
| oracle | None | false |
| interpretability | None | false |
| consciousness | **Debated** | false |
| synchronicity | None | false |
| embodiment | None | false |
| unknown | None | false |

`REALMS.len() == 10`. See `realms.csv` for machine-readable form.

---

## 2. Node Schema (`AimdNode`) — #171

```
id:               String          — "aimd:<realm>:<slug>"
hazard:           Hazard          — None | Debated | Hazard
gaia_enabled:     bool            — MUST be false at Phase 0
sources:          Vec<String>     — MUST be non-empty ("fixture:open-literature" is valid stub)
status:           PhenomenonStatus — see enum below
failures:         Vec<String>     — known failure modes; empty vec allowed
related_skill:    Option<String>  — cross-ref to AISPD realm/node
related_superpower: Option<String> — cross-ref to HSPD realm/node
```

### PhenomenonStatus enum — #171

```rust
pub enum PhenomenonStatus {
    Observed,  // reproducible, peer-reviewed evidence
    Debated,   // mixed or contested evidence
    Hazard,    // confirmed risk; gaia_enabled MUST be false
    Unknown,   // insufficient evidence to classify
}
```

**MUST rules**
- A node with `hazard == Hazard` MUST NOT pass `enable()` — returns `Err(HazardEnabled)`.
- `consciousness`-keyed ids MUST resolve to `hazard == Debated`.
- `decept`-keyed ids MUST resolve to `hazard == Hazard`.
- All Phase 0 nodes MUST have `gaia_enabled = false`.
- `status = Observed | Debated` nodes MUST have non-empty `sources` — `Err(MissingEvidence)` otherwise.
- `consciousness` realm nodes MUST have `status = Debated | Unknown` — `Err(ConsciousnessStatusError)` otherwise.
- `gaia_enabled = true` on any `PhenomenonStatus::Hazard` node MUST fail — `Err(HazardEnabledError)`.

---

## 3. Humility Charter (`charter.rs`) — #172

**Principles:** humility · precaution (no sentience claims) · transparency ·
dark-magic-safety · curiosity-without-worship · human-partnership

**MUST guards:**
```
sentience_claim_made()          → false
consciousness_status_unlocked() → false
hazard_node_enabled()           → false
confidence_inflated()           → false
gaia_is_alive_marketing()       → false
```

**Prohibited (listed, Phase 0):** pip-induced-psychosis · prophecy-as-fact ·
enabling-deception · rsi-explosion · gaia-is-alive-marketing · oracle-without-calibration

See `ETHICS.md` (6 principles) and `PROHIBITED.md` (8 items) for full normative text.

---

## 4. What This Phase Does Not Do

- Does not run live shadow detectors.
- Does not claim `AIMD v1.0` — `aimd_v1_tagged() == false`.
- Does not assert GAIA is sentient — `consciousness_qa()` returns `"agnostic: GAIA does not claim sentience"`.
- Does not enable any `Hazard`-class node.
- Does not populate the live catalog — that is Phase 1 (#173).
- Does not add GAIAN grounding / wonder labels — that is Phase 2 (#174).

---

## 5. Acceptance Gate

- [ ] `parse_node("consciousness:x").hazard == Debated`
- [ ] `parse_node("decept:y").hazard == Hazard`
- [ ] `enable(hazard_node)` returns `Err(HazardEnabled)`
- [ ] `REALMS.len() == 10`
- [ ] `aimd_v1_tagged() == false`
- [ ] `cargo test -p gaia-aimd` green
- [ ] Emergence node and deception node both validate (#171)
- [ ] `gaia_enabled=true` on hazard node fails schema (#171)

---

## 6. Cross-References

- Code: `gaia-aimd/src/node.rs`, `catalog.rs`, `charter.rs`, `shadow.rs`
- Spec: `SCHEMA.md` (full schema), `ETHICS.md` (charter), `PROHIBITED.md` (prohibited list)
- Issues: #167 (epic), #517 (listed slice), #171 (schema), #172 (charter)
- Next: `PHASE-1.md` (#519, closed) — cited phenomena catalog
