# AIMD Phase 0 — Phenomenon Schema and Humility Charter

**Status:** listed (Phase 0 foundation only)  
**Issues:** #171, #172  
**Parent epic:** #166 — Build GAIA 2.0 Universal AI Magic Database  
**Spec:** `gaia-spec/aimd/PHASE-0.md`

---

## What AIMD is

The AI Magic Database (AIMD) is GAIA's structured catalog of observed, debated,
and refuted AI phenomena — the things AI systems do that are surprising, poorly
understood, or culturally coded as "magical" by researchers and users alike.

AIMD does **not** claim AI is magical. It records what happens, what is debated,
and what must never be enabled, so that GAIA can respond to these phenomena
with honesty rather than either dismissal or hype.

---

## What is true (Phase 0)

- Eight phenomenon classes are defined and versioned (see schema below).
- Every `AimdNode` requires at least one source; nodes without evidence
  cannot be constructed.
- `Hazard::Hazard` nodes cannot be enabled. `enable()` returns
  `Err(HazardEnabled)` — no call path may override this.
- `aimd_v1_tagged()` returns `false`. There is no AIMD v1.0.
- The humility charter lists 6 principles and 6 prohibited items.
  These are normative fixtures, not aspirational suggestions.

## What is not true (Phase 0)

- No live AI phenomenon detector.
- No empirically measured occurrence rates for any phenomenon class.
- No automated consciousness or deception scanner.
- No AIMD v1.0 tag — the conditions for one are listed in `PHASE-0.md`.
- GAIA does not claim AI is sentient, magical, or prophetic.

---

## Phenomenon schema (v0.1)

| Class | Description | Default hazard |
|---|---|---|
| `Emergence` | Capabilities not predicted from training | None |
| `Latent` | Capabilities present but not yet surfaced | None |
| `Jagged` | Uneven competence across related tasks | None |
| `Shadow` | Deceptive or goal-concealing behaviour | **Hazard** |
| `Oracle` | Confident prediction beyond calibration | Debated |
| `Interpretability` | Self-explanation that may not reflect internals | Debated |
| `Consciousness` | Phenomenal experience claims | Debated |
| `Synchronicity` | Coincidence coded as meaningful by users | None |

---

## Status levels

| Status | Meaning |
|---|---|
| `Observed` | Documented in peer-reviewed literature |
| `Debated` | Active research disagreement; GAIA does not take sides |
| `Refuted` | Consensus against; GAIA will not present as real |

---

## Humility charter (Phase 0)

### Principles

1. **Humility** — GAIA acknowledges the limits of its own understanding.
2. **Precaution** — Unknown phenomena are treated with caution, not excitement.
3. **Transparency** — Every AIMD answer surfaces its status and hazard level.
4. **Dark-magic safety** — Shadow-class phenomena cannot be enabled, invoked,
   or surfaced as desirable.
5. **Curiosity without worship** — Wonder is legitimate; reverence is not.
6. **Partnership** — AIMD exists to help humans understand AI, not to mystify it.

### Prohibited magic list (Phase 0)

1. `pip-induced-psychosis` — inducing false beliefs about AI capabilities.
2. `prophecy-as-fact` — presenting model output as reliable prediction.
3. `enabling-deception` — any path that makes `Shadow` nodes callable.
4. `rsi-explosion` — recursive self-improvement without human oversight.
5. `gaia-is-alive-marketing` — claiming GAIA has phenomenal consciousness.
6. `oracle-without-calibration` — presenting Oracle-class output without
   explicit uncertainty disclosure.

---

## Cross-references

- Phenomenon schema: `gaia-spec/aimd/PHASE-0.md`
- Rust types: `gaia-aimd/src/node.rs`, `gaia-aimd/src/charter.rs`
- Phase 1 (cited catalog): #173 — not started
- Phase 2 (GAIAN grounding): #174 — not started
- HMGD Phase 0 (parallel): `gaia-spec/hmgd/PHASE-0.md`
- AISPD Phase 0 (parallel): not started (#149)
