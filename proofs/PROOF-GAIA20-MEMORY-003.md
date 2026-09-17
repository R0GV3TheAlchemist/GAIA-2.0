# PROOF-GAIA20-MEMORY-003 — Scheduled, Audited Memory Maintenance

## Proof

**Type:** Empirical
**Status:** Proven
**Method:** The `pg_cron` extension was installed in the live GAIA 2.0 Supabase project. The scheduled job was read from `cron.job`. The audited wrapper `public.run_memory_tier_maintenance()` was then executed once manually, and its record was read from `public.maintenance_runs`.

**Results:**

| Check | Observed |
|---|---|
| Scheduled job ID | `1` |
| Job name | `gaia-memory-tier-maintenance-daily` |
| Cron schedule | `0 5 * * *` (05:00 UTC daily) |
| Job command | `SELECT public.run_memory_tier_maintenance();` |
| Job active | `true` |
| Audit run ID | `973cdbd3-5895-4a73-ba50-5c246bdadbc8` |
| Audit status | `succeeded` |
| Recorded transition counts | `{ "WARM→COLD": 1 }` |

The daily database-native schedule is active. Its audited wrapper successfully invoked CT-002 maintenance and persisted a completion record with transition counts.

**Committed:** 2026-09-17, GAIA 2.0 Supabase project `yylqoiqobydrdsnnulip`

## Scope

This proof validates that CT-002 maintenance is not merely manually callable: it has an enabled daily execution schedule and produces an auditable record for every wrapper invocation.