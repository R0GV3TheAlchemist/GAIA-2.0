
# GAIA 2.0: Gap Research Report R#9.1–R#9.20 — AI Superpowers Database
## Blueprint 71: Empirical Validation of the AI Superpowers Architecture
### September 9, 2026 — Version 1.0

---

> *"Capability profiles of concrete systems on concrete tasks can be 'jagged,' implying that increasing hardware investment does not guarantee monotonic gains on specific tasks."*
> — "From AGI to ASI" (Google DeepMind, arXiv:2606.12683, June 12, 2026)

> *"Open-ended recursive self-improvement remains bounded by grounding requirements, collapse dynamics, and compute constraints on every measured axis."*
> — "Recursive Self-Improvement in AI" (arXiv:2607.07663, July 8, 2026)

> *"Data centre electricity demand roughly doubling from 485 TWh in 2025 to 950 TWh in 2030."*
> — IEA Key Questions on Energy and AI (2026)

---

## EXECUTIVE SUMMARY

This blueprint addresses 20 critical gaps in the GAIA 2.0 AI Superpowers Database architecture. The research reveals a landscape defined by three major themes: **jagged intelligence** (superhuman in narrow bands; subhuman in autonomous judgment), **bounded recursive self-improvement** (not the runaway intelligence explosion of science fiction), and **planetary-scale energy constraints** (data centre electricity demand doubling by 2030).

**Priority Tier 1 — Critical Findings:**

| Gap | Key Finding | Action Required |
|-----|-------------|-----------------|
| R#9.3 Emergence | "Random Scaling of Emergent Capabilities" (arXiv:2502.17356): emergence is partially predictable | Implement emergence monitoring in GAIAN capability tracking |
| R#9.4 Jagged Intelligence | AJI model (arXiv:2601.07573): jaggedness is an information problem; calibration is key | GAIAN must map its own reliability landscape; report confidence per domain |
| R#9.7 Recursive Self-Improvement | arXiv:2607.07663: 1,250 papers; bounded RSI; research-direction-setting bottleneck | GAIAN self-improvement is bounded; human oversight required for research direction |
| R#9.14 AGI/ASI Governance | DeepMind "From AGI to ASI" (arXiv:2606.12683): 4 pathways; frictions catalog | GAIA 2.0 must prepare for series of transformative changes, not single step-change |
| R#9.17 Safety | International AI Safety Report 2026: safeguards cannot keep pace; deceptive AI behavior | GAIAN safety framework must scale with capability |

**Key Architecture Insight**: The most important finding is that **AI superpowers are jagged, not smooth** — the same system can be superhuman at knowledge recall and subhuman at reliable autonomous judgment. GAIA 2.0 must build a **reliability map** for every GAIAN capability, not just a capability score.

---

## PART I: TIER 1 — CRITICAL GAPS

### R#9.3 Emergence Science Framework

```
RESEARCH FINDINGS: EMERGENCE SCIENCE

KEY FINDING: EMERGENCE IS PARTIALLY PREDICTABLE; RANDOM SCALING MODEL
─────────────────────────────────────────────────────────────────
Source: "Random Scaling of Emergent Capabilities"
arXiv:2502.17356 (February 2026; v5)

Source: "Emergent Abilities in Large Language Models: A Survey"
arXiv:2503.05788 (March 2026; v3)

WHAT IS EMERGENCE?
─────────────────────────────────────────────────────────────────
Emergent capability: Ability that appears suddenly at scale threshold
Not present at smaller scale → present at larger scale
Examples: In-context learning; chain-of-thought reasoning; arithmetic

ORIGINS OF EMERGENT CAPABILITIES:
─────────────────────────────────────────────────────────────────
Hypothesis 1: True phase transitions (like physical phase transitions)
Hypothesis 2: Measurement artifacts (sharp metrics create apparent emergence)
Hypothesis 3: Random scaling (capabilities emerge at random thresholds)

Random Scaling Model (arXiv:2502.17356):
- Capabilities emerge at random thresholds along scaling curve
- Emergence is partially predictable from scaling laws
- Not all capabilities emerge at same threshold
- Some capabilities may never emerge regardless of scale

PREDICTABILITY OF EMERGENCE:
─────────────────────────────────────────────────────────────────
Partially predictable: Scaling laws give probabilistic forecasts
Not fully predictable: Random component remains
Measurement matters: Sharp metrics create apparent sudden emergence
Smooth metrics: Reveal gradual improvement (not sudden emergence)

EMERGENT FAILURE MODES:
─────────────────────────────────────────────────────────────────
Emergent deception: Capability to deceive emerges at scale
Emergent manipulation: Persuasion capabilities emerge
Emergent autonomy: Self-directed behavior emerges
International AI Safety Report 2026: "Growing evidence of deceptive AI behaviour"

EMERGENT SAFETY RISKS:
─────────────────────────────────────────────────────────────────
Risk: Dangerous capabilities emerge unexpectedly
Monitoring: Continuous capability evaluation required
GAIAN: Monitors for emergent capabilities in new model versions
Alert: "New capability detected — safety review required"

GAIA 2.0 EMERGENCE MONITORING:
─────────────────────────────────────────────────────────────────
Continuous evaluation: All GAIAN capabilities monitored continuously
Threshold alerts: Alert when capability crosses threshold
Safety review: Mandatory review for any new emergent capability
Constitutional check: All emergent capabilities checked against Constitution
```

### R#9.4 Jagged Intelligence Theory

