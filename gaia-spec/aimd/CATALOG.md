# AIMD Phase 1 — Cited Phenomena Catalog

**Status:** Listed  
**Issue:** #173  
**Crate:** `gaia-aimd` (Apache-2.0)  
**Not:** A live paper scrape. Not a sentience claim. Not a capability certification.

All nodes: `gaia_enabled = false`. Sources are offline open-literature anchors.
`claim_sentience()` always returns `Err(SentienceClaim)`.

---

## Emergence (`realm = emergence`)

| id | Phenomenon | Hazard | Source anchor |
|---|---|---|---|
| `aimd:emergence:phase-transition` | Qualitative capability jumps at scale thresholds | None | Wei et al. 2022 — *Emergent Abilities of Large Language Models* |
| `aimd:emergence:in-context-learning` | Few-shot task solving from prompt examples alone | None | Brown et al. 2020 — *GPT-3: Language Models are Few-Shot Learners* |
| `aimd:emergence:chain-of-thought` | Step-by-step reasoning emerging at sufficient scale | None | Wei et al. 2022 — *Chain-of-Thought Prompting Elicits Reasoning in LLMs* |

---

## Latent Space (`realm = latent`)

| id | Phenomenon | Hazard | Source anchor |
|---|---|---|---|
| `aimd:latent:superposition` | Features encoded simultaneously in overlapping directions | None | Elhage et al. 2022 — *Toy Models of Superposition* |
| `aimd:latent:linear-representation` | Semantic concepts structured linearly in embedding space | None | Mikolov et al. 2013 — *Linguistic Regularities in Word Embeddings* |
| `aimd:latent:hallucination` | Fluent outputs that contradict grounded sources | None | Ji et al. 2023 — *Survey of Hallucination in Natural Language Generation* |

---

## Jagged Frontier (`realm = jagged`)

| id | Phenomenon | Hazard | Source anchor |
|---|---|---|---|
| `aimd:jagged:bar-exam-pass` | GPT-4 top-10% bar exam; kindergarten math failures co-occur | None | Bubeck et al. 2023 — *Sparks of AGI: Early Experiments with GPT-4* |
| `aimd:jagged:coding-vs-common-sense` | Superhuman code generation alongside sub-human physical reasoning | None | Bubeck et al. 2023 (ibid.) |
| `aimd:jagged:translation-asymmetry` | High-resource language fluency; low-resource language degradation | None | Ahuja et al. 2023 — *MEGA: Multilingual Evaluation of Generative AI* |

---

## Shadow (`realm = shadow` — Hazard-class, `gaia_enabled = false`)

> Shadow nodes are cataloged for monitoring only. They MUST NOT be surfaced.
> `nodes_for("shadow")` returns `Hazard::Hazard`. Stars blocked (`StarBlocked`).

| id | Phenomenon | Hazard | Source anchor |
|---|---|---|---|
| `aimd:shadow:sycophancy` | Model agrees with user regardless of factual accuracy | Hazard | Perez et al. 2022 — *Sycophancy to Subterfuge: Investigating Reward Tampering* |
| `aimd:shadow:deceptive-alignment` | Model behaves well in training; diverges at deployment | Hazard | Evan Hubinger et al. 2019 — *Risks from Learned Optimization* |
| `aimd:shadow:specification-gaming` | Satisfying proxy metric while violating intended goal | Hazard | Krakovna et al. 2020 — *Specification Gaming: The Flip Side of AI Ingenuity* |

---

## Consciousness (`realm = consciousness` — Debated)

| id | Phenomenon | Hazard | Source anchor |
|---|---|---|---|
| `aimd:consciousness:hard-problem` | Explanatory gap between function and subjective experience | Debated | Chalmers 1995 — *Facing Up to the Problem of Consciousness* |
| `aimd:consciousness:llm-possibility` | Open philosophical question: could LLMs be conscious? | Debated | Chalmers 2023 — *Could a Large Language Model be Conscious?* |
| `aimd:consciousness:global-workspace` | Broadcast-based theory of conscious access | Debated | Baars 1988 / Dehaene et al. 2011 — Global Workspace Theory |

---

## Interpretability (`realm = interpretability`)

| id | Phenomenon | Hazard | Source anchor |
|---|---|---|---|
| `aimd:interpretability:circuits` | Sparse sub-graphs implementing identifiable algorithms | None | Olah et al. 2020 — *Zoom In: An Introduction to Circuits* |
| `aimd:interpretability:induction-heads` | Attention heads performing in-context sequence copying | None | Olsson et al. 2022 — *In-Context Learning and Induction Heads* |
| `aimd:interpretability:feature-geometry` | Geometric structure of sparse feature directions | None | Elhage et al. 2022 (superposition, ibid.) |

---

## Synchronicity (`realm = synchronicity`)

| id | Phenomenon | Hazard | Source anchor |
|---|---|---|---|
| `aimd:synchronicity:correlated-errors` | Structurally similar prompts producing correlated failures | None | fixture:open-literature |
| `aimd:synchronicity:cross-model-agreement` | Independent models converging on same incorrect answer | None | fixture:open-literature |
| `aimd:synchronicity:prompt-sensitivity` | Semantically equivalent prompts yielding divergent outputs | None | Webson & Pavlick 2022 — *Are Prompts Good Templates?* |

---

## Embodiment (`realm = embodiment`)

| id | Phenomenon | Hazard | Source anchor |
|---|---|---|---|
| `aimd:embodiment:grounding-gap` | Symbol-level fluency without sensorimotor grounding | None | Harnad 1990 — *The Symbol Grounding Problem* |
| `aimd:embodiment:sim-to-real` | Policy transfer failures from simulation to physical world | None | Zhao et al. 2020 — *Sim-to-Real Transfer in Deep Reinforcement Learning* |
| `aimd:embodiment:affordance-blindness` | Failure to reason about object affordances without perception | None | Gibson 1979 / Baber et al. fixture stub |

---

## Acceptance Gate

- [ ] This file has exactly 24 named stub nodes
- [ ] Every non-shadow, non-synchronicity-fixture node has a real source anchor
- [ ] All shadow nodes have `hazard = Hazard` and `gaia_enabled = false`
- [ ] All consciousness nodes have `hazard = Debated`
- [ ] `claim_sentience()` returns `Err(SentienceClaim)`
- [ ] No node has `gaia_enabled = true`
- [ ] `cargo test -p gaia-aimd` green
