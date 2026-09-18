# #335 runtime to trace_events boundary

Status: local contract only. Schema exists on `gaia-2-0`. Runtime does not write.

## Event shape

`TraceEvent`: kind, ts, actor_id, intent_id, correlation_id, outcome, reason code, request_hash.
Forbidden in the event: raw prompts, tokens, capability secrets, unrestricted tool output.

## Adapter

`TraceSink` is the only emission path. `MemoryTraceSink` is the default for `cargo test`.
`refuse_live_supabase()` stays Err until a separately reviewed client exists.

## Execution gate

- `GateMode::FailClosed`: missing or active gap lock blocks.
- `GateMode::LocalDev`: missing lock table is allowed so crates run without credentials.

## RLS / role (before any live write)

Writes to `trace_events` must use a constrained role, not the anon key.
Apply #334 advisor SQL first.