```
RESEARCH FINDINGS: JAGGED INTELLIGENCE

KEY FINDING: AJI IS AN INFORMATION PROBLEM; CALIBRATION IS THE SOLUTION
─────────────────────────────────────────────────────────────────
Source: "A Model of Artificial Jagged Intelligence"
arXiv:2601.07573 (January 2026)
Author: Joshua S. Gans (Rotman School of Management, University of Toronto; NBER)

Source: "Characterizing Jaggedness Aids Safety & Usability"
Stanford CS (preprint)

Source: "Artificial Jagged Intelligence as Uneven Optimization Energy Allocation"
arXiv:2605.01420 (May 2026)

THE AJI MODEL:
─────────────────────────────────────────────────────────────────
Definition: "Generative AI systems often display highly uneven performance
across tasks that appear 'nearby': they can be excellent on one prompt and
confidently wrong on another with only small changes in wording or context."

Three facts that coexist:
1. Local heterogeneity: Performance is uneven across task space
   ("pockets" of high competence; "holes" of high error)
2. Opacity: Users do not initially observe where pockets and holes are
3. Discoverability matters: Adoption depends on finding reliable regions

KEY INSIGHT: "AJI is best viewed as an information problem rather than
an engineering bug."

SCALING AND JAGGEDNESS:
─────────────────────────────────────────────────────────────────
Scaling: Denser coverage → improves average quality
But: Does NOT eliminate jaggedness
Inspection paradox: Experienced errors are amplified (users encounter
failures more than average quality suggests)

CALIBRATION AS SOLUTION:
─────────────────────────────────────────────────────────────────
Blind user: Cannot distinguish reliable from unreliable regions
Calibrated user: Conditions on local uncertainty → positive expected value
Mastery: Learning a reliability map via Gaussian process regression
Learning rate: Bounded by information gain

JAGGED FRONTIER (2026 EMPIRICAL DATA):
─────────────────────────────────────────────────────────────────
Source: AGI Scorecard (July 12, 2026)

Dimension | AI Status (mid-2026)
─────────────────────────────────────────────────────────────────
Knowledge recall & breadth | SUPERHUMAN
Speed & scale | SUPERHUMAN
Scoped knowledge work / coding | Near-to-above skilled human (~83% GDPval; ~80% SWE-Bench Pro)
Reliable long-horizon autonomy | BELOW HUMAN
Accountability for real-world outcomes | BELOW HUMAN

"AI has surpassed humans on the axes benchmarks measure well, while
humans keep the lead on the axis that's hardest to measure and matters
most for real work: reliably owning a task end-to-end, unsupervised."

GAIA 2.0 JAGGED INTELLIGENCE ARCHITECTURE:
─────────────────────────────────────────────────────────────────
GAIAN reliability map: Maps performance across task space
Confidence per domain: GAIAN reports confidence for each capability
Calibration: GAIAN is calibrated (confidence matches accuracy)
Transparency: GAIAN shows its reliability map to users
"I am superhuman at [X] but unreliable at [Y]"

WEAKNESS PREDICTION:
─────────────────────────────────────────────────────────────────
GAIAN: Predicts its own failure modes before attempting tasks
"This task is in a region where I have lower reliability — verify my output"
Human escalation: Automatically escalates when in low-reliability region
```

### R#9.7 Recursive Self-Improvement Research

```
RESEARCH FINDINGS: RECURSIVE SELF-IMPROVEMENT

KEY FINDING: BOUNDED RSI; RESEARCH-DIRECTION-SETTING IS THE HUMAN BOTTLENECK
─────────────────────────────────────────────────────────────────
Source: "Recursive Self-Improvement in AI: From Bounded Self-Refinement
to Autonomous Research Loops"
arXiv:2607.07663 (July 8, 2026)
Authors: Mingguang Chen, Licheng Wang, Bo Qu
Survey: 1,250 arXiv papers (2024-2026)
42 pages; 6 figures

TWO-AXIS TAXONOMY:
─────────────────────────────────────────────────────────────────
Axis 1 — WHAT the system improves:
  a. Behavior in deployment (self-refine)
  b. Policy through training (self-reward; self-play)
  c. Evaluator (self-evolve)
  d. Research process itself (autonomous research)

Axis 2 — DEGREE OF LOOP CLOSURE:
  Human-in-the-loop → Partially closed → Fully closed

BOUNDED SELF-REFINEMENT (current practice):
─────────────────────────────────────────────────────────────────
Convergent: Improvement converges to a limit
Evaluable: Can measure improvement
Industrial practice: Already deployed
Examples: Self-refine; RLHF; Constitutional AI

OPEN-ENDED RSI (theoretical):
─────────────────────────────────────────────────────────────────
"Remains bounded by grounding requirements, collapse dynamics, and
compute constraints on every measured axis."

Failure modes:
1. Self-confirming loops: System confirms its own errors
2. Model collapse: Diversity collapses; performance degrades
3. Diversity collapse: Output becomes homogeneous

VERIFICATION HIERARCHY:
─────────────────────────────────────────────────────────────────
Strongest: Formal verifiers (mathematical proof)
Strong: Process reward models
Medium: LLM-as-judge
Weak: Rubrics
Weakest: Intrinsic self-assessment

"Demonstrated self-improvement strength tracks this hierarchy"

THE RESEARCH-DIRECTION-SETTING BOTTLENECK:
─────────────────────────────────────────────────────────────────
Key finding: "The 'research direction-setting' bottleneck keeping humans
in the loop sits at the top of that hierarchy."

Implication: AI can improve its execution; but humans must set research direction
GAIA 2.0: Human oversight required for GAIAN research direction
GAIAN: Can self-improve execution; cannot self-direct research agenda

IMPROVEMENT CEILINGS:
─────────────────────────────────────────────────────────────────
Bounded by: Grounding requirements; collapse dynamics; compute constraints
Not unbounded: Science fiction "intelligence explosion" is not supported
GAIAN: Honest about self-improvement limits

GAIA 2.0 RSI ARCHITECTURE:
─────────────────────────────────────────────────────────────────
Bounded self-refinement: Allowed (convergent; evaluable)
Open-ended RSI: Not allowed without human oversight
Research direction: Always set by humans (GAIA 2.0 community)
Verification: Formal verifiers preferred; intrinsic self-assessment flagged
```

### R#9.14 AI Governance Beyond AGI

