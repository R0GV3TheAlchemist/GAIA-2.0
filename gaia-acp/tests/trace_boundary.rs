use gaia_acp::*;

#[test]
fn emits_allow_deny_replay_and_failure_shapes() {
    let mut sink = MemoryTraceSink::default();
    sink.emit(from_invoke(
        TraceKind::Allow, 1, "agent-a", "intent-a", "c1", ReasonCode::Allow, "h1",
    ));
    sink.emit(from_invoke(
        TraceKind::Deny, 2, "agent-a", "intent-a", "c2", ReasonCode::ToolNotListed, "h2",
    ));
    sink.emit(from_invoke(
        TraceKind::Replay, 3, "agent-a", "intent-a", "c3", ReasonCode::ApprovalReplay, "h3",
    ));
    sink.emit(from_invoke(
        TraceKind::ExecutionFailure, 4, "agent-a", "intent-a", "c4", ReasonCode::StateInvalid, "h4",
    ));
    assert_eq!(
        sink.kinds(),
        vec![
            TraceKind::Allow,
            TraceKind::Deny,
            TraceKind::Replay,
            TraceKind::ExecutionFailure,
        ]
    );
    assert!(!sink.events[0].leaks(&["sk-secret", "Bearer abc"]));
}

#[test]
fn gap_lock_blocks_fail_closed_and_local_dev_runs_without_remote() {
    assert!(execution_allowed(Some(GapLock { active: true }), GateMode::LocalDev).is_err());
    assert!(execution_allowed(None, GateMode::FailClosed).is_err());
    assert!(execution_allowed(None, GateMode::LocalDev).is_ok());
    assert!(execution_allowed(Some(GapLock { active: false }), GateMode::FailClosed).is_ok());
}

#[test]
fn live_supabase_write_is_refused() {
    assert!(refuse_live_supabase().is_err());
}
