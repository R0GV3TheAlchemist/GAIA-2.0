# GAIA 2.0 Supabase Schema

**Filed:** 2026-09-17  
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

Repo copies live under `supabase/migrations/` in `GAIA-2.0`.

## Enums (live)

| Enum | Values |
|---|---|
| `memory_tier` | HOT, WARM, COLD |
| `consent_action` | grant, revoke, failover, suspend, resume |
| `agent_role` | orchestrator, execution, safety, consent, memory, knowledge, interface, monitor |
| `circuit_breaker_state` | CLOSED, OPEN, HALF_OPEN |
| `proof_type` | simulation, formal, empirical, emergent_confirmation |
| `proof_status` | proven, partial, in_progress |

## Tables (live row counts at inventory)

### `public.memories` (3 rows)

Operational HOT/WARM/COLD store.

| Column | Type | Notes |
|---|---|---|
| `memory_id` | uuid PK | `gen_random_uuid()` |
| `created_at` | timestamptz | default now() |
| `content_hash` | text | integrity |
| `content` | jsonb | default `{}` |
| `tier` | memory_tier | default WARM |
| `last_accessed` | timestamptz | nullable |
| `access_count` | int | default 0 |
| `relevance_score` | float8 | 0–1, default 0.5 |
| `decay_rate` | float8 | default 0.03 |
| `tier_upgraded_at` | timestamptz | nullable |
| `owner_id` | uuid | nullable; RLS subject |
| `updated_at` | timestamptz | default now() |

**Bridge to MemoryHierarchy:** add `cognitive_type` (`working|short_term|episodic|semantic|procedural|long_term`) and `gaian_id` without breaking HOT/WARM/COLD.

**Metric 6:** retention ≥85% at 30 days is computed on HOT+WARM only. COLD is excluded by design.

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

This table is the seed for Hugging Face dataset `gaia-memory-access-log` after anonymization.

### `public.consent_events` (2 rows)

| Column | Type |
|---|---|
| `id` | uuid PK |
| `created_at` | timestamptz |
| `subject_id` / `actor_id` | uuid nullable |
| `action` | consent_action |
| `scope` | text |
| `cause` | text nullable |
| `duration_ms` | int nullable |
| `shard_key` | text nullable |
| `payload` | jsonb |

**Gap vs Consent Ledger spec (#127):** live table is an event log, not yet an HMAC-chained, append-only cryptographic ledger with per-scope AES-256 keys and ErasureReceipts. Next migration must add:

- `prev_hash`, `entry_hash`, `signature`
- `scope` constrained to the fourteen scopes
- no UPDATE/DELETE grants for authenticated roles
- cryptographic erasure vault metadata (key_id, destroyed_at, destruction_proof)

Fourteen scopes: EPISODIC_MEMORY, SEMANTIC_MEMORY, EMOTIONAL_PROFILE, ARCHETYPAL_PROFILE, SOMATIC_PROFILE, TRANSPERSONAL_HISTORY, INDIVIDUATION_RECORD, IDENTITY_ANCHORS, CULTURAL_PROFILE, PERSONHOOD_TELEMETRY, SHADOW_HISTORY, CONSENT_LEDGER_ITSELF, THIRD_PARTY_SHARING, RESEARCH_USE.

### `public.agent_health` (10 rows)

| Column | Type |
|---|---|
| `id` | uuid PK |
| `agent_role` | agent_role |
| `instance_id` | text |
| `is_primary` | bool |
| `cb_state` | circuit_breaker_state default CLOSED |
| `failure_count` | int |
| `last_failure_at` / `last_failover_at` | timestamptz |
| `uptime_ratio` | float8 |
| `observed_at` | timestamptz |

Targets from CT-003: Execution failure ≤2% under nominal load; Safety/Consent hot-standby; failover <500ms; trip ≥3 failures / 30s; recovery probe 60s; Execution suspends if Safety or Consent breaker is OPEN.

### `public.agent_incidents` (5 rows)

Event types: `failure`, `circuit_opened`, `circuit_half_open`, `circuit_closed`, `failover`, `execution_suspended`.

### `public.elements` (3 rows)

PK `z` (1–118). Columns: `symbol`, `name`, `data` jsonb, `updated_at`.

### `public.minerals` (1 row)

PK `id`. Includes `formula`, `element_ids`, `mineral_class`, `crystal_system`, `hardness_mohs`, `gaia_layer_alignment[]`, `crystal_link`, `notes`, `data` jsonb.

### `public.crystals` (1 row)

PK `crystal_id`. Includes `septagram_nodes[]`, `data` jsonb, `version`, `last_updated`.

### `public.proofs` (1 row)

PK `proof_id`. Types and statuses as enums. Optional `canon_path`, `github_sha`, `committed_at`.

### `public.maintenance_runs` (1 row)

Scheduled HOT/WARM/COLD re-evaluation log.

## RLS posture

RLS is on for every public table. That is necessary and not sufficient.

Production policies must encode:

- `owner_id = auth.uid()` for memories and access events
- Execution writes blocked unless a matching consent grant is active for that scope
- `consent_events` insert-only for user/service roles
- lithic tables readable to authenticated; writable only to knowledge/service role
- `proofs` writable only to CI service role

Supabase RLS is a PostgreSQL WHERE clause applied to every query. Do not bypass it with the service role from the client.

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

## Cost and ethics bounds

- No real child data in this project.
- No clinical claims from `coherence_sessions` until pre-registration and ethics approval.
- RESEARCH_USE consent required before any export to Hugging Face.
- Anonymize `owner_id` / `subject_id` before leaving Supabase.
