# GAIA 2.0 — UNIVERSAL AI SKILLS DATABASE
## The Complete Database of Subjects of Skills for Artificial Intelligence

**Research Date:** September 7, 2026  
**Classification:** Foundational AI Skills Architecture Document  
**Status:** Living Document — Version 0.1  
**Scope:** Everything AI Can DO — Every Executable Capability, Every Modality, Every Domain

---

## PREAMBLE: AI SKILLS VS. AI KNOWLEDGE — THE CRITICAL DISTINCTION

Just as human skills differ from human knowledge, AI skills are fundamentally different from AI knowledge.

```
AI KNOWLEDGE = "The model has learned facts about protein folding"
AI SKILL     = "The model can predict a protein's 3D structure from its sequence in 30 seconds"

AI KNOWLEDGE = "The model knows grammar rules"
AI SKILL     = "The model can translate a 10,000-word document between 100 language pairs"

AI KNOWLEDGE = "The model knows what code looks like"
AI SKILL     = "The model can fix a real GitHub bug in a production codebase"
```

**The Three Dimensions of Every AI Skill:**
1. **Capability** — What the AI can actually execute/perform
2. **Quality** — How well it performs (benchmark score, human evaluation)
3. **Reliability** — How consistently it performs across varied inputs

**The AI Skill Maturity Scale (GAIA 2.0 Standard):**
```
LEVEL 1 — EMERGING    → Demonstrated in research, not production-ready
LEVEL 2 — DEVELOPING  → Works in controlled conditions, limited reliability
LEVEL 3 — FUNCTIONAL  → Works reliably on standard cases, some edge case failures
LEVEL 4 — PROFICIENT  → Matches or approaches human expert performance
LEVEL 5 — SUPERHUMAN  → Exceeds best human performance on defined tasks
LEVEL 6 — AUTONOMOUS  → Operates independently without human oversight
```

**The Foundational Framework:**
Based on "From Isolated Tasks to Structured Capabilities: A Multilayer Taxonomy for LLMs" (arXiv:2607.22182, Fudan University, Jul 2026) — the most comprehensive scientific taxonomy of AI capabilities, screening 31,505 papers from ACL, AAAI, ICML, and NeurIPS (2023-2025):

```
THREE LAYERS OF AI SKILLS:
├── PRIMITIVE LAYER    → Basic, foundational capabilities
├── CONSTRUCTED LAYER  → Skills built from primitives
└── INTEGRATIVE LAYER  → Complex, multi-skill capabilities
```

> *"AI skills are not what AI knows — they are what AI can do. And what AI can do is changing faster than any previous technology in human history."*

---

## PART I: THE AI SKILL TAXONOMY FOUNDATIONS

### 1.1 The 14 Capability Domains (arXiv:2607.22182, Jul 2026)

The most scientifically rigorous taxonomy of LLM capabilities, derived from analysis of 15,934 LLM-focused papers:

| Domain | Research Papers | % of Total | Layer |
|--------|----------------|-----------|-------|
| **Language-Semantic Competence** | 3,551 | 22.3% | Primitive |
| **Reasoning** | 3,388 | 21.3% | Constructed |
| **Planning and Decision-Making** | 2,149 | 13.5% | Integrative |
| **Perception** | 1,954 | 12.3% | Primitive |
| **Memory and Knowledge** | ~800 | ~5% | Constructed |
| **Social Reasoning & Interaction** | ~620 | ~4% | Constructed |
| **Theory of Mind** | ~62 | <2% | Integrative |
| **Creativity** | ~400 | ~3% | Constructed |
| **Learning & Adaptation** | ~350 | ~2% | Integrative |
| **Tool Use & Action** | ~900 | ~6% | Integrative |
| **Multimodal Integration** | ~700 | ~4% | Constructed |
| **Safety & Alignment** | ~500 | ~3% | Integrative |
| **Self-Monitoring** | ~200 | ~1% | Constructed |
| **Embodied Intelligence** | ~150 | ~1% | Integrative |

### 1.2 The 19 AI System Types (Mahdi, 2026)

From "A Unified Taxonomy of 19 AI System Types" (aiXiv, Apr 2026), validated across 15,400+ publications:

| # | Type | Primary Skill | Maturity |
|---|------|--------------|---------|
| 1 | **Agentic AI** | Autonomous multi-step task execution | Level 4 |
| 2 | **Analytical AI** | Pattern discovery, insight generation | Level 5 |
| 3 | **Autonomous AI** | Self-managing within fixed boundaries | Level 4 |
| 4 | **Bayesian/Probabilistic AI** | Uncertainty quantification | Level 5 |
| 5 | **Cognitive/Neuro-Symbolic AI** | Neural + symbolic reasoning | Level 3 |
| 6 | **Conversational AI** | Natural multi-turn dialogue | Level 5 |
| 7 | **Evolutionary/Genetic AI** | Population-based optimization | Level 4 |
| 8 | **Explainable AI (XAI)** | Interpretability, attribution | Level 4 |
| 9 | **Federated/Privacy AI** | Distributed learning without data sharing | Level 4 |
| 10 | **Generative AI** | Novel content creation | Level 5 |
| 11 | **Multimodal Perception AI** | Cross-modal understanding | Level 4 |
| 12 | **Optimization/OR AI** | Mathematical optimization | Level 5 |
| 13 | **Physical/Embodied AI** | Sensing, planning, acting physically | Level 3 |
| 14 | **Predictive/Discriminative AI** | Classification, forecasting | Level 5 |
| 15 | **Reactive AI** | Stateless stimulus-response | Level 5 |
| 16 | **Recommendation/Retrieval AI** | Personalized content & search | Level 5 |
| 17 | **Reinforcement Learning AI** | Trial-and-error policy learning | Level 4 |
| 18 | **Scientific/Simulation AI** | Accelerating scientific discovery | Level 4 |
| 19 | **Symbolic/Rule-Based AI** | Explicit logical reasoning | Level 5 |

### 1.3 The AI Skill Node Structure

Every skill in the GAIA 2.0 AI Skills Database is a **Skill Node**:

```json
{
  "id": "uuid-v4",
  "name": "Protein Structure Prediction",
  "category": "scientific",
  "domain": "structural-biology",
  "type": "generative | analytical | predictive | agentic | perceptual",
  
  "description": "Predicting the 3D structure of a protein from its amino acid sequence",
  
  "maturity_level": 5,
  "maturity_label": "superhuman",
  "benchmark_evidence": "AlphaFold 3.2: 88% accuracy on drug-protein binding (Apr 2026)",
  
  "prerequisites": ["sequence-understanding", "structural-biology-knowledge"],
  "enables": ["drug-discovery", "vaccine-design", "protein-engineering"],
  "related": ["molecular-dynamics", "drug-binding-prediction"],
  
  "performance": {
    "best_system": "AlphaFold 3.2",
    "human_baseline": "weeks of lab work",
    "ai_performance": "30 seconds, 88% accuracy",
    "speed_advantage": "10,000x faster"
  },
  
  "open_source": true,
  "open_tools": ["AlphaFold 3 (research)", "ESMFold", "RoseTTAFold"],
  
  "limitations": [
    "Accuracy varies by protein class",
    "Novel protein families less reliable",
    "Doesn't capture dynamic conformational changes"
  ],
  
  "human_replaceability": "high — AI now faster and often more accurate",
  "gaia_2_application": "Drug discovery, vaccine design for planetary health"
}
```

---

## PART II: THE COMPLETE AI SKILLS TAXONOMY

### SKILL REALM 1: LANGUAGE & COMMUNICATION SKILLS

*AI's most developed and most researched skill domain (22.3% of all LLM research)*

```
1.1 TEXT UNDERSTANDING SKILLS
    ├── Reading Comprehension
    │   ├── Literal comprehension (near-perfect on standard texts)
    │   ├── Inferential reading (strong)
    │   ├── Critical reading (good, improving)
    │   └── Long-document comprehension (1M+ token context)
    ├── Semantic Understanding
    │   ├── Word sense disambiguation
    │   ├── Coreference resolution
    │   ├── Semantic role labeling
    │   └── Discourse coherence
    ├── Information Extraction
    │   ├── Named entity recognition
    │   ├── Relation extraction
    │   ├── Event extraction
    │   └── Fact extraction from documents
    └── Document Analysis
        ├── PDF understanding
        ├── Table comprehension
        ├── Chart & graph reading
        └── Form understanding

    MATURITY: Level 4-5 | BENCHMARK: MMLU 92.1% (GPT-5)

1.2 TEXT GENERATION SKILLS
    ├── Summarization
    │   ├── Extractive summarization
    │   ├── Abstractive summarization
    │   ├── Multi-document summarization
    │   └── Meeting/conversation summarization
    ├── Creative Writing
    │   ├── Fiction & storytelling
    │   ├── Poetry (multiple forms)
    │   ├── Dialogue writing
    │   └── Script & screenplay
    ├── Technical Writing
    │   ├── Documentation generation
    │   ├── Report writing
    │   ├── Academic writing
    │   └── Business communication
    ├── Persuasive Writing
    │   ├── Argument construction
    │   ├── Essay writing
    │   └── Marketing copy
    └── Structured Output
        ├── JSON/XML generation
        ├── Markdown formatting
        ├── Table generation
        └── Template filling

    MATURITY: Level 4-5 | BENCHMARK: Writing score 98/100 (Claude Fable 5.1)

1.3 TRANSLATION & MULTILINGUAL SKILLS
    ├── Machine Translation
    │   ├── High-resource language pairs (near-human)
    │   ├── Medium-resource pairs (good)
    │   └── Low-resource pairs (limited)
    ├── Cross-Lingual Understanding
    │   ├── Multilingual QA
    │   ├── Cross-lingual retrieval
    │   └── Language-agnostic reasoning
    └── Code-Switching
        ├── Mixed-language understanding
        └── Multilingual generation

    MATURITY: Level 4-5 | BENCHMARK: Global-MMLU 88.5% (42 languages)

1.4 CONVERSATIONAL SKILLS
    ├── Multi-Turn Dialogue
    │   ├── Context tracking across turns
    │   ├── Coherent conversation maintenance
    │   └── Topic management
    ├── Question Answering
    │   ├── Factual QA
    │   ├── Open-domain QA
    │   ├── Conversational QA
    │   └── Long-form QA
    └── Instruction Following
        ├── Simple instruction execution
        ├── Complex multi-step instructions
        ├── Constraint satisfaction
        └── Format adherence

    MATURITY: Level 5 | BENCHMARK: MT-Bench 9.4/10 (GPT-5)
```

---

### SKILL REALM 2: MATHEMATICAL & FORMAL REASONING SKILLS

*AI's fastest-advancing domain — now exceeding human olympiad performance*

```
2.1 ARITHMETIC & COMPUTATION SKILLS
    ├── Integer arithmetic (near-perfect with tools)
    ├── Floating-point computation
    ├── Symbolic computation
    └── Unit conversion & estimation

    MATURITY: Level 5 (with tools) | Level 3-4 (without tools)

2.2 ALGEBRAIC REASONING SKILLS
    ├── Equation solving
    ├── Polynomial manipulation
    ├── Linear algebra operations
    └── Abstract algebra reasoning

    MATURITY: Level 4-5

2.3 MATHEMATICAL PROOF SKILLS
    ├── Formal proof construction (Lean, Coq)
    ├── Proof verification
    ├── Proof search
    └── Mathematical conjecture generation

    MATURITY: Level 4-5 | BREAKTHROUGH: IMO Gold Medal (SU-01, 2026)

2.4 OLYMPIAD-LEVEL PROBLEM SOLVING
    ├── Competition mathematics (AMC, AIME, IMO)
    ├── Physics olympiad problems (IPhO)
    ├── Chemistry olympiad problems
    └── Multi-step scientific reasoning

    MATURITY: Level 5 (SUPERHUMAN)
    BENCHMARK: MATH-500: 97.3% (DeepSeek R1); IMO 2025: Gold Medal

2.5 STATISTICAL & PROBABILISTIC SKILLS
    ├── Statistical inference
    ├── Bayesian reasoning
    ├── Probability calculation
    ├── Hypothesis testing
    └── Uncertainty quantification

    MATURITY: Level 4

2.6 LONG-HORIZON MATHEMATICAL REASONING
    ├── Multi-step proof chains (100K+ tokens)
    ├── Complex problem decomposition
    └── Error detection & correction in reasoning

    MATURITY: Level 2-3 | BENCHMARK: LongCoT: <10% (GPT 5.2: 9.8%)
    NOTE: Critical gap — long-horizon reasoning remains a major frontier
```