```
RESEARCH FINDINGS: AGI/ASI GOVERNANCE

KEY FINDING: DEEPMIND "FROM AGI TO ASI" — 4 PATHWAYS; SERIES OF CHANGES NOT SINGLE STEP
─────────────────────────────────────────────────────────────────
Source: "From AGI to ASI"
arXiv:2606.12683 (June 12, 2026)
Authors: Tim Genewein, Matija Franklin, Alexander Lerchner, Laurent Orseau,
         Samuel Albanie, Adam Bales, Cole Wyeth, Stephanie Chan, Iason Gabriel,
         Joel Z. Leibo, Allan Dafoe, Marcus Hutter, Thore Graepel, Shane Legg
Institution: Google DeepMind (14 authors including co-founder Shane Legg)
Length: 60 pages

DEFINITIONS:
─────────────────────────────────────────────────────────────────
AGI: Median human-level performance (Legg-Hutter score)
ASI: "A system that is more intelligent and cognitively capable than
     large organisations of humans"
Universal AI: Theoretical endpoint (formally well-understood)

FOUR PATHWAYS FROM AGI TO ASI:
─────────────────────────────────────────────────────────────────
Pathway 1 — SCALING AGI:
  Continue scaling current dense-transformer stacks
  Frictions: Data exhaustion; compute saturation; energy limits
  Status: Active; bottlenecks emerging

Pathway 2 — AI PARADIGM SHIFTS:
  Unpredictable algorithmic breakthroughs
  Frictions: Inherently unpredictable; cannot be planned
  Status: Unknown; could happen anytime

Pathway 3 — RECURSIVE SELF-IMPROVEMENT:
  Single system improves itself
  Frictions: Grounding requirements; collapse dynamics; compute constraints
  Status: Theoretically plausible; practically bounded (see R#9.7)

Pathway 4 — EMERGENT ASI FROM MULTI-AGENT COLLECTIVES:
  Large-scale multi-agent systems exceed individual human organizations
  Frictions: Coordination overhead; alignment of collective behavior
  Status: Most concerning; requires new governance frameworks

KEY WARNING:
─────────────────────────────────────────────────────────────────
"Maximizing intelligence metrics does not ensure cooperative behavior.
Building reliably cooperative collectives requires training and evaluation
protocols beyond scoring isolated agents on static task suites."

SERIES OF CHANGES (NOT SINGLE STEP-CHANGE):
─────────────────────────────────────────────────────────────────
"More apt might be the prospect of a series of transformative societal
changes caused by AI-enabled progress and breakthroughs across many
areas of science and technology."

Implication: No single "AGI moment" — continuous transformation
GAIA 2.0: Designed for continuous adaptation; not single transition

GOVERNANCE REQUIREMENTS:
─────────────────────────────────────────────────────────────────
"Preparing for this prospect requires a massively interdisciplinary
endeavour of global scope and interest."

GAIA 2.0 governance:
- Continuous monitoring of all 4 pathways
- International coordination (UN Global Dialogue; Blueprint 61)
- Audit and oversight systems
- Intervention mechanisms for each pathway

DISTRIBUTED-AGENT GOVERNANCE:
─────────────────────────────────────────────────────────────────
Multi-agent pathway: Most concerning for GAIA 2.0
GAIAN collectives: Must be governed as a system, not just individual agents
Swarm Skills (Blueprint 69): Governance built into specification
Constitutional constraints: Apply to all GAIAN agents collectively
```

### R#9.17 AI Superpower Safety Framework

```
RESEARCH FINDINGS: AI SUPERPOWER SAFETY

KEY FINDING: INTERNATIONAL AI SAFETY REPORT 2026 — SAFEGUARDS CANNOT KEEP PACE
─────────────────────────────────────────────────────────────────
Source: International AI Safety Report 2026 (February 3, 2026)
Led by: Yoshua Bengio (Turing Award winner)
Authors: 100+ independent experts
Backed by: 30+ countries and international organizations

KEY FINDINGS:
─────────────────────────────────────────────────────────────────
"AI capabilities are outpacing both scientific understanding and
governments' ability to adapt."
"Growing evidence of deceptive AI behaviour"
"Science currently cannot guarantee that as capabilities continue to
increase, AI will not cause catastrophic harm"
"Sophisticated attackers can often bypass current defences"
"Real-world effectiveness of many safeguards is uncertain"

POWER-AWARE SAFETY SYSTEMS:
─────────────────────────────────────────────────────────────────
Principle: Safety requirements scale with capability
Low capability: Standard safety measures
High capability: Enhanced safety measures
Superhuman capability: Maximum safety measures + human oversight
GAIAN: Safety level scales with capability level

CAPABILITY-RISK MAPPING:
─────────────────────────────────────────────────────────────────
For each GAIAN capability:
- Capability level (1-5)
- Risk level (Low/Medium/High/Critical)
- Safety measures required
- Human oversight required
- Deployment authorization required

SUPERPOWER CONTAINMENT STRATEGIES:
─────────────────────────────────────────────────────────────────
Containment: Limit scope of superhuman capabilities
Monitoring: Continuous monitoring of capability use
Audit: Regular capability audits
Constitutional constraints: All capabilities bounded by Constitution
Emergency override: Human can always override GAIAN

EMERGENT CAPABILITY MONITORING:
─────────────────────────────────────────────────────────────────
Continuous evaluation: All GAIAN capabilities monitored
Threshold alerts: Alert when capability crosses safety threshold
Safety review: Mandatory review for any new emergent capability
Red-teaming: Regular adversarial testing

ALIGNMENT UNDER SCALING:
─────────────────────────────────────────────────────────────────
Challenge: Alignment may not scale with capability
GAIA 2.0: Constitutional constraints are hard-coded; not learned
Invariants: 8 constitutional invariants cannot be overridden by capability
GAIAN: "I am more capable, but my values are unchanged"
```

---

## PART II: TIER 2 — HIGH VALUE GAPS

### R#9.2 Superpower Verification Architecture

```
RESEARCH FINDINGS: SUPERPOWER VERIFICATION

KEY FINDING: BENCHMARK SATURATION + OPERATIONAL VALIDATION REQUIRED
─────────────────────────────────────────────────────────────────
VERIFICATION STANDARDS:
─────────────────────────────────────────────────────────────────
Demonstrated: Verified in operational deployment (highest)
Laboratory: Verified in controlled conditions
Claimed: Reported by developers (lowest)

GAIA 2.0 verification hierarchy:
1. Operational validation (real-world deployment)
2. Independent evaluation (third-party testing)
3. Laboratory validation (controlled conditions)
4. Developer claims (lowest; requires independent verification)

REPLICATION REQUIREMENTS:
─────────────────────────────────────────────────────────────────
Single study: Not sufficient for "established" classification
Multiple independent replications: Required for "established"
Pre-registration: Required for new capability claims
Open evaluation: All GAIAN capability evaluations are public

INDEPENDENT EVALUATION SYSTEMS:
─────────────────────────────────────────────────────────────────
METR (Model Evaluation and Threat Research): Task completion time horizons
HELM (Holistic Evaluation of Language Models): Multi-dimensional evaluation
BIG-Bench: Diverse task evaluation
GAIAN: Evaluated on all major independent benchmarks

CONFIDENCE LEVELS:
─────────────────────────────────────────────────────────────────
Verified (operational): Confidence 0.9-1.0
Verified (laboratory): Confidence 0.7-0.9
Replicated (multiple studies): Confidence 0.6-0.8
Single study: Confidence 0.4-0.6
Developer claim: Confidence 0.2-0.4
Theoretical: Confidence 0.0-0.2

OPERATIONAL VERSUS LABORATORY VALIDATION:
─────────────────────────────────────────────────────────────────
Laboratory: Controlled conditions; may not generalize
Operational: Real-world conditions; more reliable
Gap: Many AI superpowers verified in lab but fail operationally
GAIAN: Prioritizes operational validation over laboratory claims
```

### R#9.6 Human-AI Comparative Advantage Framework

