# Trace events — local seam + mapped row (#335)

Status: **local adapter verified**. Live HTTP writes to Supabase `trace_events` are **not** implemented and **not** claimed.

Deployed schema exists in project `gaia-2-0`. Verified runtime integration does **not**.

## Authority order

1. Kernel `AuditLog` hash chain (authoritative).
2. Orchestrator `AuditEvent` history.
3. Optional `TraceEventSink` (best-effort telemetry).

Sink failure must not invalidate the local chain. `TrustAudit::append` catches sink panics after the kernel append succeeds. `TrustAudit::generate()` uses `NoopSink` so local/dev works without credentials.

## Privacy rules

Do not persist in telemetry:

- raw prompts or intent text
- secret material, credentials, capability tokens
- unrestricted tool or LLM output
- arbitrary error chains or free-form JSON blobs

Allow-listed `SafeTraceMeta` only: audit sequence, plan ID, pseudonymous executor ref, rule ID, key fingerprint, gap ID, node count, execution mode.

Classification uses the first whitespace-separated token only. Known codes must match exactly or use a controlled prefix (`GAIA_FW_*`, `node-started*`). Unknown strings map to `TraceEventKind::Other` with reason `GAIA_OTHER`. The raw string is **not** copied into the trace event.

## Execution gate (wired locally)

`permit_execution` is the runner check. It does not call Supabase.

- Active gap lock → `Deny` + `ExecutionBlockedByGapLock` / `GAIA_GAP_LOCK_ACTIVE`. Fail-closed in every mode.
- Control plane unreachable + deployed → `Deny` + `ControlPlaneUnavailable`.
- Control plane unreachable + local-dev → `Allow` with telemetry; do not block the kernel audit path.

`LocalRunner::run` uses a clear `InMemoryGate` (no network).
`LocalRunner::run_with_gate` is the testable wiring. Covered in `tests/approved_run.rs`.

## Mapped live row (still no HTTP)

`gaia-acp` `LiveTraceRow` now uses deployed column names:

`event`, `gaian_id`, `correlation_id`, `canon_refs`, `started_at`, `ended_at`, `latency_ms`, `inputs`, `outputs`, `error`, `meta`.

Rules:

- `inputs` and `outputs` are always `{}`
- `canon_refs` is empty unless an explicit safe ref is later added
- `error` is a stable reason code, never a chain
- allow-listed extras live only in `meta`
- default mode is `Off`; `MappedOnly` + `TestBoundary` records in memory only
- no `SupabaseSink` export; no credentials in this workspace

## RLS / key custody (prerequisite for live writes)

Do not enable live writes until #334 is closed and rechecked.

Required before any runtime insert:

- Dedicated server-only role, not the anon key and not the service-role key in application logs.
- RLS policies on `trace_events` and `gap_locks` that allow insert/select only for that role.
- `constitution_articles` remains server-only; no client read path.
- Retention: operational traces default to 30 days unless an incident flag preserves evidence.
- Incident evidence-preservation: freeze deletion for correlated `correlation_id` rows; do not rewrite `inputs`/`outputs`.
- No public HTTP endpoint.
- Hugging Face is not a telemetry store.

See also `docs/TRACE_BOUNDARY_STRIDE_SRE_SAELA.md`.
