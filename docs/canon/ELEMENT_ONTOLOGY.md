# GAIA Element Ontology

**Canon document.** This file defines the complete set of valid values for the
`Governing Element` field in any GAIA Hermetic Tablet. It is the authoritative
reference for issue #808 (element field audit) and #828 (element field
verification CI check).

---

## The Five Classical Elements

GAIA's element ontology is rooted in the Western alchemical tradition while
being extended to include the non-classical elements used across the tablet
corpus. The five primary elements are ordered from densest (most material) to
most subtle (most spiritual).

| Element | Symbol | Quality | Direction | Season | Tablet association |
|---|---|---|---|---|---|
| Earth | 🜃 | Cold + Dry | North | Winter | Grounding, body, structure, matter |
| Water | 🜄 | Cold + Wet | West | Autumn | Emotion, flow, dissolution, the unconscious |
| Fire | 🜂 | Hot + Dry | South | Summer | Will, transformation, purification, Sol Niger |
| Air | 🜁 | Hot + Wet | East | Spring | Mind, communication, intellect, breath |
| Aether / Spirit | 🜀 | Quintessence | Centre / Above | Eternal | Unity, transcendence, the prima materia resolved |

## Extended Elements

The following elements appear in GAIA's tablet corpus beyond the classical five.
They are valid `Governing Element` values.

| Element | Domain | Notes |
|---|---|---|
| Light | Photonic / Radiant | Used in tablets concerning vision, illumination, and the colour spine |
| Shadow | Depth psychology | Sol Niger, integration of the dark; paired with Light |
| Sound | Acoustic / Vibrational | Resonance, harmony, the music domain; see RR-002 |
| Wood | Growth / Organic life | East Asian Five Phase (五行) correspondence: expansion, spring |
| Metal | Refinement / Structure | East Asian Five Phase: contraction, autumn; distinct from Earth |
| Void | Śūnyatā / Quantum vacuum | Non-being as generative; used in tablets concerning emptiness and potential |
| Cosmos | Planetary / Universal | Tablets that operate at the noospheric / L7 scale |

## Compound Elements

Some tablets declare a compound governing element when the work intentionally
bridges two domains. Compound elements are written as `Element × Element`
(e.g. `Fire × Air`, `Earth × Water`). No more than two elements may be
compounded in a single tablet field.

## Validation Rules

- The `Governing Element` field MUST contain exactly one value from the
  tables above, or one valid compound of two values.
- Values are case-sensitive as written here.
- `Aether` and `Spirit` are synonyms; both are valid; `Spirit` is preferred
  in tablets with a devotional framing, `Aether` in tablets with a
  cosmological framing.
- A tablet may list `None` if the work is deliberately element-agnostic
  (rare; requires justification in the tablet body).

## Refusing Invalid Values

The following are **not** valid `Governing Element` values and must be
rejected by the CI validator (#834 / #835):
- Numbered codes (e.g. `Element-4`)
- Chakra names used as elements (chakra belongs in `chakra_id`, not here)
- GFI dimension names (those are scale-layer labels, not elements)
- Free-form prose

**References:** #808 · #828 · #833 · #834 · Epic #798