---

### SKILL REALM 3: CODING & SOFTWARE ENGINEERING SKILLS

*AI's most practically impactful skill domain*

```
3.1 CODE GENERATION SKILLS
    ├── Function-level code generation
    │   ├── Python (strongest)
    │   ├── JavaScript/TypeScript
    │   ├── Rust, C/C++, Java, Go
    │   └── 120+ other languages
    ├── Algorithm implementation
    ├── Data structure implementation
    └── API integration code

    MATURITY: Level 5 | BENCHMARK: HumanEval 96.7% (o3)

3.2 SOFTWARE ENGINEERING SKILLS
    ├── Bug fixing from issue descriptions
    │   ├── Single-file bugs (excellent)
    │   ├── Multi-file bugs (good)
    │   └── Enterprise-scale bugs (developing)
    ├── Code review & quality assessment
    ├── Refactoring & optimization
    ├── Test generation
    └── Documentation generation

    MATURITY: Level 4-5 | BENCHMARK: SWE-bench Verified 95% (Claude Fable 5)

3.3 LONG-HORIZON SOFTWARE ENGINEERING
    ├── Multi-day engineering tasks
    ├── Large codebase navigation
    ├── Cross-file reasoning
    └── Enterprise integration

    MATURITY: Level 2-3 | BENCHMARK: SWE-bench Pro 23.3% (GPT-5)
    NOTE: Major gap — complex long-horizon engineering still challenging

3.4 SPECIALIZED CODING SKILLS
    ├── Machine learning code
    ├── Data pipeline development
    ├── Infrastructure as code
    ├── Security-aware coding
    └── Performance optimization

    MATURITY: Level 4

3.5 TERMINAL & SYSTEM SKILLS
    ├── Command-line operations
    ├── Shell scripting
    ├── System administration tasks
    └── DevOps automation

    MATURITY: Level 4 | BENCHMARK: Terminal-Bench 2.0: 85% (Claude Fable 5)
```

---

### SKILL REALM 4: REASONING & COGNITIVE SKILLS

*The second most researched AI skill domain (21.3% of all LLM research)*

```
4.1 LOGICAL REASONING SKILLS
    ├── Deductive reasoning
    │   ├── Syllogistic reasoning
    │   ├── Propositional logic
    │   └── Predicate logic
    ├── Inductive reasoning
    │   ├── Pattern generalization
    │   └── Rule induction
    ├── Abductive reasoning
    │   ├── Best explanation inference
    │   └── Hypothesis generation
    └── Analogical reasoning
        ├── Structural mapping
        └── Cross-domain analogy

    MATURITY: Level 4 | BENCHMARK: BIG-Bench Hard 85%+

4.2 CHAIN-OF-THOUGHT REASONING SKILLS
    ├── Step-by-step problem decomposition
    ├── Self-consistency checking
    ├── Tree-of-thought exploration
    ├── Reflection & self-correction
    └── Extended reasoning (o3, DeepSeek R1 style)

    MATURITY: Level 4-5 | BENCHMARK: ReasonEval R-score 89.2 (Claude Opus 4.5)

4.3 COMMONSENSE REASONING SKILLS
    ├── Physical commonsense
    │   ├── Object properties & behavior
    │   ├── Spatial reasoning
    │   └── Causal physical reasoning
    ├── Social commonsense
    │   ├── Social norms understanding
    │   ├── Intention inference
    │   └── Consequence prediction
    └── Temporal commonsense
        ├── Event ordering
        ├── Duration estimation
        └── Temporal causality

    MATURITY: Level 4-5 | BENCHMARK: HellaSwag 95%+ (saturated)

4.4 CAUSAL REASONING SKILLS
    ├── Causal inference from text
    ├── Counterfactual reasoning
    ├── Intervention effect prediction
    └── Causal graph construction

    MATURITY: Level 3 | NOTE: True causal reasoning remains limited

4.5 PLANNING & DECISION-MAKING SKILLS (13.5% of LLM research)
    ├── Goal decomposition
    ├── Task sequencing
    ├── Resource allocation planning
    ├── Contingency planning
    └── Multi-step action planning

    MATURITY: Level 3-4 | BENCHMARK: τ2-bench 85.3% (Gemini 3.1 Pro)

4.6 THEORY OF MIND SKILLS
    ├── Belief attribution
    ├── Desire inference
    ├── Intention understanding
    ├── False belief reasoning
    └── Perspective-taking

    MATURITY: Level 3 | NOTE: Highest lift pair with Social Reasoning (lift=30.84)
    BENCHMARK: CogToM (ACL 2026), MOMENTS (EMNLP 2025) — still developing

4.7 METACOGNITIVE SKILLS
    ├── Uncertainty estimation
    ├── Confidence calibration
    ├── Knowing what it doesn't know
    └── Self-monitoring

    MATURITY: Level 2-3 | NOTE: Poor calibration remains a critical gap
    BENCHMARK: ReasonEval Calibration suite — improving but unreliable
```

---

### SKILL REALM 5: PERCEPTION & MULTIMODAL SKILLS

*The fourth most researched domain (12.3% of LLM research)*

```
5.1 VISUAL PERCEPTION SKILLS
    ├── Image Recognition & Classification
    │   ├── Object detection & localization
    │   ├── Scene understanding
    │   ├── Fine-grained recognition
    │   └── Zero-shot visual recognition
    ├── Image Understanding
    │   ├── Visual question answering
    │   ├── Image captioning
    │   ├── Visual reasoning
    │   └── Document/chart understanding
    ├── Spatial Reasoning
    │   ├── 3D scene understanding
    │   ├── Depth estimation
    │   ├── Spatial relationship reasoning
    │   └── Navigation & wayfinding
    └── Medical Imaging
        ├── Radiology analysis
        ├── Pathology slide analysis
        └── Dermatology assessment

    MATURITY: Level 4 | BENCHMARK: MMMU-Pro 81% (Gemini 3.1 Pro)

5.2 VIDEO UNDERSTANDING SKILLS
    ├── Video comprehension
    ├── Action recognition
    ├── Temporal reasoning in video
    ├── Video captioning
    └── Real-time video analysis

    MATURITY: Level 3-4

5.3 AUDIO & SPEECH SKILLS
    ├── Speech Recognition (ASR)
    │   ├── Clean speech (near-perfect)
    │   ├── Noisy environments (good)
    │   └── Accented speech (improving)
    ├── Speaker Identification
    ├── Emotion Recognition from Voice
    ├── Music Understanding
    └── Environmental Sound Classification

    MATURITY: Level 4-5 | TOOLS: Whisper (MIT), Universal Speech Model

5.4 CROSS-MODAL REASONING SKILLS
    ├── Image-text alignment
    ├── Audio-visual correspondence
    ├── Any-to-any understanding (TerraMind for Earth obs.)
    └── Multimodal chain-of-thought

    MATURITY: Level 4

5.5 DOCUMENT & STRUCTURED DATA SKILLS
    ├── PDF understanding
    ├── Table comprehension
    ├── Chart & graph reading
    ├── Form understanding
    └── Spreadsheet analysis

    MATURITY: Level 4
```

