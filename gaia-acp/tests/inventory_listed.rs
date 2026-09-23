use gaia_acp::{
    from_invoke, ClaimClass, InvokeTraceInput, MemoryTraceSink, ReasonCode, TraceKind, TraceSink,
};

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
    let events = &sink.events;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].reason, ReasonCode::UntrustedAuthority.to_string());
}