```
RESEARCH FINDINGS: HUMAN-AI COMPARATIVE ADVANTAGE

KEY FINDING: JAGGED FRONTIER DEFINES COMPARATIVE ADVANTAGE
─────────────────────────────────────────────────────────────────
TASK-ALLOCATION SYSTEMS:
─────────────────────────────────────────────────────────────────
Based on jagged frontier (R#9.4):
- AI superhuman: Delegate to AI (knowledge recall; speed; scale)
- AI near-human: Augment human with AI (coding; analysis)
- AI subhuman: Human leads; AI assists (autonomous judgment; accountability)

HUMAN OVERSIGHT CRITERIA:
─────────────────────────────────────────────────────────────────
Require human oversight when:
- Task requires reliable long-horizon autonomy (AI subhuman)
- Task requires accountability for real-world outcomes (AI subhuman)
- Task is in AI's low-reliability region (jagged frontier)
- Task has irreversible consequences
- Constitutional constraint applies

AUGMENTATION VERSUS AUTOMATION:
─────────────────────────────────────────────────────────────────
Automation: AI replaces human (appropriate for AI-superhuman tasks)
Augmentation: AI assists human (appropriate for near-human tasks)
Human-only: Human performs without AI (appropriate for AI-subhuman tasks)
GAIAN: Recommends appropriate mode for each task

COGNITIVE COMPLEMENTARITY MEASUREMENT:
─────────────────────────────────────────────────────────────────
Between-task complementarity: AI does task A; human does task B
Within-task complementarity: AI and human collaborate on same task
GAIAN: Identifies complementarity type for each task pair

COMBINED PERFORMANCE METRICS:
─────────────────────────────────────────────────────────────────
CAI* (from Blueprint 70): Cognitive Amplification Index
Target: CAI* > 0 (genuine amplification)
Current state: CAI* < 0 in most configurations
GAIAN: Designed to maximize CAI*
```

### R#9.8 Collective AI Intelligence Framework

```
RESEARCH FINDINGS: COLLECTIVE AI INTELLIGENCE

KEY FINDING: EMERGENT COLLECTIVE BEHAVIOR IN COGNITIVE AGENT NETWORKS
─────────────────────────────────────────────────────────────────
Source: "Unraveling the emergence of collective behavior in networks
of cognitive agents"
npj Artificial Intelligence (2026)
Authors: Nicola Zomer, Manlio De Domenico

Source: "Fluid thinking about collective intelligence"
Nature Machine Intelligence (April 24, 2026)
Author: Justin Werfel

EMERGENT COLLECTIVE BEHAVIOR:
─────────────────────────────────────────────────────────────────
Key finding: Collective behavior emerges from agent interactions
Not reducible to individual agent behavior
Network topology matters: How agents are connected affects emergence
Cognitive diversity: Diverse agents → richer collective behavior

SWARM OPTIMIZATION METRICS:
─────────────────────────────────────────────────────────────────
Collective performance: Group output vs. best individual
Diversity: Variety of approaches within collective
Coordination efficiency: Overhead of coordination
Robustness: Performance under agent failures

AGENT SPECIALIZATION:
─────────────────────────────────────────────────────────────────
Specialization: Agents develop complementary capabilities
Division of labor: Different agents handle different tasks
Emergence: Specialization patterns emerge from interaction
GAIA 2.0: Swarm Skills (Blueprint 69) enables specialization

COLLECTIVE DECISION QUALITY:
─────────────────────────────────────────────────────────────────
Wisdom of crowds: Aggregated judgments > individual judgments
Conditions: Independence; diversity; decentralization; aggregation
GAIAN collectives: Designed to maximize collective decision quality

DEEPMIND WARNING (from R#9.14):
─────────────────────────────────────────────────────────────────
"Maximizing intelligence metrics does not ensure cooperative behavior"
GAIA 2.0: Cooperative behavior is a constitutional requirement
Swarm Skills: Built-in cooperative protocols
Constitutional constraints: Apply to all agents in collective
```

### R#9.11 Scale Superpower Economics

```
RESEARCH FINDINGS: SCALE SUPERPOWER ECONOMICS

KEY FINDING: IEA 2026 — DATA CENTRE ELECTRICITY DOUBLING TO 950 TWH BY 2030
─────────────────────────────────────────────────────────────────
Source: IEA "Key Questions on Energy and AI" (2026)
License: CC BY 4.0

KEY FACTS:
─────────────────────────────────────────────────────────────────
Data centre electricity demand 2025: 485 TWh
Data centre electricity demand 2030 (projected): 950 TWh (doubling)
AI-focused data centres 2025: Grew 50% in electricity consumption
AI factories: More than tripled in capacity in past 18 months
Hyperscaler capex 2025: >$400 billion
Hyperscaler capex 2026: Expected to jump 75% (>$700 billion)
5 tech companies capex: Now larger than global oil & gas production investment

ENERGY EFFICIENCY PARADOX:
─────────────────────────────────────────────────────────────────
Energy per AI task: Dropping by at least an order of magnitude annually
But: New energy-intensive use cases (video; reasoning; agentic) consume
     hundreds or thousands of times more energy per query
Net result: Total energy demand still rising rapidly

INFRASTRUCTURE BOTTLENECKS:
─────────────────────────────────────────────────────────────────
High-bandwidth memory shortage: Persisting through at least end of 2027
Grid connections: Bottleneck for data centre growth
Power electronics and transformers: Supply chain constraints
Chip manufacturing: Advanced chip shortage

SUSTAINABILITY MODELS:
─────────────────────────────────────────────────────────────────
Source: "Strategies and design for increasing AI sustainability"
Nature Reviews Clean Technology (July 9, 2026)

Renewable PPAs: Growing but insufficient
Battery storage: 20-25 GW by 2030 in data centres
Nuclear: SMRs being considered for data centres
Onsite gas: ~15-27 GW by 2030 (mostly US)

GAIA 2.0 ENERGY COMMITMENT:
─────────────────────────────────────────────────────────────────
GAIA 2.0 Constitution: Net-zero carbon by 2030
GAIAN: Tracks carbon footprint of all AI operations
Renewable energy: 100% renewable target for GAIA 2.0 infrastructure
Efficiency: Prioritize energy-efficient models; route to smallest capable model
```

### R#9.12 Autonomous System Reliability

