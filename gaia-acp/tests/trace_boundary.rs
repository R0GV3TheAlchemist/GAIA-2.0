use gaia_acp::{
    from_invoke, ClaimClass, InvokeTraceInput, MemoryTraceSink, ReasonCode, TraceKind, TraceSink,
};

#[test]
fn allow_and_deny_are_recorded() {
    let mut sink = MemoryTraceSink::default();

    sink.emit(from_invoke(InvokeTraceInput {
        kind: TraceKind::Allow,
        ts: 1,
        actor_id: "agent-a",
        intent_id: "intent-a",
        correlation_id: "c1",
        reason: ReasonCode::Allow,
        request_hash: "h1",
        claim_class: ClaimClass::Established,
    }));

    sink.emit(from_invoke(InvokeTraceInput {
        kind: TraceKind::Deny,
        ts: 2,
        actor_id: "agent-a",
        intent_id: "intent-a",
        correlation_id: "c2",
        reason: ReasonCode::ToolNotListed,
        request_hash: "h2",
        claim_class: ClaimClass::Established,
    }));

    sink.emit(from_invoke(InvokeTraceInput {
        kind: TraceKind::Replay,
        ts: 3,
        actor_id: "agent-a",
        intent_id: "intent-a",
        correlation_id: "c3",
        reason: ReasonCode::ApprovalReplay,
        request_hash: "h3",
        claim_class: ClaimClass::Established,
    }));

    sink.emit(from_invoke(InvokeTraceInput {
        kind: TraceKind::ExecutionFailure,
        ts: 4,
        actor_id: "agent-a",
        intent_id: "intent-a",
        correlation_id: "c4",
        reason: ReasonCode::StateInvalid,
        request_hash: "h4",
        claim_class: ClaimClass::Established,
    }));

    assert_eq!(sink.events.len(), 4);
}

#[test]
fn prohibited_claim_is_tagged() {
    let mut sink = MemoryTraceSink::default();
    let ev = from_invoke(InvokeTraceInput {
        kind: TraceKind::Allow,
        ts: 1,
        actor_id: "agent-a",
        intent_id: "intent-a",
        correlation_id: "c1",
        reason: ReasonCode::Allow,
        request_hash: "h1",
        claim_class: ClaimClass::Prohibited,
    });
    sink.emit(ev);
    assert_eq!(sink.events.len(), 1);
    assert_eq!(sink.events[0].claim_class, ClaimClass::Prohibited);
}
