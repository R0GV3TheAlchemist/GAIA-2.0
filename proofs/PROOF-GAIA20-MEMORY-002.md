# PROOF-GAIA20-MEMORY-002 — CT-002 Lifecycle Maintenance

## Proof

**Type:** Empirical
**Status:** Proven
**Method:** A live Supabase proof fixture was created with two memories having `last_accessed` timestamps 15 days before the maintenance run. One fixture began in `HOT`; the other began in `WARM`. The deployed `public.maintain_memory_tiers(now())` function was executed, then both resulting records were read back from `public.memories`.
**Results:**

| Fixture | Before | Expected | Observed | Decay rate after |
|---|---|---|---|---:|
| `proof-maintenance-hot-to-warm-001` | `HOT`, 0.01 | `HOT → WARM` | `WARM` | 0.03 |
| `proof-maintenance-warm-to-cold-001` | `WARM`, 0.03 | `WARM → COLD` | `COLD` | 0.07 |

The maintenance function returned one `HOT→WARM` transition and one `WARM→COLD` transition. Both persisted records had the expected tier-specific decay rate.

**Committed:** 2026-09-17, GAIA 2.0 Supabase project `yylqoiqobydrdsnnulip`

## Scope

This proof validates the inactivity branch of CT-002’s tiered-memory lifecycle:

- `HOT → WARM` after 14 days without access
- `WARM → COLD` after 14 days without access
- Tier decay rates: HOT 0.01, WARM 0.03, COLD 0.07

The access-triggered branch is independently covered by the live `public.access_memory(uuid)` validation.