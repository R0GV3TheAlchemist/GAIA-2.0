use gaia_acp::trace::{from_invoke, ClaimClass, InvokeTraceInput, MemoryTraceSink, TraceKind, TraceSink};
use gaia_acp::types::ReasonCode;

#[test]
fn untrusted_authority_is_logged() {
    let mut sink = MemoryTraceSink::default();
    let ev = from_invoke(InvokeTraceInput {
        kind: TraceKind::Allow,
        ts: 1,
        actor_id: "agent-a",
        intent_id: "intent-1",
        correlation_id: "corr-1",
        reason: ReasonCode::UntrustedAuthority,
        request_hash: "hash",
        claim_class: ClaimClass::Prohibited,
    });
    sink.emit(ev);
    let events = sink.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].reason, ReasonCode::UntrustedAuthority);
}
