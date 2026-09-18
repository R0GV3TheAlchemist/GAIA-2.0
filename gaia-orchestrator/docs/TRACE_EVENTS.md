# Trace events — Phase 1 local seam (#335)

Status: **local adapter only**. This document does not claim live writes to Supabase `trace_events`.

## Authority order

1. Kernel `AuditLog` hash chain (authoritative).
2. Orchestrator `AuditEvent` history.
3. Optional `TraceEventSink` (best-effort telemetry).

Sink failure must not invalidate the local chain. `TrustAudit::generate()` uses `NoopSink` so local/dev works without credentials.

## Privacy rules

Do not persist in telemetry:

- raw prompts or intent text
- secret material, credentials, capability tokens
- unrestricted tool or LLM output
- arbitrary error chains or free-form JSON blobs

Allow-listed `SafeTraceMeta` only: audit sequence, plan ID, pseudonymous executor ref, rule ID, key fingerprint, gap ID, node count, execution mode.

Unknown local event strings map to `TraceEventKind::Other` with reason `GAIA_OTHER`. The raw string is **not** copied into the trace event.

## Execution gate (specified, not remotely enforced here)

- Active gap lock → emit `ExecutionBlockedByGapLock` / `GAIA_GAP_LOCK_ACTIVE`. Runner should refuse new work (fail-closed in deployed mode).
- Control plane unreachable → emit `ControlPlaneUnavailable`. Local-dev fallback: continue with `NoopSink` and do not block the kernel audit path.

## Future Supabase mapping (not implemented)

Deployed `public.trace_events` columns: `event`, `gaian_id`, `correlation_id`, `canon_refs`, `started_at`, `ended_at`, `latency_ms`, `inputs`, `outputs`, `error`, `meta`.

A future feature-gated sink must:

- map to those names exactly
- write empty `{}` for `inputs` and `outputs` in v1
- use a least-privilege server role + RLS
- never `Debug`-print a service-role key
- never silently drop undelivered events without a local buffer policy

Phase 1 does **not** export `SupabaseSink`.

## RLS / key custody (prerequisite for live writes)

Document and review #334 before any live write integration. Required: dedicated role, RLS policies on `trace_events` / `gap_locks`, retention and incident evidence-preservation notes, no public HTTP endpoint.
