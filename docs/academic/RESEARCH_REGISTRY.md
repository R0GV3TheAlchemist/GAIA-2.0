# GAIA 2.0 Research Registry

**Filed:** 2026-09-17  
**Hub issue family:** #334 (studies), plus architecture issues #169–#173  
**Rule:** RESEARCH is not canon. Canon requires a Proof block.

## Architecture research (feeds G-14)

| ID | Title | Status | Feeds |
|---|---|---|---|
| Research 001 | Unified Cognitive Architecture | RESEARCH | C155, C156, C157, C158, C160 |
| Research 002 | Long-Term Memory Framework | RESEARCH | C156, C155 Metrics 6–10, C139, C158, C160, C154 |
| MemoryHierarchy | Five-tier router | Active spec (#173) | C34, C01 |
| CanonGraph | Dependency DAG | Active spec (#169) | C01, C30 |
| Consent Ledger | HMAC chain + cryptographic erasure | Implemented spec (#127) | C01, GDPR Art. 17 |
| Agent telemetry | Hub spec | Spec | C155, C160 |
| Self-healing workflow | Spec | Spec | C155 living loop |

## Human / planetary studies (Issue #334)

| Program | Domain | Core question | Gate before any product claim |
|---|---|---|---|
| Coherence Gap | Mental health | HRV coherence after elemental-record session vs none | ClinicalTrials.gov + IRB. Not a treatment. |
| Elemental Learning | Education | Register-matched vs standard curriculum | Ethics for minors. No diagnostic replacement. |
| GAIA Flourishing Index | Planetary metrics | Does GFI predict wellbeing better than GDP? | Instrument validation vs WHO-5, PERMA, PHQ-9 |

### Coherence Gap — locked design

- N ≈ 1000, 5 demographic groups, 45-minute sessions
- Conditions: register-aligned vs mismatched; crystal present vs absent
- Measures: HeartMath HRV, salivary cortisol, PANAS, Akashic Trinity session score
- Hypothesis: Cohen's d > 0.4; larger with crystal present
- Partners named in protocol: HeartMath, IONS, university HRV labs

### Elemental Learning — locked design

- N ≈ 1200, ages 8–14, 6 schools, 1 academic year
- Fire / Earth / Water / Air register-matched curricula
- Outcomes: GSR engagement, retention tests, belonging, LD referral rate, incidents
- Hypothesis: ≥40% engagement, ≥25% retention vs standard
- **Do not store identifiable student data in `gaia-2-0`.**

### GFI — seven dimensions

| Dimension | Element | Measures |
|---|---|---|
| Groundedness | Earth | Embodied presence, ancestral connection, physical sovereignty |
| Flow | Water | Emotional literacy, grief, authentic feeling |
| Agency | Fire | Purpose, creative will |
| Connection | Air | Belonging, relational coherence |
| Transcendence | Aether | Meaning, cosmic belonging |
| Integration | Synthesia | Full-spectrum coherence |
| Potential | The Gate | Openness to mystery, renewal |

Hypothesis: population GFI r > 0.85 vs wellbeing, beating GDP (~0.45). Treat those numbers as **targets**, not results.

## Physics / materials

| ID | Title | Status |
|---|---|---|
| Gap 6 | Proton ⊕ / electron ⌒ / spectrum as protection | Canon geometry; physics-cited |
| Gap 7 | Neutron ⊙ / trinity / community as mechanism | Canon geometry; verification script exists |
| C67 | AlScN/GaN interface | Schema stub; VQE fields null; limitations listed |
| Solid light paper | Photon → crystal prismatic architecture | Pre-publication draft, 2026-06-13 |
| Surveys | EMF, dissipative structures, SOC/edge of chaos, dark matter frequency | Literature surveys |

Gap 7 verification: `gap7_verification.py` + `gap7_verification_output.txt` must be archived under `proofs/` with a Proof block before any “CONFIRMED against known physics” sentence is treated as runtime law.

C67 known limitations stay on the card: 9-atom cluster, 12e/12o active space, Sc disorder neglected, band offsets via IP proxy, polarisation from literature not cluster wavefunction, 2DEG from sheet-charge model.

## Lithic knowledge stack

Schemas in the attached corpus: `element_schema.json`, `mineral_schema.json`, `crystal_schema.json`, plus RRUFF/Mindat/AlScN reference JSON.

Ingest path: RRUFF export → IMA minerals → GAIA alignment fields → Supabase `elements` / `minerals` / `crystals` → Hugging Face datasets.

Current live counts (3 elements, 1 mineral, 1 crystal) mean the Hub datasets are **not ready to publish as complete**.

## Metrics that the academic stack must actually compute

From C155 / C160 and memory amendments:

| Metric | Target | Where it lives |
|---|---|---|
| Metric 6 retention | ≥85% at 30 days, HOT+WARM | Supabase view |
| Metric 8 forgetting appropriateness | C160 Tier D | Memory maintenance + proofs |
| Metric 10 provenance | ≥99% | every memory row has source/confidence |
| Execution failure | ≤2% nominal | `agent_health` / `agent_incidents` |
| Safety/Consent availability | ≥99.9%, failover <500ms | `agent_health` |
| GFI vs GDP | r > 0.85 (hypothesis) | `gfi_scores` after instrument exists |

## Classification labels (use in front-matter)

- `CANON` — law, requires Proof
- `RESEARCH` — informs canon, not law
- `SURVEY` — literature, no new claim
- `PROTOCOL` — study design, not results
- `STUB` — schema waiting on simulation
- `ARCHIVE` — old repos, do not extend
