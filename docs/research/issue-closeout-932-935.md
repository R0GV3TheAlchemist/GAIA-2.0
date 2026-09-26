# Closeout map for #932 #933 #934 #935

Listed evidence. No new runtime.

## #932 Grounding and attribution

- `gaia-runtime` `GroundedResponse`, `enforce_grounding`
- `score_faithfulness` precision/recall/jaccard/composite
- `attribute_sentences` `[source: hex]`
- `gaia-spec/rag/attribution.md`
- `gaia-acp` `GroundingClaim` + `invoke_grounded` + `with_faithfulness`

## #933 Tool auth and audit

- `gaia-agents` `tool_registry.rs` permission tiers
- `gaia-acp` `tool_auth.rs` + append-only `ToolAuditLog`
- Lives in ACP, not a separate `gaia-security` crate

## #934 Circuit breakers and quotas

- `gaia-runtime` `circuit_breaker.rs`, `quotas.rs`, `dormancy.rs`
- Unit tests: three failures open; 101st call limited; dormant rejects; half-open success closes

## #935 Contract tests

- `gaia-integrity/tests/contracts.rs` ingest→aikd, aikd→runtime, agents→acp
- `ingest_to_memos.rs`, `memos_to_aikd.rs` added this PR
- `cargo test -p gaia-integrity --features contracts`
