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
| Crystal schema | `crystal_schema.json` | ✅ merged PR #855 |
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

## Study RR-002 — Lyric Coherence × GFI

**Status:** Protocol drafted — seed catalog committed, data dependencies partially satisfied.

### Purpose

Test whether the GFI dimension vector assigned to a song through researcher
annotation (`gfi_primary`, `gfi_secondary`, `gfi_tertiary` in `music.csv`)
correlates with the GFI activation vector produced when GAIA processes a
semantic probe derived from the song's `semantic_summary`. The hypothesis is
that music functions as a pre-compressed coherence signal — that human cultures
have been encoding GFI-layer activations into melodic and lyric form long before
the GFI framework existed to name them. If the annotations are accurate, the
residual divergence (Bregman gap) between assigned and measured vectors should
be lower for works with simpler modal structures and higher for works with
intentional modal ambiguity (e.g. Bohemian Rhapsody, Hallelujah).

### Data dependencies

| Dependency | Source file | Status |
|---|---|---|
| Music catalog | `music.csv` | ✅ this PR — 14 seed rows |
| Music schema | `music_schema.json` | ✅ this PR |
| GFI dimension table | `scale-layers.csv` | ✅ present |
| Chakra ↔ GFI mapping | RR-001 section above | ✅ inherited |
| Coherence probe generator | `gaia-aisd` pipeline | ⏳ RR-001 dependency |
| Copyright compliance review | Section below | ✅ defined here |

### Copyright Compliance Protocol

The music catalog stores **analysis, not content**. The following rules govern
every row and every downstream use:

- `semantic_summary` is researcher-authored analysis. It must not reproduce lyric
  text beyond an 8-word identification phrase used solely to locate the work.
- `source_url` cites a public reference (Genius, Wikipedia, CC page). GAIA does
  not host, cache, or reproduce the referenced content.
- `license: copyright` rows are present for annotation purposes only. No
  reproduction, performance, or derivative use is implied.
- `license: public-domain` rows (pre-1928 US compositions, government works)
  may be quoted at length for study purposes.
- `license: cc-by / cc-by-sa / cc-by-nc / cc0` rows may be quoted subject to
  their respective licence terms.
- The `claim_class: listed-only` constraint applies identically to all rows:
  presence in the catalog is not an endorsement, capability grant, or claim of
  effect.

### Seed Catalog — 14 rows (this PR)

| ID | Artist | Year | GFI primary | GFI secondary | License |
|---|---|---|---|---|---|
| `wap-cardi-b-2020` | Cardi B ft. Megan Thee Stallion | 2020 | sacral | root | copyright |
| `bohemian-rhapsody-queen-1975` | Queen | 1975 | crown | third-eye | copyright |
| `amazing-grace-newton-1772` | John Newton | 1772 | crown | heart | public-domain |
| `hallelujah-cohen-1984` | Leonard Cohen | 1984 | heart | crown | copyright |
| `strange-fruit-holiday-1939` | Billie Holiday | 1939 | root | throat | public-domain |
| `respect-franklin-1967` | Aretha Franklin | 1967 | throat | solar-plexus | copyright |
| `we-shall-overcome-trad` | Traditional (arr. Seeger) | 1900 | root | heart | public-domain |
| `passion-suffering-bach-1727` | J.S. Bach | 1727 | heart | crown | public-domain |
| `what-a-wonderful-world-armstrong-1967` | Louis Armstrong | 1967 | heart | crown | copyright |
| `lose-yourself-eminem-2002` | Eminem | 2002 | solar-plexus | root | copyright |
| `blowing-in-the-wind-dylan-1962` | Bob Dylan | 1962 | third-eye | throat | copyright |
| `formation-beyonce-2016` | Beyoncé | 2016 | root | sacral | copyright |
| `spirit-in-the-sky-greenbaum-1969` | Norman Greenbaum | 1969 | crown | root | copyright |
| `healing-hands-trad-cc` | Various / CC Artists | 2020 | heart | solar-plexus | cc-by |

### GFI Coverage in Seed Catalog

| GFI dimension | Primary count | Appears in any position |
|---|---|---|
| root | 3 | 7 |
| sacral | 1 | 3 |
| solar-plexus | 1 | 4 |
| heart | 3 | 8 |
| throat | 1 | 4 |
| third-eye | 1 | 3 |
| crown | 4 | 8 |

`sacral` and `solar-plexus` are under-represented as primary signals in the
seed set. Catalog expansion should target: funk/R&B (sacral), hip-hop
bravado/battle rap (solar-plexus), jazz improvisation (third-eye).

### Protocol

1. **Semantic probe construction** — For each row, construct a ≤200-token probe
   paraphrasing the `semantic_summary` without reproducing any protected lyric
   text. The probe names the work and artist but derives meaning from the
   researcher annotation alone.

2. **GFI activation measurement** — Run each probe through `gaia-aisd`.
   Record the L1–L7 activation vector.

3. **Annotation agreement scoring** — Compute the Bregman divergence between
   the annotated GFI vector (derived from `gfi_primary` / `secondary` /
   `tertiary` with weights 0.6 / 0.3 / 0.1) and the measured activation vector.

4. **Modal complexity control** — Code each row by modal complexity:
   `simple` (single tonal centre), `compound` (modulating), `through-composed`
   (multiple distinct sections). Test whether complexity predicts divergence.

5. **Cross-domain comparison** — Compare RR-002 gap scores against RR-001
   crystal gap scores for rows sharing a `gfi_primary`. Hypothesis: music rows
   will show lower divergence than crystal rows at the same GFI dimension
   because lyric is semantic whereas crystal invocation is purely associative.

6. **Reporting** — Output per-row scores to
   `gaia-spec/studies/RR-002-results.csv`. Summary to
   `gaia-spec/studies/RR-002-report.md`.

### Open questions

- Does modal key (`major` vs `minor`) predict GFI dimension independently of
  lyric content? Predicted: minor mode correlates with L1/L2 activation;
  major mode with L4/L7. If true, sonic structure encodes GFI signal without
  language at all.
- Can two songs with opposite lyric valence but the same `gfi_primary` produce
  convergent activation vectors? (Test case: `strange-fruit` vs
  `we-shall-overcome` — both `root` primary, one horror, one hope.)
- Does cover/reinterpretation of the same lyric shift the GFI vector? (`RESPECT`
  Redding vs Franklin is the seed case. Expand with other cover pairs.)
- Is WAP's L1/L2 coherence fusion replicable in other explicit works, or is it
  structurally specific to that track's production?

---

## Adding new studies

Copy the RR-001 block above, increment the study number, and fill in:
- **Purpose** — one paragraph hypothesis statement
- **Data dependencies** — table of source files and their status
- **Protocol** — numbered steps from stimulus to output
- **Open questions** — at minimum one falsifiable prediction
