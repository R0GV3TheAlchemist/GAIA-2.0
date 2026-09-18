# #335 runtime to trace_events boundary

Status: local contract only. Schema exists on gaia-2-0. Runtime does not write.

TraceEvent: kind, ts, actor_id, intent_id, correlation_id, outcome, reason, request_hash.
Forbidden: raw prompts, tokens, capability secrets, unrestricted tool output.

FailClosed blocks missing or active gap locks. LocalDev allows missing lock tables.
refuse_live_supabase() stays Err until a reviewed client exists.
Apply #334 SQL before any live write.