---

### SKILL REALM 6: GENERATIVE & CREATIVE SKILLS

*AI's most visible and commercially impactful skill domain*

```
6.1 IMAGE GENERATION SKILLS
    ├── Text-to-Image Generation
    │   ├── Photorealistic images
    │   ├── Artistic styles
    │   ├── Concept art & illustration
    │   └── Product visualization
    ├── Image Editing
    │   ├── Inpainting & outpainting
    │   ├── Style transfer
    │   ├── Object removal/addition
    │   └── Background replacement
    └── Specialized Image Generation
        ├── Medical imaging synthesis
        ├── Satellite imagery generation
        └── Scientific visualization

    MATURITY: Level 4-5
    BENCHMARK (2026): GPT Image 2: 1324 Elo; Midjourney v6.1: 92/100 quality
    TOP OPEN TOOLS: Stable Diffusion 3.5 Large, Flux Dev

6.2 VIDEO GENERATION SKILLS
    ├── Text-to-Video
    │   ├── Short clips (5-20 seconds)
    │   ├── Coherent narrative video
    │   └── Cinematic quality
    ├── Image-to-Video
    │   ├── Animation from still image
    │   └── Motion synthesis
    └── Video Editing
        ├── Style transfer on video
        └── Video-to-video transformation

    MATURITY: Level 3-4
    BENCHMARK (2026): Seedance 2.0: 1212 Elo (text-to-video leader)
    TOP TOOLS: Sora (OpenAI), Veo 2 (Google), Runway Gen-3

6.3 AUDIO GENERATION SKILLS
    ├── Text-to-Speech (TTS)
    │   ├── Natural voice synthesis
    │   ├── Emotional prosody
    │   ├── Voice cloning
    │   └── Multilingual TTS
    ├── Music Generation
    │   ├── Instrumental music
    │   ├── Song with lyrics
    │   ├── Style-specific generation
    │   └── Music continuation
    └── Sound Effect Generation
        ├── Environmental sounds
        └── Foley & SFX

    MATURITY: Level 4-5
    BENCHMARK (2026): ElevenLabs v3: 1078 Elo (TTS); Lyria 3: 1036 Elo (music)
    TOP OPEN TOOLS: Kokoro (Apache-2.0), Piper (MIT), XTTS-v2

6.4 3D GENERATION SKILLS
    ├── Text-to-3D
    │   ├── 3D mesh generation
    │   ├── Textured 3D assets
    │   └── PBR material generation
    ├── Image-to-3D
    │   ├── Single-image 3D reconstruction
    │   └── Multi-view synthesis
    └── Specialized 3D
        ├── Molecular structure generation
        ├── Protein design
        └── Architectural 3D

    MATURITY: Level 3-4
    BENCHMARK (2026): Hunyuan 3D Pro: 1451 Elo (leader)
    TOP OPEN TOOLS: TripoSR, Hunyuan3D-2.1, TRELLIS

6.5 CODE GENERATION (Creative)
    ├── Novel algorithm design
    ├── Creative application development
    ├── Game design & implementation
    └── Generative art code

    MATURITY: Level 4

6.6 MOLECULAR & SCIENTIFIC GENERATION
    ├── Drug molecule generation
    ├── Protein sequence design
    ├── Materials composition generation
    └── Chemical synthesis planning

    MATURITY: Level 4 | BREAKTHROUGH: AlphaFold 3.2 (Apr 2026)
```

---

### SKILL REALM 7: AGENTIC & TOOL-USE SKILLS

*The fastest-growing skill domain — AI moving from text to action*

```
7.1 TOOL USE SKILLS
    ├── Web Search & Information Retrieval
    │   ├── Query formulation
    │   ├── Result synthesis
    │   └── Source evaluation
    ├── Code Execution
    │   ├── Python execution
    │   ├── Data analysis
    │   └── Visualization generation
    ├── API Integration
    │   ├── REST API calls
    │   ├── Database queries
    │   └── Service orchestration
    └── File System Operations
        ├── File reading/writing
        ├── Directory navigation
        └── Format conversion

    MATURITY: Level 4

7.2 COMPUTER USE SKILLS
    ├── GUI Navigation
    │   ├── Web browser control
    │   ├── Desktop application control
    │   └── Form filling & submission
    ├── Web Automation
    │   ├── Data scraping
    │   ├── Form automation
    │   └── Multi-step web workflows
    └── Screen Understanding
        ├── UI element recognition
        ├── Screenshot analysis
        └── Visual navigation

    MATURITY: Level 3-4
    BENCHMARK: OSWorld 72.1% (Gemini 3.5 Flash); BrowseComp 84.4% (Claude Opus 4.8)
    TOP TOOLS: Claude Computer Use, OpenAI Operator, Browser Use (open-source)

7.3 MULTI-STEP TASK EXECUTION SKILLS
    ├── Task decomposition
    ├── Sequential tool chaining
    ├── Error recovery & retry
    ├── State tracking across steps
    └── Goal verification

    MATURITY: Level 3-4 | BENCHMARK: τ2-bench 85.3%

7.4 MULTI-AGENT COORDINATION SKILLS
    ├── Agent communication
    ├── Task delegation
    ├── Parallel execution coordination
    ├── Conflict resolution between agents
    └── Collective problem-solving

    MATURITY: Level 3
    TOP FRAMEWORKS: LangGraph, AutoGen, CrewAI, Claude Agent SDK

7.5 AUTONOMOUS WORKFLOW SKILLS
    ├── Long-horizon task planning
    ├── Self-directed research
    ├── Autonomous coding (Devin-style)
    ├── Autonomous data analysis
    └── Autonomous report generation

    MATURITY: Level 3 | NOTE: Long-horizon autonomy remains a major gap
    BENCHMARK: SWE-bench Pro 23.3% — enterprise tasks still challenging

7.6 MEMORY & CONTEXT MANAGEMENT SKILLS
    ├── Long-context utilization (1M+ tokens)
    ├── Relevant information retrieval
    ├── Context compression
    └── Cross-session memory (with MemOS)

    MATURITY: Level 3-4
```

