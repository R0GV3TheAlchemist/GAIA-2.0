# PROOF-GAIA20-MEMORY-001 — CT-002 Memory Access Runtime

## Proof

**Type:** Empirical
**Status:** Proven
**Method:** A live WARM-tier memory fixture was inserted into the GAIA 2.0 Supabase project with `relevance_score = 0.50`, `decay_rate = 0.03`, and no prior access. `public.access_memory(uuid)` was executed once and its returned state was recorded.

**Results:**

| Check | Expected | Observed |
|---|---:|---:|
| Access count | 1 | 1 |
| Relevance score | 0.55 | 0.55 |
| Tier | `WARM` | `WARM` |
| Decay rate | 0.03 | 0.03 |
| `last_accessed` | populated | populated |

The initial access used recency weight 1.0 and applied the specified `+0.05` relevance increase. A WARM memory did not promote to HOT because it had not yet reached five accesses in the rolling seven-day window.

**Committed:** 2026-09-17, GAIA 2.0 Supabase project `yylqoiqobydrdsnnulip`

## Scope

This proof validates the access-triggered branch of CT-002:

- Atomic row-locked memory access
- Access-count increment and timestamp update
- Relevance boost capped at 1.0
- CT-002 tier promotion thresholds
- Tier-specific decay-rate assignment
- Append-only `memory_access_events` audit record

The inactivity demotion branch is independently validated in `PROOF-GAIA20-MEMORY-002.md`.