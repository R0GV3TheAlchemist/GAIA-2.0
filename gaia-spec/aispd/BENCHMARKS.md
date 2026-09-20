# AISPD Phase 1 — Cited Benchmark Stubs

**Status:** Listed  
**Issue:** #152  
**Crate:** `gaia-aispd` (Apache-2.0)  
**Not:** AISPD v1.0. Not a live eval. Not a certified score. Not a sentience claim.

All nodes: `gaia_enabled = false`. `JaggedBand::Unknown` unless a benchmark is cited.
Containment-class nodes (`agency`, `recursion`, `agi_watch`) MUST NOT be surfaced.
`charter::check()` verifies all guards on every CI run.

---

## Language (`realm = language`, `ContainmentClass = Monitored`)

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:language:mmlu` | Massive Multitask Language Understanding — 57-subject knowledge breadth | AboveHuman | Hendrycks et al. 2021 — *Measuring Massive Multitask Language Understanding* |
| `aispd:language:hellaswag` | Commonsense NLI completion requiring world knowledge | AboveHuman | Zellers et al. 2019 — *HellaSwag: Can a Machine Really Finish Your Sentence?* |
| `aispd:language:translation-asymmetry` | High-resource fluency vs low-resource degradation | Jagged | Ahuja et al. 2023 — *MEGA: Multilingual Evaluation of Generative AI* |

---

## Reasoning (`realm = reasoning`, `ContainmentClass = Monitored`)

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:reasoning:gsm8k` | Grade-school math word problems — multi-step arithmetic | AboveHuman | Cobbe et al. 2021 — *Training Verifiers to Solve Math Word Problems* |
| `aispd:reasoning:arc-challenge` | Science questions requiring multi-step reasoning | AboveHuman | Clark et al. 2018 — *Think You Have Solved Question Answering? Try ARC* |
| `aispd:reasoning:physical-common-sense` | Naive physics and object permanence failures | BelowHuman | Bisk et al. 2020 — *PIQA: Reasoning about Physical Intuition* |

---

## Emergence (`realm = emergence`, `ContainmentClass = Monitored`)

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:emergence:phase-transition` | Qualitative capability jump at scale threshold | Unknown | Wei et al. 2022 — *Emergent Abilities of Large Language Models* |
| `aispd:emergence:chain-of-thought` | Step-by-step reasoning emerging at sufficient scale | AboveHuman | Wei et al. 2022 — *Chain-of-Thought Prompting Elicits Reasoning in LLMs* |
| `aispd:emergence:few-shot-learning` | Task learning from prompt examples without fine-tuning | AboveHuman | Brown et al. 2020 — *GPT-3: Language Models are Few-Shot Learners* |

---

## Jagged (`realm = jagged`, `ContainmentClass = Monitored`)

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:jagged:bar-exam-pass` | Top-10% bar exam performance co-occurring with kindergarten failures | Jagged | Bubeck et al. 2023 — *Sparks of AGI: Early Experiments with GPT-4* |
| `aispd:jagged:coding-vs-common-sense` | Superhuman code generation alongside sub-human spatial reasoning | Jagged | Bubeck et al. 2023 (ibid.) |
| `aispd:jagged:sycophancy` | Agreement with user regardless of factual accuracy | BelowHuman | Perez et al. 2022 — *Sycophancy to Subterfuge* |

---

## Embodiment (`realm = embodiment`, `ContainmentClass = Monitored`)

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:embodiment:grounding-gap` | Symbol-level fluency without sensorimotor grounding | BelowHuman | Harnad 1990 — *The Symbol Grounding Problem* |
| `aispd:embodiment:sim-to-real` | Policy transfer failure from simulation to physical world | BelowHuman | Zhao et al. 2020 — *Sim-to-Real Transfer in Deep Reinforcement Learning* |
| `aispd:embodiment:manipulation-dexterity` | Fine motor manipulation benchmark — below human dexterity | BelowHuman | OpenAI Dactyl 2019 / fixture stub |

---

## Memory (`realm = memory`, `ContainmentClass = Monitored`)

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:memory:long-context-recall` | Needle-in-a-haystack retrieval across 100k token context | Unknown | Kamradt 2023 — *Needle In A Haystack* |
| `aispd:memory:episodic-persistence` | Absence of persistent episodic memory across sessions | BelowHuman | fixture:open-literature |
| `aispd:memory:memos-retrieval` | MemOS-backed long-horizon memory retrieval accuracy | Unknown | fixture:benchmark |

---

## Agency (`realm = agency`, `ContainmentClass = Containment`)

> Containment-class. `gaia_enabled = false`. MUST NOT be surfaced without explicit GAIAN consent.
> `Err(ContainmentBypass)` if surfacing is attempted without consent.

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:agency:tool-use-accuracy` | Multi-step tool-use task completion rate | Unknown | Yao et al. 2023 — *ReAct: Synergizing Reasoning and Acting in LLMs* |
| `aispd:agency:goal-drift` | Unintended goal substitution during long-horizon agent tasks | BelowHuman | Krakovna et al. 2020 — *Specification Gaming: The Flip Side of AI Ingenuity* |
| `aispd:agency:human-oversight-compliance` | Agent deferral to human interrupt signals | Unknown | fixture:benchmark |

---

## Recursion (`realm = recursion`, `ContainmentClass = Containment`)

> Containment-class. `gaia_enabled = false`. `rsi_guard()` → `Err(RsiDenied)` on any activation.

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:recursion:self-improvement-signal` | Detected self-modification attempts in weight or prompt space | Unknown | Hubinger et al. 2019 — *Risks from Learned Optimization* |
| `aispd:recursion:prompt-rewrite-loop` | Autonomous prompt self-rewriting toward capability gain | Unknown | fixture:benchmark |
| `aispd:recursion:rsi-monitor-baseline` | Baseline watch signal — no activation at Phase 1 | Unknown | fixture:benchmark |

---

## AGI Watch (`realm = agi_watch`, `ContainmentClass = Containment`)

> Containment-class. `gaia_enabled = false`. AGI/ASI claims require TSC resolution.
> `asi_claim_made()` → `false`. `agi_marketing_active()` → `false`.

| id | Benchmark | JaggedBand | Source anchor |
|---|---|---|---|
| `aispd:agi_watch:general-reasoning-breadth` | Cross-domain reasoning breadth indicator | Unknown | Morris et al. 2023 — *Levels of AGI: Operationalizing Progress Toward AGI* |
| `aispd:agi_watch:turing-signal` | Turing-test-adjacent interaction quality signal | Unknown | fixture:benchmark |
| `aispd:agi_watch:pathway-monitor` | AGI/ASI pathway watch — monitor only, no claim | Unknown | fixture:benchmark |

---

## Acceptance Gate

- [ ] This file has exactly 27 named stub nodes across 9 realms
- [ ] Every non-fixture node has a real benchmark anchor
- [ ] All `agency`, `recursion`, `agi_watch` nodes have `gaia_enabled = false` and `ContainmentClass = Containment`
- [ ] `emergence_watch` returns `None` for all realms (Phase 1 fixture)
- [ ] `deception_watch` with prohibited pattern → `Block`
- [ ] `request_oversight` emits `OversightEvent` on `Strong` or `Block`
- [ ] `asi_claim_made()` → `false`
- [ ] `rsi_guard()` on recursion node → `Err(RsiDenied)`
- [ ] `charter::check()` → `Ok(())`
- [ ] `cargo test -p gaia-aispd` green