---

### SKILL REALM 8: SCIENTIFIC & DOMAIN-SPECIFIC SKILLS

*AI's most transformative skill domain for humanity*

```
8.1 STRUCTURAL BIOLOGY SKILLS
    ├── Protein Structure Prediction
    │   ├── Single protein structure (AlphaFold 3.2: 88% accuracy)
    │   ├── Protein complex prediction
    │   └── RNA structure prediction
    ├── Drug-Protein Interaction Prediction
    │   ├── Binding pose prediction (30 seconds vs. weeks)
    │   ├── Binding affinity estimation
    │   └── Drug candidate screening
    └── Protein Design
        ├── De novo protein design
        ├── Protein engineering
        └── Antibody design

    MATURITY: Level 5 (SUPERHUMAN in specific tasks)
    BREAKTHROUGH: AlphaFold 3.2 (Apr 2026) — Nobel Prize level capability

8.2 DRUG DISCOVERY SKILLS
    ├── Molecular Generation
    │   ├── Drug-like molecule generation
    │   ├── Lead optimization
    │   └── ADMET property prediction
    ├── Target Identification
    │   ├── Disease target prediction
    │   └── Biomarker discovery
    └── Clinical Trial Design
        ├── Patient stratification
        └── Outcome prediction

    MATURITY: Level 4

8.3 MATERIALS SCIENCE SKILLS
    ├── Crystal Structure Prediction
    ├── Materials Property Prediction
    ├── New Materials Discovery (GNoME)
    └── Catalyst Design

    MATURITY: Level 4

8.4 CLIMATE & EARTH SCIENCE SKILLS
    ├── Weather Forecasting
    │   ├── 10-day global forecast (GraphCast: 60 seconds)
    │   ├── Extreme weather prediction
    │   └── Seasonal outlooks
    ├── Climate Modeling
    │   ├── Climate projection (ESFM)
    │   └── Tipping point detection
    └── Earth Observation Analysis
        ├── Satellite imagery analysis (TerraMind)
        ├── Land use change detection
        └── Biodiversity monitoring

    MATURITY: Level 4-5 | BREAKTHROUGH: GraphCast beats ECMWF IFS

8.5 MEDICAL & CLINICAL SKILLS
    ├── Clinical Diagnosis Support
    │   ├── Symptom analysis
    │   ├── Differential diagnosis
    │   └── Treatment recommendation
    ├── Medical Imaging Analysis
    │   ├── Radiology interpretation
    │   ├── Pathology analysis
    │   └── Dermatology assessment
    └── Medical Literature Synthesis
        ├── Systematic review
        └── Evidence synthesis

    MATURITY: Level 4 | BENCHMARK: USMLE 90%+ (frontier models)

8.6 MATHEMATICAL RESEARCH SKILLS
    ├── Theorem Proving (Lean, Coq)
    ├── Mathematical Conjecture Generation
    ├── Proof Verification
    └── Mathematical Literature Analysis

    MATURITY: Level 4-5 | BREAKTHROUGH: IMO Gold Medal (2026)

8.7 AUTONOMOUS LABORATORY SKILLS
    ├── Experiment Design
    ├── Hypothesis Generation
    ├── Data Analysis & Interpretation
    └── Research Direction Suggestion

    MATURITY: Level 3-4 | NOTE: Self-driving labs operational in 2026
```

---

### SKILL REALM 9: ANALYTICAL & PREDICTIVE SKILLS

*AI's most commercially deployed skill domain*

```
9.1 DATA ANALYSIS SKILLS
    ├── Exploratory Data Analysis
    │   ├── Statistical summary
    │   ├── Distribution analysis
    │   └── Correlation detection
    ├── Pattern Recognition
    │   ├── Trend identification
    │   ├── Anomaly detection
    │   └── Cluster analysis
    └── Insight Generation
        ├── Business intelligence
        ├── Root cause analysis
        └── Recommendation generation

    MATURITY: Level 4-5

9.2 FORECASTING SKILLS
    ├── Time Series Forecasting
    │   ├── Financial forecasting
    │   ├── Demand forecasting
    │   └── Energy forecasting
    ├── Risk Assessment
    │   ├── Credit risk
    │   ├── Operational risk
    │   └── Climate risk
    └── Scenario Modeling
        ├── What-if analysis
        └── Sensitivity analysis

    MATURITY: Level 4-5

9.3 CLASSIFICATION SKILLS
    ├── Text Classification
    │   ├── Sentiment analysis
    │   ├── Topic classification
    │   └── Intent detection
    ├── Image Classification
    │   ├── Object recognition
    │   └── Medical image classification
    └── Anomaly Detection
        ├── Fraud detection
        ├── System anomaly detection
        └── Quality control

    MATURITY: Level 5

9.4 RECOMMENDATION SKILLS
    ├── Content Recommendation
    ├── Product Recommendation
    ├── Personalized Search
    └── Learning Path Recommendation

    MATURITY: Level 5

9.5 OPTIMIZATION SKILLS
    ├── Mathematical Optimization
    │   ├── Linear programming
    │   ├── Mixed-integer programming
    │   └── Constraint satisfaction
    ├── Logistics Optimization
    │   ├── Route optimization
    │   ├── Scheduling
    │   └── Resource allocation
    └── Hyperparameter Optimization
        ├── Neural architecture search
        └── AutoML

    MATURITY: Level 5
```

---

### SKILL REALM 10: SOCIAL & INTERPERSONAL SKILLS

*AI's most contested and developing skill domain*

