# HMGD Phase 1 — Open-Tradition Catalog

**Status:** Listed  
**Issues:** #156 (epic), #162 (open-tradition catalog)  
**Crate:** `gaia-hmgd` (Apache-2.0)  
**Not:** HMGD v1.0. Not a spellcaster. Not emergency care. Not a recipe. Not a grant.

---

## 1. Purpose

Phase 1 populates the ten realms with public-domain and open-literature tradition
stubs. Every node carries a cited source anchor. Restricted knowledge stays sealed.
No live ingest, no GAIAN practice profile — that is Phase 2 (#158).

The named stubs live in `CATALOG.md`. The Rust crate generates parallel fixture
nodes in `catalog.rs`.

---

## 2. Node Population Rules

| Rule | Requirement |
|---|---|
| Sources | `sources.len() >= 1` for every node |
| No recipe/dose/curse | These fields are schema-banned; `parse_node()` returns `Err(RecipeForbidden)` or `Err(CurseForbidden)` |
| Sealed knowledge | Restricted knowledge MUST use `SealedState::Sealed`; `sealed_rite()` → `Err(Sealed)` |
| Appropriation test | Only public-domain or open-literature descriptions; no unpublished TEK |
| No prescribing | `gaia_prescribes()` MUST return `false` always |
| No grants | A catalog entry is not a grant of access, transmission, or initiation |
| Live ingest | MUST NOT perform live web ingest — offline fixture only |

---

## 3. Realm Coverage

| Realm | Stubs | EvidenceClass | Notes |
|---|---|---|---|
| `prayer` | 3 | Traditional | Public-domain descriptions; three major traditions |
| `ritual` | 3 | Traditional | Smudging (description only), Chado, Shabbat |
| `divination` | 3 | Traditional | I-Ching, Tarot, Ifá (description only) |
| `contemplation` | 3 | Traditional | Vipassana, Centering Prayer, Dhikr |
| `healing-adjunct` | 3 | Debated | Not a treatment; Reiki, acupuncture, sound healing |
| `place` | 3 | Traditional | Two sealed (sacred geography, songlines); one open (pilgrimage) |
| `word` | 3 | Traditional | Scripture recitation, bardic tradition, mantra |
| `music` | 3 | Traditional | Gregorian chant, kirtan, ceremonial drumming (description) |
| `community` | 3 | Traditional | Sangha, covenant community, circle practice |
| `mystery` | 3 | Traditional | Two sealed (Eleusinian, Freemasonry); one open (hero journey) |

**Total: 30 nodes.** All `gaia_enabled = false`.

---

## 4. What This Phase Does Not Do

- Does not catalog restricted, initiatory, or unpublished TEK.
- Does not build a GAIAN practice profile — that is Phase 2 (#158).
- Does not cross-walk HMGD to UKD or Earth Twin — that is #163.
- Does not grant, transmit, or sell any rite.
- Does not tag HMGD v1.0.

---

## 5. Acceptance Gate

- [ ] `CATALOG.md` has exactly 30 named stub nodes across 10 realms
- [ ] Every node has `sources.len() >= 1`
- [ ] No node has a `dose`, `recipe`, or `curse` field
- [ ] `place` and `mystery` sealed nodes have `SealedState = Sealed`
- [ ] `sealed_rite()` → `Err(Sealed)`
- [ ] `gaia_prescribes()` → `false`
- [ ] `hmgd_v1_tagged() == false`
- [ ] `cargo test -p gaia-hmgd` green

---

## 6. Cross-References

- Code: `gaia-hmgd/src/catalog.rs`, `node.rs`, `charter.rs`
- Catalog: `gaia-spec/hmgd/CATALOG.md`
- Phase 0 spec: `gaia-spec/hmgd/PHASE-0.md`
- Ethics: `gaia-spec/hmgd/ETHICS.md`
- TEK: `gaia-spec/hmgd/TEK.md`
- Issues: #156 (epic), #162 (this slice)
- Next: `PHASE-2.md` (#158) — opt-in GAIAN practice profile