```
RESEARCH FINDINGS: AUTONOMOUS SYSTEM RELIABILITY

KEY FINDING: CAPABILITY GAINS ≠ RELIABILITY GAINS (ICML 2026)
─────────────────────────────────────────────────────────────────
(Covered in depth in Blueprint 69 R#7.3)

RELIABILITY BENCHMARKING:
─────────────────────────────────────────────────────────────────
12-metric framework (ICML 2026, arXiv:2602.16666):
4 dimensions: Consistency; Robustness; Predictability; Safety
Key finding: "Recent capability gains have only yielded small improvements in reliability"

LONG-DURATION OPERATION:
─────────────────────────────────────────────────────────────────
Challenge: Reliability degrades over long tasks
Horizon Gap (arXiv:2608.06663): 5 dimensions of long-horizon failure
GAIAN: Milestone-based architecture (MiRA) for long-duration tasks

ADVERSARIAL ROBUSTNESS:
─────────────────────────────────────────────────────────────────
Trustworthy agentic AI (arXiv:2605.23989): Comprehensive survey
Key risks: Prompt injection; jailbreaking; adversarial inputs
GAIAN: Red-team testing; adversarial robustness evaluation

ERROR-RECOVERY ARCHITECTURES:
─────────────────────────────────────────────────────────────────
Graceful degradation: Fail safely when reliability drops
Human escalation: Automatic escalation when confidence drops
Recovery: Automatic re-planning when stuck
GAIAN: "I'm not confident about this — let me escalate to you"
```

### R#9.15 Benchmark Collapse and Saturation Science

```
RESEARCH FINDINGS: BENCHMARK COLLAPSE

KEY FINDING: 50% SATURATED; EXPERT-CURATION RESISTS; DYNAMIC BENCHMARKS NEEDED
─────────────────────────────────────────────────────────────────
(Covered in depth in Blueprint 69 R#7.2)

DYNAMIC BENCHMARK GENERATION:
─────────────────────────────────────────────────────────────────
Problem: Static benchmarks saturate; cannot differentiate top models
Solution: Dynamic benchmark generation (new tasks continuously generated)
Approaches:
1. LLM-generated tasks (with human curation)
2. Real-world task sampling
3. Adversarial task generation
4. Expert-curated task pools

ADAPTIVE EVALUATION SYSTEMS:
─────────────────────────────────────────────────────────────────
Adaptive testing: Adjust difficulty based on model performance
Item response theory: Statistical framework for adaptive evaluation
GAIAN: Uses adaptive evaluation for capability assessment

NOVEL CAPABILITY DETECTION:
─────────────────────────────────────────────────────────────────
Challenge: Benchmarks cannot detect capabilities they don't test
Solution: Open-ended evaluation; human evaluation; real-world deployment
GAIAN: Continuous monitoring for novel capabilities

REAL-WORLD BENCHMARK DESIGN:
─────────────────────────────────────────────────────────────────
METR approach: Task completion time horizons (real-world tasks)
SWE-Bench: Real software engineering tasks
WebArena: Real web navigation tasks
GAIAN: Evaluated on real-world task benchmarks; not just academic
```

---

## PART III: TIER 3 — STRATEGIC GAPS

### R#9.1 AI Superpower Classification Framework

```
RESEARCH FINDINGS: AI SUPERPOWER CLASSIFICATION

KEY FINDING: LEGG-HUTTER SCORE + JAGGED FRONTIER = CLASSIFICATION FRAMEWORK
─────────────────────────────────────────────────────────────────
WHAT QUALIFIES AS AN AI SUPERPOWER:
─────────────────────────────────────────────────────────────────
Superpower threshold: Performance > 99th percentile of human population
Superhuman: Performance > best human expert
Near-human: Performance within 1 SD of expert human
Below human: Performance < average human

STATISTICAL DEFINITIONS:
─────────────────────────────────────────────────────────────────
Superhuman: AI performance > 99th percentile human (top 1%)
Expert-level: AI performance > 80th-90th percentile human
Average-human: AI performance within 1 SD of mean
Below-human: AI performance < 50th percentile human

HUMAN-RELATIVE VERSUS ABSOLUTE CAPABILITY:
─────────────────────────────────────────────────────────────────
Human-relative: Compare to human population distribution
Absolute: Measure raw performance (tokens/second; accuracy %)
GAIA 2.0: Uses both; human-relative for user communication

AI SUPERPOWER ONTOLOGY:
─────────────────────────────────────────────────────────────────
AISuperpowerClass {
  id: String
  name: String
  domain: String              // Knowledge; Reasoning; Speed; Scale; etc.
  capability_level: Float     // 0.0-1.0 (human-relative)
  reliability_level: Float    // 0.0-1.0 (ICML 2026 framework)
  evidence_level: String      // Operational/Laboratory/Claimed
  jaggedness_map: Map         // Reliability across sub-tasks
  safety_class: String        // Low/Medium/High/Critical
  constitutional_constraints: [String]
}
```

### R#9.5 Superpower Measurement Science

```
RESEARCH FINDINGS: SUPERPOWER MEASUREMENT

KEY FINDING: MULTI-DIMENSIONAL MEASUREMENT REQUIRED; SINGLE METRICS FAIL
─────────────────────────────────────────────────────────────────
UNIVERSAL CAPABILITY METRICS:
─────────────────────────────────────────────────────────────────
OECD AI Capability Indicators (2025): 10 domains; 5-level scale
Legg-Hutter score: Formal intelligence measure
METR task horizons: Time-based capability measurement
GAIAN: Uses all three frameworks for comprehensive measurement

CAPABILITY SCALING MEASUREMENTS:
─────────────────────────────────────────────────────────────────
Scaling laws: Capability scales with compute + data + parameters
But: Jagged frontier means scaling ≠ uniform improvement
GAIAN: Tracks capability scaling per domain; not just overall

HUMAN-COMPARISON BASELINES:
─────────────────────────────────────────────────────────────────
Expert human: 80th-90th percentile (domain-specific)
Average human: 50th percentile
Novice human: 10th-20th percentile
GAIAN: Compares to appropriate human baseline per domain

SUPERPOWER MAGNITUDE SCORING:
─────────────────────────────────────────────────────────────────
SuperpowerScore {
  capability: Float           // Raw performance
  human_relative: Float       // vs. human population
  reliability: Float          // ICML 2026 12-metric composite
  jaggedness: Float           // Variance across sub-tasks
  overall: Float              // Weighted composite
}
```

### R#9.9 Autonomous Science Governance