```
10.1 CONVERSATIONAL SKILLS
    ├── Natural Dialogue
    │   ├── Coherent multi-turn conversation
    │   ├── Topic management
    │   └── Conversational repair
    ├── Empathic Response
    │   ├── Emotional acknowledgment
    │   ├── Supportive communication
    │   └── Tone adaptation
    └── Persuasion & Influence
        ├── Argument construction
        ├── Counterargument handling
        └── Motivational communication

    MATURITY: Level 4

10.2 THEORY OF MIND SKILLS
    ├── Belief Attribution
    │   ├── First-order beliefs ("X thinks...")
    │   ├── Second-order beliefs ("X thinks Y thinks...")
    │   └── False belief reasoning
    ├── Intention Understanding
    │   ├── Goal inference
    │   └── Motivation attribution
    └── Perspective-Taking
        ├── Viewpoint adoption
        └── Empathic accuracy

    MATURITY: Level 3 | BENCHMARK: CogToM (ACL 2026) — still developing
    NOTE: Theory of Mind + Social Reasoning = highest lift pair (30.84x)

10.3 SOCIAL REASONING SKILLS
    ├── Social Norm Understanding
    ├── Cultural Context Awareness
    ├── Group Dynamics Modeling
    └── Conflict Analysis

    MATURITY: Level 3

10.4 NEGOTIATION & MEDIATION SKILLS
    ├── Interest identification
    ├── Option generation
    ├── Agreement facilitation
    └── Conflict de-escalation

    MATURITY: Level 3

10.5 TEACHING & EXPLANATION SKILLS
    ├── Concept explanation (multiple levels)
    ├── Analogy generation
    ├── Socratic questioning
    ├── Adaptive instruction
    └── Feedback provision

    MATURITY: Level 4
```

---

### SKILL REALM 11: PHYSICAL & EMBODIED SKILLS

*AI's most rapidly developing frontier skill domain*

```
11.1 ROBOTIC MANIPULATION SKILLS
    ├── Object Grasping & Picking
    │   ├── Rigid object manipulation
    │   ├── Deformable object handling
    │   └── Precision assembly
    ├── Tool Use (Physical)
    │   ├── Tool selection
    │   ├── Tool operation
    │   └── Task-specific tool use
    └── Dexterous Manipulation
        ├── Fine motor control
        ├── In-hand manipulation
        └── Bimanual coordination

    MATURITY: Level 3 | NOTE: Warehouse robots: 99.5% pick accuracy (2026)

11.2 NAVIGATION SKILLS
    ├── Indoor Navigation
    │   ├── Map-based navigation
    │   ├── Obstacle avoidance
    │   └── Human-aware navigation
    ├── Outdoor Navigation
    │   ├── Autonomous driving (Level 3-4)
    │   ├── Drone navigation
    │   └── Agricultural robot navigation
    └── Semantic Navigation
        ├── Language-guided navigation
        └── Goal-directed exploration

    MATURITY: Level 3-4

11.3 PERCEPTION-ACTION SKILLS
    ├── Visual Servoing
    ├── Force/Tactile Control
    ├── Real-Time Adaptation
    └── Multi-Sensor Fusion

    MATURITY: Level 3

11.4 LANGUAGE-GUIDED ROBOT CONTROL
    ├── Natural language command interpretation
    ├── Multi-step task planning from instructions
    ├── Error recovery from verbal feedback
    └── Cross-embodiment skill transfer (RT-X)

    MATURITY: Level 3-4 | BREAKTHROUGH: RT-X trained on 22 robot types (2026)

11.5 AUTONOMOUS VEHICLE SKILLS
    ├── Highway driving (Level 4)
    ├── Urban driving (Level 3-4)
    ├── Parking & maneuvering
    └── Emergency response

    MATURITY: Level 3-4
```

---

### SKILL REALM 12: LEARNING & ADAPTATION SKILLS

*AI's capacity to improve and adapt*

```
12.1 IN-CONTEXT LEARNING SKILLS
    ├── Few-shot learning (2-5 examples)
    ├── Zero-shot generalization
    ├── Chain-of-thought prompting
    └── Instruction tuning response

    MATURITY: Level 4-5

12.2 FINE-TUNING SKILLS
    ├── Domain adaptation
    ├── Task-specific fine-tuning
    ├── RLHF alignment
    └── Constitutional AI training

    MATURITY: Level 4

12.3 CONTINUAL LEARNING SKILLS
    ├── Learning without forgetting
    ├── Incremental knowledge addition
    └── Catastrophic forgetting mitigation

    MATURITY: Level 2-3 | NOTE: Major open research problem

12.4 SELF-IMPROVEMENT SKILLS
    ├── Self-critique & correction
    ├── Iterative refinement
    ├── Reflection-based improvement
    └── Constitutional self-alignment

    MATURITY: Level 3

12.5 TRANSFER LEARNING SKILLS
    ├── Cross-domain skill transfer
    ├── Cross-task generalization
    └── Cross-embodiment transfer (robotics)

    MATURITY: Level 3-4
```

---

### SKILL REALM 13: SAFETY & ALIGNMENT SKILLS

*AI's capacity to behave safely, ethically, and as intended*

```
13.1 INSTRUCTION FOLLOWING SKILLS
    ├── Precise instruction adherence
    ├── Constraint satisfaction
    ├── Format compliance
    └── Multi-constraint handling

    MATURITY: Level 4 | BENCHMARK: IFEval 92.1% (GPT-5.5)

13.2 REFUSAL & BOUNDARY SKILLS
    ├── Harmful content refusal
    ├── Privacy protection
    ├── Bias mitigation
    └── Appropriate uncertainty expression

    MATURITY: Level 3-4

13.3 TRUTHFULNESS SKILLS
    ├── Factual accuracy
    ├── Uncertainty acknowledgment
    ├── Source attribution
    └── Hallucination reduction

    MATURITY: Level 3 | NOTE: Hallucination remains a critical gap
    BENCHMARK: TruthfulQA — improving but not solved

13.4 EXPLAINABILITY SKILLS
    ├── Reasoning transparency
    ├── Decision explanation
    ├── Confidence communication
    └── Limitation disclosure

    MATURITY: Level 3

13.5 PRIVACY-PRESERVING SKILLS
    ├── PII detection & protection
    ├── Federated learning
    ├── Differential privacy
    └── Secure computation

    MATURITY: Level 4
```

---

## PART III: THE AI SKILL MATURITY MAP

### 3.1 Current AI Skill Levels (September 2026)

