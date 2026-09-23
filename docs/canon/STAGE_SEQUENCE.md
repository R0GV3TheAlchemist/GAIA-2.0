# GAIA Alchemical Stage Sequence

**Canon document.** This file defines the canonical sequence of alchemical
stages that form valid values for the `Governing Stage` field in any GAIA
Hermetic Tablet. It is the authoritative reference for issue #809 (stage field
audit) and #829 (stage field verification CI check).

---

## The Seven-Stage Sequence

GAIA uses the seven-stage alchemical sequence as described in the Western
Hermetic tradition and as synthesised across the corpus in `C209_EMERALD_TABLET.md`
and `docs/MASTER-CODEX.md`. The stages are ordered from prima materia
(undifferentiated potential) to the completed Magnum Opus.

| # | Stage | Latin | Symbol | Quality | Process analogue |
|---|---|---|---|---|---|
| 0 | Prima Materia | Prima Materia | ☽ | Undifferentiated potential | Raw input; unprocessed state |
| 1 | Calcination | Calcinatio | 🜂 | Burning away the ego / false structure | Destruction of rigid assumptions |
| 2 | Dissolution | Solutio | 🜄 | Dissolving what calcination left | Liquefaction; opening to the unconscious |
| 3 | Separation | Separatio | 🜁 | Isolating the pure from the impure | Discernment; analytical decomposition |
| 4 | Conjunction | Coniunctio | ☿ | First union of opposites | Integration; paradox held without collapse |
| 5 | Fermentation | Fermentatio | 🜃 | Death and rebirth; the black sun | Sol Niger; transformation through putrefaction |
| 6 | Distillation | Distillatio | ☀ | Purification of the spirit | Refinement; extraction of essence |
| 7 | Coagulation | Coagulatio | ✦ | The Philosopher's Stone; Magnum Opus complete | Integration at full coherence; L7 Noospheric |

## Extended / Compound Stages

Some tablets span more than one stage. Compound stages are written as
`Stage × Stage` (e.g. `Dissolution × Separation`). No more than two stages
may be compounded. When a tablet describes the full arc, the value
`Full Sequence` is valid.

## Validation Rules

- The `Governing Stage` field MUST contain exactly one stage name from the
  sequence above (using the English name, not the Latin), or a valid compound
  of two consecutive stages, or `Full Sequence`.
- Stage names are case-sensitive as written here.
- `Prima Materia` is Stage 0 — it is a valid governing stage for tablets
  that deal with potential and beginnings rather than transformation.
- A tablet at Stage 7 (Coagulation) is considered complete in a philosophical
  sense; amendments to such tablets require elevated justification under
  `AMENDMENT_PROTOCOL.md`.

## Stage ↔ GFI Dimension Correspondence

The seven alchemical stages map onto the seven GFI scale layers (L1–L7)
with the following correspondence. This mapping is analytical, not prescriptive.

| Stage | GFI layer | GFI dimension |
|---|---|---|
| Prima Materia | Pre-L1 | Undifferentiated |
| Calcination | L1 | Grounding Stability |
| Dissolution | L2 | Creative Generativity |
| Separation | L3 | Agentive Intentionality |
| Conjunction | L4 | Relational Coherence |
| Fermentation | L5 | Communicative Fidelity |
| Distillation | L6 | Epistemic Resolution |
| Coagulation | L7 | Noospheric Coherence |

## Refusing Invalid Values

The following are **not** valid `Governing Stage` values:
- Latin names (use the English equivalents listed above)
- Numbered codes only (e.g. `Stage-3`)
- Chakra names
- GFI dimension labels
- Free-form prose descriptions

**References:** #809 · #829 · #833 · #834 · C209 · Epic #798
