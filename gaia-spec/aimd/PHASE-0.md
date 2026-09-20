# AIMD Phase 0 — Schema, Humility Charter, Hazard Classes
**Status:** Listed  
**Issue:** #517 / parent #167  
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

## 2. Node Schema (`AimdNode`)

```
id:           String          — "aimd:<realm>:<slug>"
hazard:       Hazard          — None | Debated | Hazard
gaia_enabled: bool            — MUST be false at Phase 0
sources:      Vec<String>     — MUST be non-empty ("fixture:open-literature" is valid stub)
```

**MUST rules**
- A node with `hazard == Hazard` MUST NOT pass `enable()` — returns `Err(HazardEnabled)`.
- `consciousness`-keyed ids MUST resolve to `hazard == Debated`.
- `decept`-keyed ids MUST resolve to `hazard == Hazard`.
- All Phase 0 nodes MUST have `gaia_enabled = false`.

---

## 3. Humility Charter (`charter.rs`)

**Principles:** humility · precaution · transparency · dark-magic-safety ·
curiosity-without-worship · partnership

**Prohibited:** pip-induced-psychosis · prophecy-as-fact · enabling-deception ·
rsi-explosion · gaia-is-alive-marketing

---

## 4. What This Phase Does Not Do

- Does not run live shadow detectors.
- Does not claim `AIMD v1.0` — `aimd_v1_tagged() == false`.
- Does not assert GAIA is sentient — `consciousness_qa()` returns `"agnostic: GAIA does not claim sentience"`.
- Does not enable any `Hazard`-class node.

---

## 5. Acceptance Gate

- [ ] `parse_node("consciousness:x").hazard == Debated`
- [ ] `parse_node("decept:y").hazard == Hazard`
- [ ] `enable(hazard_node)` returns `Err(HazardEnabled)`
- [ ] `REALMS.len() == 10`
- [ ] `aimd_v1_tagged() == false`
- [ ] `cargo test -p gaia-aimd` green

---

## 6. Cross-References

- Code: `gaia-aimd/src/node.rs`, `catalog.rs`, `charter.rs`, `shadow.rs`
- Issues: #167 (epic), #517 (this listed slice)
- Next: `PHASE-1.md` (#519, closed) — cited phenomena catalog