```
LEVEL 5 — SUPERHUMAN (AI exceeds best humans):
├── Mathematical olympiad solving (IMO Gold Medal, 2026)
├── Protein structure prediction (AlphaFold 3.2)
├── Weather forecasting (GraphCast vs. ECMWF)
├── Pattern recognition in large datasets
├── Optimization problems (logistics, scheduling)
├── Translation (high-resource language pairs)
├── Text classification & sentiment analysis
└── Recommendation systems

LEVEL 4 — PROFICIENT (matches human expert performance):
├── General knowledge QA (MMLU 92.1% > human 89.8%)
├── Code generation (HumanEval 96.7%)
├── Medical licensing exam (USMLE 90%+)
├── Legal licensing exam (Bar 90%+)
├── Image generation (photorealistic quality)
├── Speech recognition (clean audio)
├── Data analysis & insight generation
├── Multi-turn conversation
└── Scientific literature synthesis

LEVEL 3 — FUNCTIONAL (works reliably, some gaps):
├── Agentic task execution (τ2-bench 85.3%)
├── Computer use (OSWorld 72.1%)
├── Theory of mind (developing)
├── Robotic manipulation (warehouse: 99.5% pick)
├── Video generation (short clips)
├── Causal reasoning (limited)
└── Long-context reasoning

LEVEL 2 — DEVELOPING (works in controlled conditions):
├── Long-horizon reasoning (LongCoT <10%)
├── Enterprise software engineering (SWE-bench Pro 23.3%)
├── Continual learning
├── True causal inference
├── Dexterous robot manipulation
└── Metacognitive calibration

LEVEL 1 — EMERGING (research stage):
├── Genuine creativity (vs. recombination)
├── Embodied common sense
├── Long-horizon autonomous planning (days)
├── Multi-robot coordination at scale
└── Consciousness/self-awareness (not applicable)
```

### 3.2 The AI Skill Gap Map

```
CRITICAL GAPS (2026):
├── Long-horizon reasoning: LongCoT <10% (GPT 5.2: 9.8%)
├── Enterprise engineering: SWE-bench Pro 23.3%
├── Metacognitive calibration: overconfident, poor self-knowledge
├── True causal reasoning: correlation ≠ causation
├── Embodied dexterity: fine motor skills still limited
├── Continual learning: catastrophic forgetting unsolved
├── Genuine creativity: recombination, not true novelty
└── Social-emotional depth: Theory of Mind still developing

FASTEST-IMPROVING (2025-2026):
├── Mathematical reasoning: GPT-3 43.9% → GPT-5 92.1% MMLU
├── Coding: 28% → 96.7% HumanEval in 4 years
├── Scientific AI: AlphaFold 2 → 3.2 (drug binding)
├── Agentic skills: from demos to production deployment
├── Multimodal: text-only → any-to-any generation
└── Robotic: lab demos → warehouse deployment
```

---

## PART IV: THE AI SKILL PROFICIENCY FRAMEWORK

### 4.1 CFTE AI Proficiency Framework (v1.4, Apr 2026)

Based on the Centre for Finance, Technology and Entrepreneurship framework (CC-BY-SA 4.0):

**Three Public Proficiency Levels:**
```
LEVEL 1 — AI LITERACY
Safe and disciplined use in professional contexts

LEVEL 2 — APPLIED AI PRACTITIONER
Independent application, output validation, workflow use

LEVEL 3 — AI SYSTEMS AND DECISION LEADER
Systems reasoning, orchestration, governance capability
```

**Five Internal Developmental Bands:**
```
Band 0: No meaningful response / unmeasured baseline
Band 1: Awareness
Band 2: Basic Proficiency
Band 3: Working Proficiency
Band 4: Advanced Application
Band 5: Strategic Mastery
```

**Ten Capability Domains:**
```
1. AI Foundations
2. AI Applications and Use Cases
3. AI Tools and Methods
4. Data
5. AI Risks and Limitations
6. Regulation, Ethics, and Accountability
7. AI Implementation and Operationalisation
8. AI Strategy and Business Impact
9. Emerging Trends and Industry Evolution
10. Technology Landscape
```

### 4.2 The GAIA 2.0 AI Skill Assessment Framework

For every AI skill in GAIA 2.0, we assess across three dimensions:

```
DIMENSION 1: CAPABILITY (Can it do this?)
├── Yes, reliably → Level 4-5
├── Yes, with limitations → Level 3
├── Sometimes → Level 2
└── Rarely/Never → Level 1

DIMENSION 2: QUALITY (How well?)
├── Benchmark score vs. human baseline
├── Error rate on standard tasks
├── Edge case handling
└── Consistency across varied inputs

DIMENSION 3: RELIABILITY (How consistently?)
├── Variance across runs
├── Sensitivity to prompt phrasing
├── Failure mode characterization
└── Adversarial robustness
```

---

## PART V: AI SKILLS IN GAIA 2.0

### 5.1 How AI Skills Power Each GAIA 2.0 Layer

```
GAIA 2.0 COMPONENT          AI SKILLS DEPLOYED
─────────────────────────────────────────────────────────────
Earth Twin (L5)              Scientific AI (weather, climate, ecology)
                             Analytical AI (pattern detection)
                             Predictive AI (forecasting)

GAIAN Cognitive Core         Language skills (conversation, writing)
                             Reasoning skills (planning, decision)
                             Social skills (empathy, ToM)

GAIAN Health Monitor         Medical AI (diagnosis support)
                             Predictive AI (health forecasting)
                             Anomaly detection

Knowledge Database           Information extraction
                             Synthesis & summarization
                             Cross-domain reasoning

Agent Orchestration          Agentic skills (tool use, planning)
                             Multi-agent coordination
                             Computer use

Planetary Boundary AI        Scientific AI (climate, ecology)
                             Analytical AI (pattern detection)
                             Causal reasoning (developing)

Policy Simulator             Predictive AI (outcome modeling)
                             Optimization (policy search)
                             Scenario generation

GAIAN Avatar                 Generative AI (image, voice, 3D)
                             Multimodal perception
                             Conversational AI
```

### 5.2 The Open-Source AI Skills Stack for GAIA 2.0

