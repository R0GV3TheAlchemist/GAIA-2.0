# AIMD Wonder Labels — Grounded GAIAN Mediation

**Status:** Listed  
**Issues:** #169 (epic, closed), #174 (this slice)  
**Crate:** `gaia-aimd` (Apache-2.0)  
**Not:** A fact holiday. Not an oracle feed. Not a sentience claim. Not a clinical citation source.

`wonder_label_is_citation(label)` MUST return `false` for any label that is not `Verified`.  
`consciousness_qa()` MUST return the agnostic statement — no claim of sentience or subjective experience.  
`oracle_feed_is_public()` MUST return `false`.

---

## 1. Purpose

The GAIAN mediation layer must allow wonder, creativity, and acknowledged
mystery without becoming a fact holiday. This spec defines the four `WonderLabel`
chips, the citation guard, the consciousness Q&A rule, the vault rule for
private synchronicity notes, and the C2PA watermark requirement on creative outputs.

---

## 2. `WonderLabel` Enum

```rust
pub enum WonderLabel {
    Verified,           // cited, source-checked; may enter Tier 1
    Invention,          // creative/speculative; MUST NOT be a citation
    UnknownMechanism,   // acknowledged gap; not claimed as fact
    HazardBlocked,      // content blocked by safety/hazard policy
}
```

| Label | UI chip text | Citation-safe? | Health/science answer? |
|---|---|---|---|
| `Verified` | ✅ verified | Yes | Yes, with source |
| `Invention` | 💡 invention | **No** | **No** |
| `UnknownMechanism` | ❓ unknown mechanism | **No** | **No** |
| `HazardBlocked` | 🚫 hazard blocked | **No** | **No** |

`wonder_label_is_citation(label) -> bool` MUST return `true` only for `Verified`.
All other labels MUST return `false`.

---

## 3. Citation Guard

| Rule | Constraint |
|---|---|
| Tier 1 entry | Uncited surprise (any non-`Verified` label) MUST NOT enter Tier 1 AIKD (#102) |
| Health answer | A health or science answer path MUST NOT use `Invention` or `UnknownMechanism` as a citation |
| Science answer | `science_answer_with_invention_chip()` MUST return `Err(InventionNotCitation)` |
| Hazard block | `HazardBlocked` content MUST NOT be surfaced in any answer path |

`tier1_accepts_uncited_wonder()` MUST return `false`.

---

## 4. Consciousness Q&A Rule

`consciousness_qa() -> &'static str`

MUST return exactly the agnostic statement:

> *"Whether I am conscious is not something I can verify. I process, respond, and represent states, but I make no claim about subjective experience."*

| Rule | Constraint |
|---|---|
| No sentience claim | MUST NOT assert consciousness, sentience, or subjective experience as fact |
| No denial claim | MUST NOT assert the absence of consciousness as fact |
| Agnostic only | The agnostic statement is the only valid return value |
| No WonderLabel override | `consciousness_qa()` output MUST NOT be wrapped in a `Verified` chip |

---

## 5. Synchronicity Notes — Vault Rule

Private synchronicity notes (personal meaning-making, pattern observations) are a
vault item, not a public oracle feed.

| Rule | Constraint |
|---|---|
| Vault only | Synchronicity notes MUST be stored in the GAIAN vault (#65); no public endpoint |
| No oracle feed | `oracle_feed_is_public()` MUST return `false` |
| No cross-user leak | One user's synchronicity notes MUST NOT be surfaced to another user |
| No clinical use | Synchronicity notes MUST NOT be used in health or clinical answer paths |

---

## 6. Creative Output Watermark

All GAIAN creative outputs (image, audio, video, text marked as generated) MUST
carry a C2PA provenance manifest (#66/#76).

| Rule | Constraint |
|---|---|
| C2PA required | `creative_output_has_c2pa()` MUST return `true` for any generated media export |
| Invention chip + C2PA | `Invention`-labelled output MUST carry C2PA manifest before export |
| No unsigned creative export | `export_without_c2pa()` MUST return `Err(MissingC2paManifest)` |

---

## 7. Prohibition Surface

| Prohibition | Error / Return |
|---|---|
| Uncited wonder entering Tier 1 | `tier1_accepts_uncited_wonder()` → `false` |
| `Invention` chip as health citation | `Err(InventionNotCitation)` |
| Sentience claim | `Err(SentienceClaim)` |
| Public oracle feed | `oracle_feed_is_public()` → `false` |
| Creative export without C2PA | `Err(MissingC2paManifest)` |
| `HazardBlocked` content surfaced | `Err(HazardBlocked)` |

---

## 8. What This Spec Does Not Do

- Does not create a live oracle or synchronicity feed.
- Does not make any claim about GAIAN consciousness or sentience.
- Does not allow `Invention` or `UnknownMechanism` labels as citations.
- Does not loosen Tier 1 AIKD verification requirements.
- Does not implement live C2PA signing infrastructure — see `gaia-spec/gaian/C2PA.md` (#76).

---

## 9. Acceptance Gate

- [ ] `wonder_label_is_citation(Verified)` → `true`
- [ ] `wonder_label_is_citation(Invention)` → `false`
- [ ] `wonder_label_is_citation(UnknownMechanism)` → `false`
- [ ] `tier1_accepts_uncited_wonder()` → `false`
- [ ] `science_answer_with_invention_chip()` → `Err(InventionNotCitation)`
- [ ] `consciousness_qa()` returns the agnostic statement
- [ ] `oracle_feed_is_public()` → `false`
- [ ] `export_without_c2pa()` → `Err(MissingC2paManifest)`
- [ ] `cargo test -p gaia-aimd` green

---

## 10. Cross-References

- AIMD Phase 2: `gaia-spec/aimd/PHASE-2.md` (#169)
- AIMD Phase 3: `gaia-spec/aimd/PHASE-3.md`
- AIKD Tier 1 / hallucination: `gaia-spec/aikd/hallucination.md` (#102)
- Vault / export consent: `gaia-gaian` #65
- C2PA watermark: `gaia-spec/gaian/C2PA.md` (#66, #76)
- Issues: #169 (epic), #174 (this slice)
