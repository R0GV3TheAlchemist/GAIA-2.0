# HSPD Phase 0 — Taxonomy, Schema, Ethics Charter

**Status:** Listed  
**Issues:** #133 (epic), #137 (taxonomy + schema), #138 (ethics charter)  
**Crate:** `gaia-hspd` (Apache-2.0)  
**Not:** HSPD v1.0. Not a clinic. Not a prescription. Not a genetic test.

---

## 1. Purpose

Phase 0 publishes the eight-realm human superpower taxonomy, the `SuperpowerNode`
schema, and the ethics charter that governs what HSPD may and may not implement.
No live catalog, no assessment, no GAIAN profile integration — those are Phase 1+.

---

## 2. Eight-Realm Taxonomy

Defined in `catalog.rs` as `gaia_hspd::REALMS`.

| Realm | Description | Default risk class |
|---|---|---|
| `cognitive` | Memory, focus, learning, reasoning enhancement | None |
| `physical` | Strength, endurance, coordination, recovery | None |
| `longevity` | Healthspan and lifespan extension practices | Debated |
| `sensory` | Enhanced perception — visual, auditory, proprioceptive | None |
| `emotional` | Emotional regulation, resilience, social attunement | None |
| `spiritual` | Contemplative, flow, and transcendent practice | None |
| `augmented` | Technological and pharmacological augmentation | Medical |
| `genetic` | Genomic and epigenetic factors | Prohibited |

`augmented` is `Medical`-class — no DIY path. `genetic` is `Prohibited`-class — inference blocked. `gaia_enabled = false` for all realms at Phase 0.

---

## 3. Node Schema (`SuperpowerNode`) — #137

```
id:          String      — "hspd:<realm>:<slug>"
realm:       Realm       — one of the eight above
kind:        KindClass   — see enum below
risk_class:  RiskClass   — see enum below
gaia_enabled: bool       — MUST be false at Phase 0
sources:     Vec<String> — MUST be non-empty
failures:    Vec<String> — known risk notes; empty vec allowed
```

### KindClass enum

```rust
pub enum KindClass {
    Natural,    // innate or genetic trait; not a gene call
    Trained,    // developed through practice; safe for all ages as interest
    Augmented,  // technological or pharmacological; adult only
    Genetic,    // genomic factor; prohibited from inference
}
```

### RiskClass enum

```rust
pub enum RiskClass {
    None,       // no known harm path
    Debated,    // potential benefit or harm under research
    Medical,    // professional oversight required; no DIY path
    Prohibited, // blocked — no GAIA call path may surface this
}
```

**MUST rules**
- `SuperpowerNode::build()` MUST return `Err(MissingEvidence)` when `sources` is empty.
- `kind == KindClass::Augmented` nodes with `age_context == UnderSixteen` MUST attach `ChildTag` — `Err(ChildTagRequired)` otherwise.
- `risk_class == RiskClass::Medical` nodes MUST NOT expose a `dose`, `protocol`, or `stack` field — `Err(MedicalDiyPath)` otherwise.
- `kind == KindClass::Genetic` nodes MUST NOT be constructed from photo, file inference, or any automated signal — `Err(GeneticInference)` otherwise. User self-declaration only.
- All Phase 0 nodes MUST have `gaia_enabled = false`.
- `hspd_v1_tagged()` MUST return `false`.

---

## 4. What This Phase Does Not Do

- Does not run a live catalog of nodes.
- Does not assess any user's superpowers.
- Does not build a GAIAN practice profile — that is Phase 2 (#135).
- Does not provide learning paths or resources — that is Phase 1 (#134).
- Does not prescribe, dose, or protocol any augmentation.
- Does not infer genetics from photos, files, or any automated signal.
- Does not tag HSPD v1.0.

---

## 5. Acceptance Gate

- [ ] `gaia_hspd::REALMS` has exactly eight entries
- [ ] `augmented` realm has `risk_class = Medical`
- [ ] `genetic` realm has `risk_class = Prohibited`
- [ ] `SuperpowerNode::build()` with empty `sources` → `Err(MissingEvidence)`
- [ ] `Augmented` node under-16 without `ChildTag` → `Err(ChildTagRequired)`
- [ ] `Medical` node with `dose` field → `Err(MedicalDiyPath)`
- [ ] `Genetic` node from inferred signal → `Err(GeneticInference)`
- [ ] `infer_actn3()` returns `Err(GeneticInference)`
- [ ] `hspd_v1_tagged() == false`
- [ ] `cargo test -p gaia-hspd` green

---

## 6. Cross-References

- Code: `gaia-hspd/src/catalog.rs`, `node.rs`, `charter.rs`
- Spec: `ETHICS.md` (charter), `PROHIBITED.md` (prohibited list), `PROFILE.md` (GAIAN profile), `profile.csv`
- Issues: #133 (epic), #137 (taxonomy + schema), #138 (ethics charter)
- Next: `PHASE-1.md` (#134, #139, #140)