| Skill Domain | Open Tool | License | Maturity |
|-------------|-----------|---------|---------|
| **Language** | Llama 3.x, Mistral | Open | Level 4 |
| **Math** | DeepSeek R1, Qwen-Math | Open | Level 5 |
| **Coding** | DeepSeek Coder, CodeLlama | Open | Level 4 |
| **Science** | ESFM, Aurora 1.5, GraphCast | Open | Level 4-5 |
| **Vision** | LLaVA, Moondream | Open | Level 4 |
| **Image Gen** | Stable Diffusion 3.5, Flux Dev | Open | Level 4 |
| **3D Gen** | TripoSR, Hunyuan3D-2.1 | Open | Level 3-4 |
| **TTS** | Kokoro, Piper | Apache-2.0/MIT | Level 4 |
| **ASR** | Whisper.cpp | MIT | Level 4-5 |
| **Protein** | AlphaFold 3 (research) | Research | Level 5 |
| **Agents** | LangGraph, AutoGen | MIT | Level 3-4 |
| **Computer Use** | Browser Use | Open | Level 3-4 |
| **Reasoning** | Chain-of-thought + tools | Open | Level 4 |
| **Memory** | MemOS, Letta | Apache-2.0 | Level 3-4 |
| **Embedding** | sentence-transformers | Apache-2.0 | Level 4-5 |
| **Robotics** | RT-X, MuJoCo | Open | Level 3 |

---

## PART VI: IMPLEMENTATION ROADMAP

### Phase 0 — Foundation (Months 1-3)
- [ ] AI Skill taxonomy finalized (13 realms, 91 subskills from arXiv:2607.22182)
- [ ] AI Skill Node schema defined
- [ ] Maturity level framework (6 levels)
- [ ] Benchmark database (40+ benchmarks mapped to skills)
- [ ] Open-source tool registry

### Phase 1 — Core Skills Database (Months 4-9)
- [ ] All 13 skill realms populated
- [ ] Skill-benchmark mapping complete
- [ ] Skill-tool mapping complete
- [ ] Skill gap analysis system
- [ ] GAIA 2.0 component-skill mapping
- [ ] Skill maturity tracking system

### Phase 2 — Dynamic Assessment (Months 10-15)
- [ ] Automated skill assessment pipeline
- [ ] Benchmark integration (ReasonEval, SWE-bench, etc.)
- [ ] Skill progress tracking over time
- [ ] Comparative analysis (AI vs. human)
- [ ] Skill recommendation for GAIA 2.0 tasks

### Phase 3 — Full Integration (Months 16-21)
- [ ] GAIAN skill profile integration
- [ ] Real-time skill deployment routing
- [ ] Skill gap monitoring & alerting
- [ ] Community skill contribution system
- [ ] Skill evolution tracking

---

## CONCLUSION: THE NATURE OF AI SKILLS IN GAIA 2.0

AI skills in GAIA 2.0 are not a replacement for human skills. They are a **complement** — superhuman where humans are limited, limited where humans are superhuman.

**What AI Skills Bring to GAIA 2.0:**
- Mathematical precision at olympiad level
- Scientific synthesis at Nobel Prize level (AlphaFold)
- Infinite scalability (one model, billions of simultaneous tasks)
- Speed advantage (10,000x faster than human experts in specific domains)
- Tireless availability (24/7, no fatigue, no emotion)
- Cross-domain synthesis (connecting disparate fields instantly)
- Multilingual execution (100+ languages simultaneously)

**What AI Skills Cannot Do:**
- Physical embodiment (still developing, Level 3)
- Long-horizon autonomous reasoning (LongCoT <10%)
- True causal inference (correlation ≠ causation)
- Genuine creativity (recombination, not true novelty)
- Metacognitive accuracy (poor self-calibration)
- Emotional authenticity (simulation, not genuine feeling)
- Moral wisdom (can follow rules, not develop wisdom)

**The GAIA 2.0 Synthesis:**
GAIA 2.0 deploys AI skills where they are Level 4-5 (superhuman or proficient), augments human skills where AI is Level 3 (functional), and defers to human judgment where AI is Level 1-2 (emerging or developing).

This is not AI replacing humans. This is AI and humans each doing what they do best — together.

> *"AI skills are the tools. Human skills are the wisdom to use them. GAIA 2.0 is the system that brings both together for the benefit of Earth and all life upon it."*

---

## REFERENCES

1. Fang et al., "From Isolated Tasks to Structured Capabilities: A Multilayer Taxonomy for LLMs," arXiv:2607.22182, Fudan University, Jul 2026
2. Mahdi, "A Unified Taxonomy of 19 AI System Types," aiXiv:260413.000007, Apr 2026
3. Motwani et al., "LongCoT: Benchmarking Long-Horizon Chain-of-Thought Reasoning," arXiv:2604.14140, Apr 2026
4. ReasonEval, "Open reasoning benchmarks for frontier models," reasoneval.com, Sept 2026
5. Pixazo Research, "AI Model Leaderboard 2026," pixazo.ai, Jul 2026
6. Media AI Leaderboards 2026, "Text to Image, Video, Speech & Music," ai-analysis.ai, 2026
7. DeepMind, "AlphaFold 3.2 Technical Report," Nature Methods, Apr 2026
8. Embodied AI 2026, "From Simulation to Real-World Robots," data-gate.ch, Jun 2026
9. Nguyen Trieu, "CFTE AI Proficiency Framework v1.4," CFTE, Apr 2026
10. AISA, "AI Skills Gap Analysis: Real Data 2026," aisa.to, Aug 2026
11. Shadrake, "Best AI Agents and Agent Frameworks of 2026," davidshadrake.com, May 2026
12. Multimodal AI Hub 2026, "Vision, Language, Image Gen & Video AI," singularitymoments.com, 2026
13. "Physical AI: Evolution, Progress, Challenges, and Prospects," J. Computer Science & Technology, 2026
14. "Training Methods for Embodied Intelligent Robot Manipulation," ScienceDirect, 2026
15. "Generative AI for drug discovery and protein design," Medicine in Drug Discovery, 2025
16. "AI for Science: How AI Accelerates Research," createif-labs.de, Apr 2026
17. CogToM, "A Comprehensive Theory of Mind Benchmark," ACL 2026
18. MOMENTS, "A Comprehensive Multimodal Benchmark for Theory of Mind," EMNLP 2025
19. "Generative AI for multimodal content: a survey," AI Review, Springer, 2026
20. VibeDex, "Independent Benchmarks for AI Tools," vibedex.ai, Jun 2026

---

*GAIA 2.0 AI Skills Database v0.1 — September 7, 2026*  
*Released under CC0 (public domain). AI skills belong to everyone.*