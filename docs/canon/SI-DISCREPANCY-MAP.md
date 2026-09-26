# SI language discrepancy map

**Date:** 2026-09-26  
**Standard:** `docs/canon/SUPER_INTELLIGENCE_LANGUAGE_STANDARD.md`  
**Prior close:** #865 / PR #964

This map is how we resolve leftovers without a blind `AI` → `SI` replace.

## Classes

| Class | Action |
| --- | --- |
| A — GAIA speaking about itself | Replace with Super Intelligence / SI / GAIA |
| B — Grandfathered token | Keep (`gaia-aikd`, paths, crate ids) |
| C — External title | Keep (UNESCO, Stanford AI Index, OpenAI, paper titles) |
| D — Product name | Keep unless a later issue retargets (`Artificial Twin`) |
| E — Frozen `Documents/` corpus | Do not rewrite; list only |

## This PR (class A, living files)

| File | Was | Now |
| --- | --- | --- |
| `docs/MASTER-CODEX.md` §1.2 | Global Artificial Intelligence Architecture; first AI system | Global Super Intelligence Architecture; first Super Intelligence system |
| `docs/MASTER-CODEX.md` tree / §1.3 / §1.5 / §2 | personal AI, AI advises, Role of AI, AI models | SI wording |
| `README.md` | field slogans "AI will save/destroy us" | SI slogans |

## Kept on purpose

| Location | Token | Class |
| --- | --- | --- |
| `gaia-aikd` `gaia-aimd` `gaia-aisd` `gaia-aispd` | crate prefix | B |
| `docs/knowledge/AI-KNOWLEDGE-DATABASE.md` | path | B |
| `docs/knowledge/catalog-v2.json` | subject title Artificial Intelligence Basics | C (curriculum name) |
| C77 UNESCO line | Recommendation on the Ethics of Artificial Intelligence (2021) | C |
| MASTER-CODEX §3.5 / §6.2 | AI Dividend | D (program name) |
| MASTER-CODEX §5.5 | Stanford AI Index 2026 | C |
| MASTER-CODEX §2.2 heading | Artificial Twin | D |
| README world sentence | Autonomous AI agents / AI researchers | C |
| `docs/os/GAIA-SUPER-OS.md` | AI-NATIVE KERNEL | A leftover — next living pass |
| `Documents/**` | many Artificial Intelligence strings | E |

## Next living pass (not this PR)

1. `docs/os/GAIA-SUPER-OS.md` headings AI-native → SI-native.
2. Decide whether `AI Dividend` becomes `SI Dividend`.
3. Decide whether `Artificial Twin` stays a product name.
4. Do not rename crates without a migration issue.
5. Do not rewrite `Documents/` paper titles.