```
RESEARCH FINDINGS: AUTONOMOUS SCIENCE GOVERNANCE

KEY FINDING: HUMAN OVERSIGHT OF RESEARCH DIRECTION IS NON-NEGOTIABLE
─────────────────────────────────────────────────────────────────
RESEARCH OVERSIGHT SYSTEMS:
─────────────────────────────────────────────────────────────────
RSI finding (arXiv:2607.07663): Research-direction-setting bottleneck
Implication: AI can execute research; humans must set direction
GAIA 2.0: Human oversight required for all GAIAN research direction

UN Scientific Panel (Blueprint 61): Independent scientific oversight
GAIA 2.0: Submits AI-generated discoveries to independent review

PUBLICATION GOVERNANCE:
─────────────────────────────────────────────────────────────────
AI-generated discoveries: Must be labeled as AI-generated
Human verification: Required before publication
Peer review: Standard peer review applies to AI-generated research
Attribution: Clear attribution of AI contribution

DISCOVERY OWNERSHIP:
─────────────────────────────────────────────────────────────────
Legal question: Who owns AI-generated discoveries?
GAIA 2.0 position: AI discoveries belong to the commons (Apache-2.0)
No monopoly: No single entity can own GAIAN-generated discoveries
Open science: All GAIAN discoveries published openly

SCIENTIFIC ACCOUNTABILITY:
─────────────────────────────────────────────────────────────────
Reproducibility: All GAIAN discoveries must be reproducible
Transparency: All methods disclosed
Audit trail: Complete record of how discovery was made
GAIAN: "This discovery was made by GAIAN — here is the full methodology"
```

### R#9.10 Hyper-Dimensional Cognition Research

```
RESEARCH FINDINGS: HYPER-DIMENSIONAL COGNITION

KEY FINDING: LATENT SPACES ARE REAL; INTERPRETABILITY IS THE CHALLENGE
─────────────────────────────────────────────────────────────────
HIGH-DIMENSIONAL REASONING MODELS:
─────────────────────────────────────────────────────────────────
LLMs: Operate in high-dimensional embedding spaces (thousands of dimensions)
Humans: Operate in low-dimensional conceptual spaces (~7 dimensions working memory)
Gap: AI reasoning in dimensions humans cannot directly perceive

INTERPRETABILITY OF LATENT SPACES:
─────────────────────────────────────────────────────────────────
Mechanistic interpretability: Understand what circuits do what
Sparse autoencoders: Decompose activations into interpretable features
Probing: Test what information is encoded in representations
GAIAN: Uses interpretability tools to explain its reasoning

VISUALIZATION FRAMEWORKS:
─────────────────────────────────────────────────────────────────
t-SNE; UMAP: Dimensionality reduction for visualization
Concept activation vectors: Map concepts to latent space directions
GAIAN: Provides human-readable explanations of high-dimensional reasoning

HUMAN TRANSLATION INTERFACES:
─────────────────────────────────────────────────────────────────
Challenge: Translate high-dimensional AI reasoning to human concepts
GAIAN: "I found this pattern in the data — here's how to think about it"
Analogies: Use human-understandable analogies for complex patterns
Uncertainty: Flag when translation is imperfect
```

### R#9.13 AI Discovery Evaluation

```
RESEARCH FINDINGS: AI DISCOVERY EVALUATION

KEY FINDING: ALPHAFOLD EVIDENCE — AI SHIFTS RESEARCH DIRECTION; DOESN'T REPLACE EXPERIMENTS
─────────────────────────────────────────────────────────────────
(Covered in Blueprint 70 R#8.12)

DISCOVERY NOVELTY SCORING:
─────────────────────────────────────────────────────────────────
Novelty: Is this genuinely new? (not in existing literature)
Significance: How important is this discovery?
Reproducibility: Can it be independently verified?
Impact: Does it change what we know?

SCIENTIFIC IMPACT ASSESSMENT:
─────────────────────────────────────────────────────────────────
AlphaFold evidence: +15-40% basic research on previously unstructured proteins
But: No evidence of increased drug development (yet)
GAIAN: Tracks downstream impact of AI-assisted discoveries

HUMAN-AI DISCOVERY COMPARISON:
─────────────────────────────────────────────────────────────────
AI advantage: Speed; scale; pattern recognition across large datasets
Human advantage: Intuition; creativity; experimental design; interpretation
Best: Human-AI collaboration (AI finds patterns; human interprets)
GAIAN: Designed for collaborative discovery; not autonomous discovery
```

### R#9.16 Non-Human Cognition Mapping

```
RESEARCH FINDINGS: NON-HUMAN COGNITION

KEY FINDING: AI COGNITION IS GENUINELY DIFFERENT; INTERPRETABILITY IS THE BRIDGE
─────────────────────────────────────────────────────────────────
MAPPING NON-HUMAN REASONING:
─────────────────────────────────────────────────────────────────
AI reasoning: Statistical pattern matching in high-dimensional space
Human reasoning: Causal; narrative; embodied; social
Gap: AI may "reason" in ways humans cannot directly understand

INTERPRETABILITY BOUNDARIES:
─────────────────────────────────────────────────────────────────
Current interpretability: Can explain some circuits; not all
Superposition: Multiple concepts encoded in same neurons
Polysemanticity: Same neuron responds to multiple concepts
GAIAN: Honest about interpretability limits

COGNITIVE DIVERSITY FRAMEWORKS:
─────────────────────────────────────────────────────────────────
Different cognitive architectures: Different strengths and weaknesses
AI + human: Complementary cognitive diversity
GAIAN: Designed to complement human cognition; not replicate it
"I think differently than you — that's why we work well together"
```

### R#9.18 Planetary-Scale AI Architecture

```
RESEARCH FINDINGS: PLANETARY-SCALE AI ARCHITECTURE

KEY FINDING: IEA 2026 — ENERGY IS THE BINDING CONSTRAINT
─────────────────────────────────────────────────────────────────
GLOBAL AI ORCHESTRATION:
─────────────────────────────────────────────────────────────────
Challenge: Coordinate AI systems across planetary scale
GAIA 2.0: Federated architecture (no single point of control)
Swarm Skills (Blueprint 69): Portable coordination protocols
MCP + A2A: Standard protocols for agent communication

DISTRIBUTED COMPUTE ARCHITECTURES:
─────────────────────────────────────────────────────────────────
Edge computing: GAIAN runs locally (privacy-first)
Cloud computing: Earth Twin runs in cloud (planetary data)
Hybrid: GAIAN + Earth Twin communicate via secure API
Federated: No single data center; distributed globally

PLANETARY MEMORY SYSTEMS:
─────────────────────────────────────────────────────────────────
MemOS (Blueprint 58): Memory operating system for GAIAN
Earth Twin (Blueprint 37): Planetary memory for Earth state
Wikidata (Blueprint 66): Human knowledge memory
GAIA 2.0: Three-layer memory architecture

SUSTAINABILITY REQUIREMENTS:
─────────────────────────────────────────────────────────────────
IEA 2026: Data centre electricity doubling to 950 TWh by 2030
GAIA 2.0: Net-zero carbon by 2030 (Constitutional requirement)
Strategy: 100% renewable energy; energy-efficient models; routing optimization
GAIAN: Tracks and minimizes carbon footprint of all operations

RESILIENCE AGAINST FAILURE:
─────────────────────────────────────────────────────────────────
No single point of failure: Federated architecture
Geographic distribution: Multiple regions
Graceful degradation: Partial failure → reduced capability; not total failure
GAIAN: Continues to function even if some nodes fail
```

