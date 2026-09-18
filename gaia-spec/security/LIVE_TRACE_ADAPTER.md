# Live trace adapter (#335)

Default: off. `refuse_live_supabase()` stays `Err`.

Authoritative chain is the local `MemoryTraceSink` / audit receipts.
Live forward is best-effort and must not roll back local events.

## Contract

- Map typed local fields only.
- `inputs` and `outputs` are `{}`.
- `reason_code` is a stable `ReasonCode` string, never a raw error.
- Metadata allow-list: `schema`, `source`.
- No URL, service role, or bearer in this crate, logs, or CI.
- Isolated tests use `LiveTraceRole::TestBoundary` and `RecordingLiveTransport`.
- A production HTTP client is out of this crate on purpose.
