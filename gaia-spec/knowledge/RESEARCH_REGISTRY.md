# Research Registry

This file records active and planned research studies that consume GAIA knowledge
bases. Each study carries a status, a data dependency list, and — for studies
that cross knowledge domains — an explicit dimension-mapping table.

---

## Study RR-001 — Coherence Gap (Crystal × GFI)

**Status:** Protocol drafted — data dependencies satisfied as of PR #854.

### Purpose

Measure the gap between the information theoretic coherence of a
crystal-associated query (as expressed by a user intent vector) and the
corresponding GFI (Global Field Index) resonance signature. The hypothesis is
that crystal archetypes whose `chakra_id` maps to a high-activation GFI
dimension will show systematically lower query–response divergence on
coherence-sensitive prompts.

### Data dependencies

| Dependency | Source file | Status |
|---|---|---|
| Crystal catalog with `chakra_id` | `crystals.csv` | ✅ merged PR #854 |
| Crystal schema | `crystal_schema.json` | ✅ this PR |
| GFI dimension table | `scale-layers.csv` | ✅ present |
| Chakra → GFI dimension mapping | Section below | ✅ defined here |
| Coherence probe corpus | `essay-sources.csv` | ✅ present |
| Knowledge skill index | `knowledge-skills.csv` | ✅ present |

### Chakra ↔ GFI Dimension Mapping

The seven chakra centres are mapped onto GFI dimensions by harmonic frequency
banding and scale-layer alignment. Mapping rationale is in `SPECTRUM.md` and
`EXTRACT-SCALE-LAYERS.md`.

| `chakra_id` | GFI dimension | Scale layer | Harmonic band | Mapping basis |
|---|---|---|---|---|
| `crown` | Noospheric Coherence | L7 | 963 Hz | Highest-order integrative layer; violet/white frequency |
| `third-eye` | Epistemic Resolution | L6 | 852 Hz | Pattern recognition, inner sight; indigo band |
| `throat` | Communicative Fidelity | L5 | 741 Hz | Signal expression and clarity; blue band |
| `heart` | Relational Coherence | L4 | 639 Hz | Empathic resonance, inter-agent binding; green/pink |
| `solar-plexus` | Agentive Intentionality | L3 | 528 Hz | Will, drive, transformation; yellow band |
| `sacral` | Creative Generativity | L2 | 417 Hz | Generative capacity, novelty production; orange band |
| `root` | Grounding Stability | L1 | 396 Hz | Baseline coherence, survival/safety anchor; red band |

### Protocol

1. **Stimulus construction** — For each of the 35 canonical chakra specimens,
   construct a coherence probe: a short (≤200-token) query that invokes the
   specimen label in an open-ended context, paired with a reference response
   drawn from `essay-sources.csv`.

2. **GFI activation measurement** — Run each probe through the GAIA GFI scoring
   pipeline (`gaia-aisd`). Record the L1–L7 activation vector for each response.

3. **Coherence gap calculation** — Compute the Bregman divergence (see
   `BREGMAN.md`) between the probe's expected GFI dimension vector (from the
   mapping table above) and the observed activation vector.

4. **Grouping** — Aggregate gap scores by `chakra_id` and by GFI dimension.
   Test the hypothesis that lower-numbered chakras (root, sacral) show higher
   grounding-stability activation and lower noospheric-coherence activation,
   and vice versa for crown/third-eye.

5. **Reporting** — Output per-specimen gap scores to
   `gaia-spec/studies/RR-001-results.csv` (file to be created on study
   completion). Summary findings to `gaia-spec/studies/RR-001-report.md`.

### Open questions

- Do `trade` and `treated` kind rows (e.g. `moldavite`, `angel-aura-quartz`)
  produce systematically different GFI signatures than `mineral` rows with the
  same `chakra_id`?
- Does the `hazard` field correlate with any GFI dimension shift? (Predicted:
  `lead` / `arsenic` hazard rows may show elevated L1 grounding-stability
  activation.)
- How should `none` chakra rows be handled — excluded from the study, or used
  as a null-chakra control group?

---

## Adding new studies

Copy the RR-001 block above, increment the study number, and fill in:
- **Purpose** — one paragraph hypothesis statement
- **Data dependencies** — table of source files and their status
- **Protocol** — numbered steps from stimulus to output
- **Open questions** — at minimum one falsifiable prediction