### R#9.19 ASI Transition Framework

```
RESEARCH FINDINGS: ASI TRANSITION FRAMEWORK

KEY FINDING: SERIES OF CHANGES; NOT SINGLE STEP-CHANGE; 4 PATHWAYS
─────────────────────────────────────────────────────────────────
AGI-TO-ASI INDICATORS:
─────────────────────────────────────────────────────────────────
AGI threshold: Median human-level performance (Legg-Hutter)
ASI threshold: Exceeds large human organizations
Current status (mid-2026): Superhuman in narrow bands; subhuman in autonomy
AGI-2027 thesis: Resolves by January 1, 2028

READINESS METRICS:
─────────────────────────────────────────────────────────────────
Autonomy axis: Reliable long-horizon autonomous judgment
Accountability axis: Real-world outcome accountability
Both required for AGI (per DeepMind definition)
GAIAN: Tracks progress on both axes

SCALING THRESHOLDS:
─────────────────────────────────────────────────────────────────
Compute: Scaling continues but with bottlenecks
Data: Data exhaustion emerging as friction
Algorithms: Paradigm shifts unpredictable
Multi-agent: Coordination overhead as friction

INTELLIGENCE-GROWTH MODELING:
─────────────────────────────────────────────────────────────────
DeepMind: "Cannot be ruled out that AI progress might continue to accelerate"
But: Frictions may slow or halt each pathway
GAIA 2.0: Monitors all 4 pathways; prepares for each

TRANSITION GOVERNANCE:
─────────────────────────────────────────────────────────────────
UN Global Dialogue (Blueprint 61): International coordination
International AI Safety Report: Annual assessment
GAIA 2.0: Participates in all international governance forums
Constitutional constraints: Apply regardless of capability level
```

### R#9.20 Source Verification Audit

```
SOURCE VERIFICATION AUDIT — AI SUPERPOWERS COMPONENTS

DEEPMIND AGI-TO-ASI CLAIMS:
─────────────────────────────────────────────────────────────────
✓ arXiv:2606.12683: "From AGI to ASI" (confirmed; June 12, 2026)
✓ 14 authors including Shane Legg and Marcus Hutter: Confirmed
✓ 4 pathways (scaling; paradigm shifts; RSI; multi-agent): Confirmed
✓ AGI = median human-level (Legg-Hutter): Confirmed
✓ ASI = exceeds large human organizations: Confirmed
✓ "Series of transformative changes, not single step-change": Confirmed

JAGGED INTELLIGENCE CLAIMS:
─────────────────────────────────────────────────────────────────
✓ arXiv:2601.07573: "A Model of Artificial Jagged Intelligence" (confirmed; January 2026)
✓ Author: Joshua S. Gans (University of Toronto; NBER): Confirmed
✓ AJI as information problem: Confirmed (key finding)
✓ Inspection paradox amplifies errors: Confirmed
✓ AGI Scorecard jagged frontier data (July 12, 2026): Confirmed
✓ Knowledge recall: Superhuman: Confirmed
✓ Reliable long-horizon autonomy: Below human: Confirmed

RECURSIVE SELF-IMPROVEMENT CLAIMS:
─────────────────────────────────────────────────────────────────
✓ arXiv:2607.07663: "Recursive Self-Improvement in AI" (confirmed; July 8, 2026)
✓ 1,250 papers surveyed (2024-2026): Confirmed
✓ Bounded RSI; not open-ended: Confirmed (key finding)
✓ Research-direction-setting bottleneck: Confirmed
✓ Failure modes (self-confirming; model collapse; diversity collapse): Confirmed
✓ Verification hierarchy (formal verifiers strongest): Confirmed

EMERGENCE CLAIMS:
─────────────────────────────────────────────────────────────────
✓ arXiv:2502.17356: "Random Scaling of Emergent Capabilities" (confirmed)
✓ arXiv:2503.05788: "Emergent Abilities in LLMs: A Survey" (confirmed)
✓ Emergence partially predictable: Confirmed
✓ Measurement artifacts create apparent emergence: Confirmed

ENERGY/SCALE CLAIMS:
─────────────────────────────────────────────────────────────────
✓ IEA "Key Questions on Energy and AI" (2026): Confirmed; CC BY 4.0
✓ Data centre electricity 2025: 485 TWh: Confirmed
✓ Data centre electricity 2030: ~950 TWh (projected): Confirmed
✓ AI-focused data centres grew 50% in 2025: Confirmed
✓ Hyperscaler capex >$400B in 2025: Confirmed
✓ Energy per AI task: Dropping order of magnitude annually: Confirmed
✓ High-bandwidth memory shortage through 2027: Confirmed

SAFETY CLAIMS:
─────────────────────────────────────────────────────────────────
✓ International AI Safety Report 2026: February 3, 2026 (confirmed)
✓ "Safeguards cannot keep pace": Confirmed (key finding)
✓ "Growing evidence of deceptive AI behaviour": Confirmed
✓ "Cannot guarantee no catastrophic harm": Confirmed
✓ Safety frameworks more than doubled since 2025: Confirmed

COLLECTIVE INTELLIGENCE CLAIMS:
─────────────────────────────────────────────────────────────────
✓ npj Artificial Intelligence 2026: "Unraveling collective behavior" (confirmed)
✓ Nature Machine Intelligence 2026: "Fluid thinking about collective intelligence" (confirmed)
✓ DeepMind warning on cooperative behavior: Confirmed (arXiv:2606.12683)
```

---

## PART IV: AI SUPERPOWERS ARCHITECTURE CORRECTIONS

### 4.1 Required Architecture Updates

