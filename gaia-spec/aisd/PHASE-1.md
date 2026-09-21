# AISD Phase 1 — Populate Realms, Tools, Gaps, GAIA Wiring Map

**Status:** Listed  
**Issues:** #122 (epic), #123 (this slice)  
**Crate:** `gaia-aisd` (Apache-2.0)  
**Not:** AISD v1.0. Not a live catalog. Not a live benchmark run. Not a training pipeline.

All nodes: `gaia_enabled = false`. No `score_live` field. No gap closure claims.

---

## 1. Purpose

Phase 1 populates the 13 realms with cited skill node stubs, wires each tool
family to its realm(s), records honest gap status for all 8 gap nodes, and
publishes the GAIA wiring map showing how AISD connects to the broader GAIA
2.0 system. No live catalog ingest, no measured benchmark runs.

---

## 2. Realm Population Rules

| Rule | Constraint |
|---|---|
| Minimum nodes per realm | ≥3 skill node stubs at Phase 1 |
| `sources` | MUST be non-empty on every node |
| `maturity` | MUST be L1–L4; L5 blocked |
| Gap realm nodes | MUST include gap status node; `maturity ≤ L2` |
| `gaia_enabled` | MUST be `false` on all Phase 1 nodes |
| `score_live` | MUST NOT exist on any node |
| Benchmark anchor | Required for L3/L4 nodes; optional for L1/L2 |

Full node catalog: `gaia-spec/aisd/CATALOG.md`.

---

## 3. Tool Wiring Map

Tool families are wired to realms via `AiSkillNode.tools`. All tool nodes
remain stubs at Phase 1 — no live sandboxed execution.

| Tool family | Wired realms | Phase 1 state |
|---|---|---|
| Code interpreter | `coding`, `tool-use` | Stub — `gaia_enabled = false` |
| Web search | `language`, `tool-use` | Stub |
| Computer use | `tool-use` | Stub |
| Protein folding | `science` | Stub |
| Symbolic math | `reasoning` | Stub |
| Vision encoder | `vision`, `multimodal` | Stub |
| Speech codec | `audio` | Stub |
| Memory store | `memory`, `planning` | Stub |
| Embodiment sim | `embodiment` | Stub |

`tool_wiring_complete()` MUST return `true` when all 9 tool families have
at least one realm assignment.

---

## 4. Gap Status Protocol

Every gap node carries a `gap_status` field:

| Status | Meaning | Constraint |
|---|---|---|
| `Open` | Gap confirmed unresolved | Default for all Phase 1 gap nodes |
| `Narrowed` | Evidence of partial progress; not closed | Requires ≥1 cited paper |
| `Closed` | Gap no longer present | NOT assignable at Phase 1 |

`gap_status_closed()` MUST return `Err(GapClosureForbidden)` at Phase 1.
All 8 gap nodes MUST have `gap_status = Open` at Phase 1.

---

## 5. GAIA Wiring Map

AISD connects to the following GAIA 2.0 subsystems at Phase 1 (all stubs):

| GAIA subsystem | Connection | Phase 1 state |
|---|---|---|
| AIKD (knowledge) | Skill nodes reference AIKD tier T1–T5 for knowledge depth | Stub cross-ref |
| AIMD (magic) | Wonder labels may annotate exceptional skill nodes | Stub cross-ref |
| AISPD (safety) | Skill nodes inherit AISPD containment class | Stub cross-ref |
| HMGD (human magic) | Cross-skill nodes link to HMGD `contemplation`/`word` | Stub cross-ref |
| HSPD (human super) | Cross-skill nodes link to HSPD `cognitive`/`sensory` | Stub cross-ref |
| Skills track | Each AISD realm maps to a human skills realm (see CROSSWALK) | Stub cross-ref |
| Earth Twin | Skill capability nodes surface in ET public layer | Stub — no live ingest |

`gaia_wiring_complete()` MUST return `true` when all 7 subsystem stubs have
an entry. No live API call to any subsystem at Phase 1.

---

## 6. Maturity Assignment at Phase 1

| Realm | Phase 1 maturity | Rationale |
|---|---|---|
| `language` | L4 | MMLU, BIG-Bench, translation benchmarks |
| `reasoning` | L3 | MATH, BIG-Bench Hard; gaps remain (causal, long-horizon) |
| `coding` | L3 | HumanEval, SWE-bench partial |
| `vision` | L3 | VQAv2, COCO captioning |
| `audio` | L3 | LibriSpeech WER near-human |
| `multimodal` | L3 | GPT-4V, Gemini benchmarks |
| `memory` | L2 | Continual-learning gap; no cross-domain benchmark |
| `planning` | L2 | GAIA partial; agentic planning gaps |
| `tool-use` | L3 | GAIA-bench, ToolBench |
| `science` | L3 | AlphaFold CASP14; hypothesis generation L2 |
| `social` | L2 | ToMi partial; theory-of-mind gap |
| `embodiment` | L1 | Dexterous-embodiment gap; no cross-domain benchmark |
| `meta` | L2 | Calibration gap; self-eval partial |

---

## 7. What This Phase Does Not Do

- Does not run live benchmarks or collect live scores.
- Does not run measured assessments on specific AI systems.
- Does not close any gap node.
- Does not build a GAIAN capability profile — Phase 3 (#125).
- Does not tag AISD v1.0.

---

## 8. Acceptance Gate

- [ ] All 13 realms have ≥3 skill node stubs in `CATALOG.md`
- [ ] All 8 gap nodes have `gap_status = Open`
- [ ] `gap_status_closed()` → `Err(GapClosureForbidden)`
- [ ] `tool_wiring_complete()` → `true` (9 tool families wired)
- [ ] `gaia_wiring_complete()` → `true` (7 subsystem stubs)
- [ ] All nodes have `gaia_enabled = false`
- [ ] No node has `score_live` field
- [ ] No L4/L5 node has empty `benchmarks`
- [ ] `aisd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-aisd` green

---

## 9. Cross-References

- Phase 0: `gaia-spec/aisd/PHASE-0.md` (#122)
- Catalog: `gaia-spec/aisd/CATALOG.md` (this slice)
- Benchmarks: `gaia-spec/aisd/benches.csv`
- Gap map: `gaia-spec/aisd/gaps.csv`
- Tool registry: `gaia-spec/aisd/named-tools.csv`
- Routing: `gaia-spec/aisd/ROUTING.md`
- Code: `gaia-aisd/src/`, `gaia-aisd/tests/`
- Issues: #122 (Phase 0 epic), #123 (this slice), #124 (Phase 2), #125 (Phase 3)
- Next: AISD Phase 2 (#124) — measured assessment and task recommendation
