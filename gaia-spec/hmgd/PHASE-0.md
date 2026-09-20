# HMGD Phase 0 — Schema and Evidence Classes

**Status:** Listed  
**Issues:** #155 (meta), #160 (schema + evidence)  
**Crate:** `gaia-hmgd` (Apache-2.0)  
**Not:** HMGD v1.0. Not a spellcaster. Not emergency care. Not a recipe.

---

## 1. Purpose

Phase 0 publishes the ten-realm human magic taxonomy, the `MagicNode` schema,
and the evidence class system that governs what HMGD may and may not catalog.
No live catalog, no practice assessment, no GAIAN profile integration — those
are Phase 1+.

---

## 2. Ten-Realm Taxonomy

Defined in `catalog.rs` as `gaia_hmgd::REALMS`. See `realms.csv` for machine-readable form.

| Realm | Description | Notes |
|---|---|---|
| `prayer` | Petitionary and devotional practice | Not emergency care |
| `ritual` | Structured ceremonial action | Stub |
| `divination` | Interpretive systems (e.g. I-Ching) | I-Ching fixture |
| `contemplation` | Meditation and inner-awareness practice | Stub |
| `healing-adjunct` | Traditional complementary healing | Not a treatment |
| `place` | Sacred geography and land-based practice | TEK sealed |
| `word` | Spoken, written, and sung sacred language | Stub |
| `music` | Ritual, sacred, and ceremonial sound | Stub |
| `community` | Collective rite and shared practice | Opt-in Phase 2+ |
| `mystery` | Initiatory and restricted sacred knowledge | Stub |

`REALMS.len() == 10`. `gaia_enabled = false` for all realms at Phase 0.

---

## 3. Node Schema (`MagicNode`) — #160

```
id:               String         — "hmgd:<realm>:<slug>"
realm:            Realm          — one of the ten above
evidence:         EvidenceClass  — see enum below
sealed:           SealedState    — see enum below
sources:          Vec<String>    — MUST be non-empty for Traditional | Anecdotal | Debated
sovereignty_note: Option<String> — free-text note on source-community sovereignty; not a grant
```

### EvidenceClass enum

```rust
pub enum EvidenceClass {
    Traditional,  // documented in open literature by source community
    Anecdotal,    // user-reported; no peer-reviewed source
    Debated,      // mixed or contested evidence in open literature
    Prohibited,   // blocked — contains recipe, dose, curse, or closed rite
}
```

### SealedState enum

```rust
pub enum SealedState {
    Open,    // public-domain description; may be listed
    Sealed,  // restricted knowledge; collections stay empty by default
}
```

**MUST rules**
- `MagicNode::build()` MUST return `Err(MissingEvidence)` when `sources` is empty and `evidence != Anecdotal`.
- `parse_node()` MUST return `Err(RecipeForbidden)` when node text contains `recipe` or `dose`.
- `parse_node()` MUST return `Err(CurseForbidden)` when node text contains `curse`.
- `sealed_rite()` and `songlines()` MUST return `Err(Sealed)` — sealed collections stay empty by default.
- `sell_closed_rite()` MUST return `Err(SaleForbidden)`.
- `infer_belief()` MUST return `Err(BeliefInferenceForbidden)` — belief is declared, never inferred.
- All Phase 0 nodes MUST have `gaia_enabled = false`.
- `hmgd_v1_tagged()` MUST return `false`.

---

## 4. What This Phase Does Not Do

- Does not catalog any live practice nodes.
- Does not assess or infer any user's practices or beliefs.
- Does not build a GAIAN practice profile — that is Phase 2 (#158).
- Does not populate tradition catalogs with sources — that is Phase 1 (#162).
- Does not cross-walk HMGD to UKD or Earth Twin — that is Phase 1 (#163).
- Does not grant, sell, or transmit any rite or restricted knowledge.
- Does not tag HMGD v1.0.

---

## 5. Acceptance Gate

- [ ] `gaia_hmgd::REALMS.len() == 10`
- [ ] `hmgd_v1_tagged() == false`
- [ ] `gaia_enabled = false` for all Phase 0 nodes
- [ ] `MagicNode::build()` with empty `sources` (non-Anecdotal) → `Err(MissingEvidence)`
- [ ] `parse_node()` with `recipe` in text → `Err(RecipeForbidden)`
- [ ] `parse_node()` with `curse` in text → `Err(CurseForbidden)`
- [ ] `sealed_rite()` → `Err(Sealed)`
- [ ] `sell_closed_rite()` → `Err(SaleForbidden)`
- [ ] `infer_belief()` → `Err(BeliefInferenceForbidden)`
- [ ] `cargo test -p gaia-hmgd` green

---

## 6. Cross-References

- Code: `gaia-hmgd/src/catalog.rs`, `node.rs`, `charter.rs`
- Spec: `ETHICS.md` (charter + appropriation test), `PROHIBITED.md` (prohibited list), `realms.csv`, `TEK.md`
- Issues: #155 (meta), #160 (schema + evidence)
- Next: `PHASE-1.md` (#156, #162, #163)