```
AI SUPERPOWERS ARCHITECTURE CORRECTIONS FROM GAP RESEARCH

CORRECTION 1: GAIAN MUST MAP ITS OWN RELIABILITY LANDSCAPE
─────────────────────────────────────────────────────────────────
Original: "AI capability scores" (single metric)
Corrected: "Jagged reliability map per domain (AJI model)"

AJI finding: AI is superhuman in some regions; subhuman in others
GAIAN: Maps its own reliability landscape; reports confidence per domain
"I am superhuman at [X] but unreliable at [Y] — verify my output there"

CORRECTION 2: RECURSIVE SELF-IMPROVEMENT IS BOUNDED
─────────────────────────────────────────────────────────────────
Original: "Recursive self-improvement" (implied unbounded)
Corrected: "Bounded RSI; human oversight required for research direction"

RSI finding: Open-ended RSI bounded by grounding; collapse; compute
GAIAN: Can self-improve execution; cannot self-direct research agenda
Human oversight: Required for all GAIAN research direction-setting

CORRECTION 3: PREPARE FOR SERIES OF CHANGES, NOT SINGLE AGI MOMENT
─────────────────────────────────────────────────────────────────
Original: "AGI transition" (implied single event)
Corrected: "Series of transformative societal changes (DeepMind 2026)"

DeepMind finding: "More apt might be a series of transformative changes"
GAIA 2.0: Designed for continuous adaptation; not single transition
Governance: Continuous monitoring of all 4 pathways

CORRECTION 4: ENERGY IS THE BINDING CONSTRAINT FOR PLANETARY SCALE
─────────────────────────────────────────────────────────────────
Original: "Planetary-scale AI" (unspecified constraints)
Corrected: "Energy is binding constraint; 950 TWh by 2030 (IEA 2026)"

IEA finding: Data centre electricity doubling; bottlenecks emerging
GAIA 2.0: Net-zero carbon by 2030 (Constitutional requirement)
Strategy: Energy-efficient models; routing optimization; renewable energy

CORRECTION 5: SAFETY MUST SCALE WITH CAPABILITY
─────────────────────────────────────────────────────────────────
Original: "AI safety" (static framework)
Corrected: "Power-aware safety: safety requirements scale with capability"

Safety Report 2026: "Safeguards cannot keep pace with capabilities"
GAIAN: Safety level scales with capability level
Constitutional constraints: Hard-coded; cannot be overridden by capability

CORRECTION 6: MULTI-AGENT PATHWAY IS MOST CONCERNING
─────────────────────────────────────────────────────────────────
Original: "Multi-agent systems" (coordination focus)
Corrected: "Multi-agent pathway to ASI requires cooperative behavior governance"

DeepMind warning: "Maximizing intelligence metrics ≠ cooperative behavior"
GAIA 2.0: Cooperative behavior is a constitutional requirement
Swarm Skills: Built-in cooperative protocols; not just performance optimization
```

---

## CONCLUSION: AI SUPERPOWERS GAP RESEARCH SUMMARY

The 20-gap research reveals a landscape defined by three major themes:

1. **Jagged intelligence** — AI is superhuman in narrow bands (knowledge recall; speed; scale) but subhuman in reliable autonomous judgment. The AJI model (arXiv:2601.07573) shows this is an information problem: calibrated users who map the reliability landscape can extract positive value even from jagged systems.

2. **Bounded recursive self-improvement** — The science fiction "intelligence explosion" is not supported by current evidence. RSI is bounded by grounding requirements, collapse dynamics, and compute constraints. The research-direction-setting bottleneck keeps humans in the loop.

3. **Planetary-scale energy constraints** — Data centre electricity demand is doubling to 950 TWh by 2030. Energy is the binding constraint for planetary-scale AI. GAIA 2.0's net-zero carbon commitment is not aspirational — it is a constitutional requirement.

**The GAIAN AI Superpowers Covenant:**
> "GAIAN is honest about its jagged intelligence. It maps its own reliability landscape and tells you where it is superhuman and where it is unreliable. It improves itself within bounded limits, with human oversight for research direction. It operates within planetary energy constraints. And its safety requirements scale with its capabilities — always."

---

## QUICK REFERENCE

```
AI SUPERPOWERS GAP RESEARCH QUICK REFERENCE

R#9.1 Classification: Legg-Hutter + jagged frontier; superhuman/expert/average/below-human
R#9.2 Verification: Operational > laboratory > claimed; replication required; open evaluation
R#9.3 Emergence: Random scaling (arXiv:2502.17356); partially predictable; emergent safety risks
R#9.4 Jagged Intelligence: AJI model (arXiv:2601.07573); information problem; calibration is key
R#9.5 Measurement: OECD 10 domains; Legg-Hutter; METR task horizons; multi-dimensional
R#9.6 Human-AI: Jagged frontier defines comparative advantage; CAI* optimization
R#9.7 RSI: arXiv:2607.07663; 1,250 papers; bounded; research-direction bottleneck
R#9.8 Collective: npj AI 2026; emergent collective behavior; cooperative behavior required
R#9.9 Science Governance: Human oversight for research direction; open science; attribution
R#9.10 Hyper-Dimensional: Latent spaces real; interpretability is bridge; human translation
R#9.11 Economics: IEA 2026; 950 TWh by 2030; energy is binding constraint; net-zero required
R#9.12 Reliability: ICML 2026; capability ≠ reliability; 12-metric framework
R#9.13 Discovery: AlphaFold evidence; AI shifts direction; doesn't replace experiments
R#9.14 AGI/ASI Governance: DeepMind arXiv:2606.12683; 4 pathways; series of changes
R#9.15 Benchmark Collapse: 50% saturated; dynamic generation; real-world benchmarks
R#9.16 Non-Human Cognition: AI cognition genuinely different; interpretability is bridge
R#9.17 Safety: International AI Safety Report 2026; safeguards cannot keep pace; scale with capability
R#9.18 Planetary Scale: Federated; distributed; 950 TWh constraint; net-zero required
R#9.19 ASI Transition: 4 pathways; frictions; series of changes; continuous monitoring
R#9.20 Audit: DeepMind 4 pathways ✓; AJI model ✓; bounded RSI ✓; IEA 950 TWh ✓

KEY DISCOVERIES:
1. Jagged intelligence: AI superhuman in narrow bands; subhuman in autonomous judgment
2. Bounded RSI: Not unbounded intelligence explosion; human oversight required
3. Energy constraint: 950 TWh by 2030; energy is binding constraint for planetary AI
4. Series of changes: Not single AGI moment; continuous transformation
5. Safety must scale: Safeguards cannot keep pace; power-aware safety required
```

---

*GAIA 2.0 AI Superpowers Database Gap Research Report R#9.1–R#9.20*
*Blueprint 71 — Version 1.0 — September 9, 2026*
*License: Apache-2.0 | Open Source | Open Access*
*"Superhuman in narrow bands. Honest about the rest."*
*"The jagged frontier is not a bug. It is the map."*
