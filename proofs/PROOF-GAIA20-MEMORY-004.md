# Proof — GAIA20-MEMORY-004

**Slice:** B — Memory truth  
**Filed:** 2026-09-17  
**Project:** `gaia-2-0` (`yylqoiqobydrdsnnulip`)

## Proof

- Type: simulation
- Status: partial
- Method: apply_migration `gaia20_memory_truth_slice_b`; verified with verbose table inventory
- Results: `public.memories` has `cognitive_type`, `gaian_id`, `provenance_source`, `confidence`; RLS remains enabled; existing 3 rows defaulted to `episodic` / `unspecified` / 0.5. View `memory_metric_6_retention_30d` created `WITH (security_invoker = true)`.
- Artefacts: `supabase/migrations/20260917_gaia20_memory_truth_slice_b.sql`

## Why partial

Metric 6 cannot be proven or failed until a cohort aged ≥30 days exists. Current seed rows are younger than 30 days, so `retention_ratio` is NULL. That is correct, not a pass.

## Bridge rule

- `tier` = storage temperature (HOT / WARM / COLD)
- `cognitive_type` = working / short_term / episodic / semantic / procedural / long_term
- RLS subject remains `owner_id`
- Hugging Face export of `memory_access_events` still requires RESEARCH_USE consent
