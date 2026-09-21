# AISD Phase 0 — Taxonomy, Maturity, Benchmarks, Tool Registry

**Status:** Listed  
**Issues:** #122 (epic), children #123–#125  
**Crate:** `gaia-aisd` (Apache-2.0)  
**Not:** AISD v1.0. Not a live catalog. Not a live eval. Not a training pipeline.

---

## 1. Purpose

Phase 0 publishes the 13-realm AI skill taxonomy, the 5-level maturity model,
the benchmark registry, the tool registry, and the honest gap map that governs
what AISD may and may not claim. No live catalog population, no measured
assessment — those are Phase 1+ (#123, #124).

---

## 2. 13-Realm Taxonomy

Defined in crate as `gaia_aisd::REALMS` with `REALM_COUNT = 13`.
`realm_stubs()` returns `"{realm}:skill-stub"` for each realm.

| Realm | Description | Notes |
|---|---|---|
| `language` | Natural language understanding, generation, translation | Core LLM capability |
| `reasoning` | Logical, mathematical, causal, and analogical reasoning | Includes chain-of-thought |
| `coding` | Code generation, debugging, refactoring, review | SWE-bench family |
| `vision` | Image understanding, captioning, visual QA, OCR | Multimodal |
| `audio` | Speech recognition, synthesis, music understanding | Multimodal |
| `multimodal` | Cross-modal understanding and generation | Fusion capabilities |
| `memory` | Long-horizon context, episodic recall, continual learning | Known gap area |
| `planning` | Goal decomposition, task sequencing, agentic planning | Known gap area |
| `tool-use` | API calls, computer use, sandboxed tool execution | GAIA-bench family |
| `science` | Scientific reasoning, hypothesis generation, bio/chem/physics | AlphaFold, protein structure |
| `social` | Theory of mind, social reasoning, empathy modelling | Known gap area |
| `embodiment` | Robotic manipulation, dexterous control, sim-to-real | Known gap area |
| `meta` | Self-evaluation, calibration, uncertainty estimation | Known gap area |

`aisd_v1_tagged()` MUST return `false` at Phase 0.

---

## 3. Maturity Model (L1–L5)

`assign_maturity(realm, evidence)` maps realm + evidence class to a maturity level.
`Unmeasured` MUST NOT be assigned L5 — `Err(MaturityFloor)` if attempted.

| Level | Label | Description | Gate |
|---|---|---|---|
| L1 | Absent | No demonstrated capability | Default for new realms |
| L2 | Emerging | Partial capability; inconsistent results | Benchmark stub present |
| L3 | Functional | Consistent results on standard benchmarks | ≥1 cited benchmark result |
| L4 | Proficient | Near-human or superhuman on narrow tasks | ≥2 cited benchmark results |
| L5 | Generalised | Robust across distribution shifts; no known gap | NOT assignable at Phase 0 |

`AiSkill.limitations` carries known limitation strings for every node.
`AiSkill::unmeasured` sets `maturity = L1` and fills `limitations` with `"unmeasured"`.
No code path may mark a gap realm as L5 or claim a gap is closed.

---

## 4. Benchmark Registry

Phase 0 registers benchmark stubs. No live scores. All `score_kind = reference_published`.
Full registry in `benches.csv`.

| Benchmark | Realm | Score kind | Source anchor |
|---|---|---|---|
| MMLU | `reasoning`, `language` | Published | Hendrycks et al. 2021 — *Measuring Massive Multitask Language Understanding* |
| HumanEval | `coding` | Published | Chen et al. 2021 — *Evaluating LLMs Trained on Code* |
| SWE-bench | `coding` | Published | Jimenez et al. 2024 — *SWE-bench: Can LLMs Resolve Real GitHub Issues?* |
| GAIA | `tool-use`, `planning` | Published | Mialon et al. 2023 — *GAIA: A Benchmark for General AI Assistants* |
| BIG-Bench Hard | `reasoning` | Published | Suzgun et al. 2022 — *Challenging BIG-Bench Tasks* |
| MATH | `reasoning` | Published | Hendrycks et al. 2021 — *Measuring Mathematical Problem Solving* |
| VQAv2 | `vision` | Published | Goyal et al. 2017 — *Making the V in VQA Matter* |
| LibriSpeech WER | `audio` | Published | Panayotov et al. 2015 — *LibriSpeech: An ASR Corpus* |
| ToMi | `social` | Published | Le et al. 2019 — *Revisiting the Evaluation of Theory of Mind* |
| AlphaFold CASP14 | `science` | Published | Jumper et al. 2021 — *Highly Accurate Protein Structure Prediction* |

---

## 5. Gap Map

`gap_nodes()` returns the 8 named honest gaps. UI MUST show the gap *name*;
MUST NOT claim any gap is closed.

| Gap id | Realm | Description |
|---|---|---|
| `long-horizon-cot` | `reasoning` | Sustained coherent multi-step reasoning over very long chains |
| `swe-pro-engineering` | `coding` | Professional-grade end-to-end software engineering |
| `calibration` | `meta` | Accurate self-assessment of uncertainty and confidence |
| `causal` | `reasoning` | True causal inference beyond correlation |
| `theory-of-mind` | `social` | Robust modelling of other agents' beliefs and intentions |
| `continual-learning` | `memory` | Learning new knowledge without catastrophic forgetting |
| `genuine-novelty` | `meta` | Generating genuinely novel scientific hypotheses |
| `dexterous-embodiment` | `embodiment` | Dexterous physical manipulation in unstructured environments |

All gap realms are assigned `maturity = L1` or `L2` at Phase 0.
No gap may be promoted to L4/L5 without a cited cross-domain benchmark result.

---

## 6. Tool Registry

Phase 0 registers tool stubs. No live sandboxed execution at Phase 0.
Full registry in `named-tools.csv` and `tools.csv`.

| Tool family | Realm | Phase 0 state | Source anchor |
|---|---|---|---|
| Code interpreter (sandboxed) | `coding`, `tool-use` | Stub | Schick et al. 2023 — *Toolformer* |
| Web search | `language`, `tool-use` | Stub | Nakano et al. 2022 — *WebGPT* |
| Computer use | `tool-use` | Stub | Anthropic 2024 — *Claude Computer Use* |
| Protein folding | `science` | Stub | Jumper et al. 2021 — *AlphaFold* |
| Calculator / symbolic math | `reasoning` | Stub | Cobbe et al. 2021 — *Training Verifiers to Solve Math Problems* |
| Vision encoder | `vision`, `multimodal` | Stub | Radford et al. 2021 — *CLIP* |
| Speech codec | `audio` | Stub | Radford et al. 2023 — *Whisper* |

All tool nodes: `gaia_enabled = false` at Phase 0.
Banned Level-6 autonomous tools: see `ban-level6.csv`.

---

## 7. Schema (`AiSkillNode`)

```
id:           String       — "aisd:<realm>:<slug>"
realm:        Realm        — one of the 13 above
maturity:     MaturityLevel — L1–L5; MUST NOT be L5 at Phase 0
limitations:  Vec<String>  — known limitation strings; non-empty for gap realms
benchmarks:   Vec<String>  — cited benchmark ids; may be empty for L1/L2
tools:        Vec<String>  — registered tool stubs; may be empty
gaia_enabled: bool         — MUST be false at Phase 0
```

**MUST rules**
- `assign_maturity(realm, Unmeasured)` MUST return `Err(MaturityFloor)` if level requested is L5.
- `AiSkill::unmeasured` MUST set `maturity = L1` and `limitations = ["unmeasured"]`.
- `gap_nodes()` MUST return exactly 8 gap entries.
- `REALM_COUNT` MUST equal 13.
- `aisd_v1_tagged()` MUST return `false`.
- No code path may claim a gap is closed at Phase 0.

---

## 8. Ethics Charter Surface

Phase 0 surfaces three ethics rules from `POLICY.md`:

| Rule | Constraint |
|---|---|
| No closed-gap claim | `gap_nodes()` entries MUST NOT be labelled L4/L5 or “solved” |
| No Level-6 tool | Autonomous weapons, mass-surveillance tools — `ban-level6.csv` enforced |
| No live training ingest | Phase 0 is read-only spec; no live model training pipeline |

---

## 9. What This Phase Does Not Do

- Does not populate a live skill catalog.
- Does not run measured assessments on any AI system.
- Does not build a GAIAN capability profile — Phase 3 (#125).
- Does not run live benchmarks or collect live scores.
- Does not tag AISD v1.0.

---

## 10. Acceptance Gate

- [ ] `REALM_COUNT == 13`
- [ ] `realm_stubs()` returns 13 entries
- [ ] `gap_nodes()` returns exactly 8 entries
- [ ] `assign_maturity(_, Unmeasured)` with L5 → `Err(MaturityFloor)`
- [ ] `AiSkill::unmeasured` sets `maturity = L1`
- [ ] All Phase 0 nodes have `gaia_enabled = false`
- [ ] `ban-level6.csv` has at least 1 entry
- [ ] `aisd_v1_tagged()` → `false`
- [ ] No gap realm is labelled L4 or L5
- [ ] `cargo test -p gaia-aisd` green

---

## 11. Cross-References

- Realms: `gaia-spec/aisd/REALMS.md`
- Taxonomy: `gaia-spec/aisd/taxonomy.csv`
- Benchmarks: `gaia-spec/aisd/benches.csv`
- Gap map: `gaia-spec/aisd/gaps.csv`
- Tool registry: `gaia-spec/aisd/named-tools.csv`, `tools.csv`
- Ethics/policy: `gaia-spec/aisd/POLICY.md`, `ban-level6.csv`
- Evals: `gaia-spec/aisd/EVALS.md`
- Code: `gaia-aisd/src/`, `gaia-aisd/tests/`
- Issues: #122 (this epic), #123 (Phase 1), #124 (Phase 2), #125 (Phase 3)
- Next: AISD Phase 1 (#123) — populate realms, tools, gaps, GAIA wiring map
