# GAIA 2.0 Supabase Schema

**Filed:** 2026-09-17  
**Updated:** 2026-09-17 Slice B  
**Project:** `gaia-2-0`  
**Project ID:** `yylqoiqobydrdsnnulip`  
**Region:** `us-east-2`  
**Status:** `ACTIVE_HEALTHY`  
**Engine:** Postgres 17.6.1  
**Host:** `db.yylqoiqobydrdsnnulip.supabase.co`

This is a live inventory, not a wishlist. All listed tables have **RLS enabled**.

## Applied migrations

| Version | Name |
|---|---|
| 20260917203522 | `gaia20_backbone_memory_consent_lithic` |
| 20260917205016 | `gaia20_fix_memory_metric_security_invoker` |
| 20260917205804 | `gaia20_memory_access_runtime` |
| 20260917210019 | `gaia20_memory_tier_maintenance` |
| 20260917212228 | `gaia20_scheduled_memory_maintenance` |
| 20260917212646 | `gaia20_agent_resilience_runtime` |
| (Slice B) | `gaia20_memory_truth_slice_b` |

Repo copies live under `supabase/migrations/` in `GAIA-2.0`.

## Enums (live)

| Enum | Values |
|---|---|
| `memory_tier` | HOT, WARM, COLD |
| `cognitive_type` | working, short_term, episodic, semantic, procedural, long_term |
| `consent_action` | grant, revoke, failover, suspend, resume |
| `agent_role` | orchestrator, execution, safety, consent, memory, knowledge, interface, monitor |
| `circuit_breaker_state` | CLOSED, OPEN, HALF_OPEN |
| `proof_type` | simulation, formal, empirical, emergent_confirmation |
| `proof_status` | proven, partial, in_progress |

## Tables (live row counts at Slice B verification)

### `public.memories` (3 rows)

Operational HOT/WARM/COLD store plus cognitive class and provenance.

| Column | Type | Notes |
|---|---|---|
| `memory_id` | uuid PK | `gen_random_uuid()` |
| `created_at` | timestamptz | default now() |
| `content_hash` | text | integrity |
| `content` | jsonb | default `{}` |
| `tier` | memory_tier | storage temperature; default WARM |
| `last_accessed` | timestamptz | nullable |
| `access_count` | int | default 0 |
| `relevance_score` | float8 | 0–1, default 0.5 |
| `decay_rate` | float8 | default 0.03 |
| `tier_upgraded_at` | timestamptz | nullable |
| `owner_id` | uuid | nullable; RLS subject |
| `updated_at` | timestamptz | default now() |
| `cognitive_type` | cognitive_type | default episodic |
| `gaian_id` | text | nullable; not an RLS key |
| `provenance_source` | text | default unspecified |
| `confidence` | float8 | 0–1, default 0.5 |

**Bridge:** `cognitive_type` is the MemoryHierarchy / Research 002 class. HOT/WARM/COLD is storage temperature only.

**Metric 6:** view `memory_metric_6_retention_30d` (`security_invoker = true`). Cohort = memories aged ≥30 days. Retained = still HOT or WARM. Target ≥0.85. COLD counts as not retained (Metric 8). Ratio is NULL until a 30-day cohort exists.

### `public.memory_access_events` (1 row)

| Column | Type |
|---|---|
| `id` | uuid PK |
| `memory_id` | uuid FK → memories |
| `actor_id` | uuid nullable |
| `accessed_at` | timestamptz |
| `tier_before` / `tier_after` | memory_tier |
| `relevance_before` / `relevance_after` | float8 |
| `recency_weight` | float8 in {0.2, 0.5, 1.0} |

Hugging Face dataset `gaia-memory-access-log` still requires RESEARCH_USE consent and anonymization. Not published.

### `public.consent_events` (2 rows)

Unchanged in Slice B. Next is Slice C (HMAC chain, fourteen scopes, cryptographic erasure).

### `public.agent_health` (10 rows)

Unchanged in Slice B.

### `public.agent_incidents` (5 rows)

Unchanged in Slice B.

### Lithic tables

`elements` (3), `minerals` (1), `crystals` (1), `proofs` (1), `maintenance_runs` (1) unchanged.

## RLS posture

RLS is on for every public table. `gaian_id` does not bypass `owner_id`.

## Next tables (not live yet)

| Table | Why |
|---|---|
| `gfi_scores` | Flourishing Index subscales + composite |
| `coherence_sessions` | HRV / PANAS / cortisol research pipeline |
| `elemental_learning_enrollments` | Register-matched education study |
| `canon_nodes` / `canon_edges` | Runtime projection of CanonGraph |
| `consent_keys` | Cryptographic erasure vault metadata |
| `gaian_identities` | Identity file + Telos (Issue #218) |
| `alscn_gan_runs` | Canon C67 simulation outputs |
