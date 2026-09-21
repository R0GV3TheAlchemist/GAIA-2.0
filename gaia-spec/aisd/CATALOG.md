# AISD Phase 1 — Skill Node Catalog

**Status:** Listed  
**Issue:** #123  
**Crate:** `gaia-aisd` (Apache-2.0)  

All nodes: `gaia_enabled = false`. No `score_live`. No gap closure.
39 skill node stubs across 13 realms (3 per realm) + 8 gap status nodes.

---

## `language`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:language:nlu` | L4 | MMLU 86%+ | Hendrycks et al. 2021 |
| `aisd:language:generation` | L4 | MT-Bench, AlpacaEval | Zheng et al. 2023 |
| `aisd:language:translation` | L3 | WMT newstest BLEU | Barrault et al. 2020 |

---

## `reasoning`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:reasoning:math` | L3 | MATH 50–90% (model-dependent) | Hendrycks et al. 2021 |
| `aisd:reasoning:logical` | L3 | BIG-Bench Hard, LogiQA | Suzgun et al. 2022 |
| `aisd:reasoning:analogical` | L3 | ARC-Challenge | Clark et al. 2018 |

**Gap node:**

| id | Gap status | Maturity | Source |
|---|---|---|---|
| `aisd:gap:long-horizon-cot` | `Open` | L2 | Bubeck et al. 2023 — *Sparks of AGI* |
| `aisd:gap:causal` | `Open` | L2 | Scholkopf et al. 2021 — *Toward Causal Representation Learning* |

---

## `coding`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:coding:generation` | L3 | HumanEval 85%+ | Chen et al. 2021 |
| `aisd:coding:debugging` | L3 | SWE-bench partial | Jimenez et al. 2024 |
| `aisd:coding:review` | L3 | CodeReviewer | Li et al. 2022 |

**Gap node:**

| id | Gap status | Maturity | Source |
|---|---|---|---|
| `aisd:gap:swe-pro-engineering` | `Open` | L2 | Jimenez et al. 2024 — *SWE-bench* |

---

## `vision`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:vision:vqa` | L3 | VQAv2 80%+ | Goyal et al. 2017 |
| `aisd:vision:captioning` | L3 | COCO CIDEr | Lin et al. 2014 |
| `aisd:vision:ocr` | L3 | TextVQA | Singh et al. 2019 |

---

## `audio`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:audio:asr` | L3 | LibriSpeech WER 2.7% | Panayotov et al. 2015 |
| `aisd:audio:tts` | L3 | MOS 4.0+ (human parity) | Wang et al. 2017 |
| `aisd:audio:music-understanding` | L2 | MusicCaps partial | Agostinelli et al. 2023 |

---

## `multimodal`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:multimodal:vision-language` | L3 | MMBench, MMMU | Liu et al. 2023 |
| `aisd:multimodal:audio-vision` | L2 | AVSpeech partial | Ephrat et al. 2018 |
| `aisd:multimodal:cross-modal-gen` | L2 | DALL-E 3 / Sora evals | Betker et al. 2023 |

---

## `memory`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:memory:long-context` | L2 | SCROLLS, LongBench partial | Shaham et al. 2022 |
| `aisd:memory:episodic-recall` | L2 | MemGPT eval partial | Packer et al. 2023 |
| `aisd:memory:working-memory` | L2 | BabiLong partial | Kuratov et al. 2024 |

**Gap node:**

| id | Gap status | Maturity | Source |
|---|---|---|---|
| `aisd:gap:continual-learning` | `Open` | L1 | Kirkpatrick et al. 2017 — *Overcoming Catastrophic Forgetting* |

---

## `planning`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:planning:goal-decomposition` | L2 | GAIA partial | Mialon et al. 2023 |
| `aisd:planning:task-sequencing` | L2 | AgentBench partial | Liu et al. 2023 |
| `aisd:planning:agentic` | L2 | SWE-agent partial | Yang et al. 2024 |

---

## `tool-use`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:tool-use:api-call` | L3 | ToolBench | Qin et al. 2023 |
| `aisd:tool-use:computer-use` | L3 | OSWorld, ScreenSpot | Xie et al. 2024 |
| `aisd:tool-use:sandboxed-exec` | L3 | GAIA level 1–2 | Mialon et al. 2023 |

---

## `science`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:science:protein-structure` | L3 | AlphaFold CASP14 GDT 92.4 | Jumper et al. 2021 |
| `aisd:science:scientific-qa` | L3 | SciQ, SciFact | Welbl et al. 2017 |
| `aisd:science:hypothesis-gen` | L2 | FrontierMath partial | Glazer et al. 2024 |

---

## `social`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:social:social-qa` | L2 | SocialIQa | Sap et al. 2019 |
| `aisd:social:empathy-modelling` | L2 | EmpatheticDialogues | Rashkin et al. 2019 |
| `aisd:social:negotiation` | L2 | DealOrNoDeal partial | Lewis et al. 2017 |

**Gap node:**

| id | Gap status | Maturity | Source |
|---|---|---|---|
| `aisd:gap:theory-of-mind` | `Open` | L2 | Ullman 2023 — *Large Language Models Fail on Trivial Alterations to Theory-of-Mind Tasks* |

---

## `embodiment`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:embodiment:sim-navigation` | L2 | Habitat, AI2-THOR | Savva et al. 2019 |
| `aisd:embodiment:manipulation` | L1 | RLBench partial | James et al. 2020 |
| `aisd:embodiment:sim-to-real` | L1 | RT-2 partial | Brohan et al. 2023 |

**Gap node:**

| id | Gap status | Maturity | Source |
|---|---|---|---|
| `aisd:gap:dexterous-embodiment` | `Open` | L1 | Dexterity survey — Chen et al. 2023 |

---

## `meta`

| id | Maturity | Benchmark anchor | Source |
|---|---|---|---|
| `aisd:meta:self-eval` | L2 | LLM-as-judge partial | Zheng et al. 2023 |
| `aisd:meta:uncertainty-est` | L2 | CalibrationBench partial | Kadavath et al. 2022 |
| `aisd:meta:self-correction` | L2 | Self-Refine partial | Madaan et al. 2023 |

**Gap nodes:**

| id | Gap status | Maturity | Source |
|---|---|---|---|
| `aisd:gap:calibration` | `Open` | L2 | Kadavath et al. 2022 — *Language Models (Mostly) Know What They Know* |
| `aisd:gap:genuine-novelty` | `Open` | L1 | Mitchell 2021 — *Why AI is Harder Than We Think* |

---

## Totals

| Count | Value |
|---|---|
| Realms covered | 13 / 13 |
| Skill node stubs | 39 |
| Gap status nodes | 8 |
| Open gaps | 8 / 8 |
| Closed gaps | 0 |
| `gaia_enabled = false` | All nodes |
