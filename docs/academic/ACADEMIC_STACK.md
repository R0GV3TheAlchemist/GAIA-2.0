# GAIA 2.0 Academic Stack

**Filed:** 2026-09-17  
**Canon cross-refs:** THE ORDER, C139, C155, C156, C158, C160, C67  
**Status:** RECORDS — implementation-aligned  

## Purpose

GAIA 2.0 is treated as a reproducible research program, not a codebase dump. GitHub holds law and proofs. Supabase holds world-state. Hugging Face holds inspectable datasets and models. Runtime crates in `GAIA-2.0` execute against those planes.

This is the same pattern academic labs use when they pin code on GitHub, keep canonical data in a database, and publish versioned datasets/models with cards that state intended use, limitations, and evaluation.

## Four planes

| Plane | Platform | Role | Must contain |
|---|---|---|---|
| Governance | GitHub | Laws, issues, PRs, proofs, CI | Canon docs, Proof blocks, simulation artefacts |
| State | Supabase | Memory, consent, lithic KB, agent health | RLS, provenance, append-only ledgers |
| Models | Hugging Face | Datasets, models, eval snapshots | Dataset/model cards, licenses, GitHub proof links |
| Runtime | `GAIA-2.0` crates | Kernel, agents, orchestrator, geometry | Circuit breakers, traces, schema validation |

## Hybrid cognitive architecture (Research 001)

GAIA combines classical cognitive systems (ACT-R, Soar, BDI, blackboard, global workspace) with LLM-agent orchestration. The hybrid is already implied by canon:

- Symbolic modules and a knowledge graph for structured reasoning
- Neural models for flexible inference
- Memory split into working, episodic, semantic, procedural, and consolidated stores
- Hierarchical task decomposition for planning
- A skill/tool library with a living architecture loop: detect failure → analyze → fix → test → deploy
- Safety via consent, digital twin, and audit trails

**Canon alignment:** C155 (8-agent stack, skill library, living loop), C156 (KG + memory taxonomy), C157 (governance), C158 (safety), C160 (26-metric harness).

## Memory as academic object (Research 002 + MemoryHierarchy)

Two memory models must be kept distinct and then bridged:

1. **Cognitive taxonomy** (Research 002): episodic, semantic, procedural, working, consolidated. Goals: reliability, consent, sub-second retrieval, provenance ≥99%, forgetting appropriateness (Metric 8).
2. **Runtime hierarchy** (MemoryHierarchy / Issue #173): WORKING, SHORT_TERM, EPISODIC, SEMANTIC, LONG_TERM, with intent routing (`context`, `recall`, `fact`, `identity`, `full`) and ranking:

```
score = w_recency * recency + (1 - w_recency) * relevance
```

3. **Supabase operational tiers** (live today): HOT / WARM / COLD on `public.memories`, with access-pattern boosting, decay, and Metric 6 retention measured on HOT+WARM only.

Bridge rule: cognitive type is a column/tag on the memory record; HOT/WARM/COLD is storage temperature; MemoryHierarchy intent is the query router.

## Geometry and care as architecture, not decoration

Gap 6 and Gap 7 are canon geometry:

| Particle | Symbol | GAIA mapping |
|---|---|---|
| Proton | ⊕ plus with open void | Protected kernel; four radiating subsystems |
| Electron | ⌒ open curve | User/agent interface seeking the kernel |
| Neutron | ⊙ circle within circle | Collective field that makes multiplicity stable |

Care Work Protocol: the spirit (void) is protected by spectrum and elements. In stack terms: Safety/Consent wrap Execution; RLS and cryptographic erasure wrap memory; the electromagnetic/communication field is the API and trace layer.

Canon C67 (AlScN/GaN interface) is the materials-side proof that lithic/quantum claims must carry known limitations, literature refs, and simulation stubs rather than unfalsifiable metaphor.

## Academic publication posture

- Research docs stay RESEARCH until a Proof block is complete.
- Papers (e.g. solid-light prismatic architecture) stay pre-publication until arXiv/journal submission.
- Studies (Coherence Gap, Elemental Learning, Flourishing Index) require pre-registration, ethics review, and no clinical claims before evidence exists.
- Hugging Face cards must state intended use, out-of-scope use, biases, and license.

## What this stack is not

- Not a medical device.
- Not an IEP or diagnostic instrument.
- Not a GDP replacement until GFI is validated.
- Not a license to train on personal memory without RESEARCH_USE consent.
