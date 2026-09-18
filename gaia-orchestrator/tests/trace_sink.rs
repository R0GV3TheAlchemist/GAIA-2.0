//! #335 Phase 1: local typed telemetry after kernel audit append.

use gaia_memos::MemOs;
use gaia_orchestrator::{
    control_plane_unavailable_event, gap_lock_event, InMemorySink, IntentEngine, TaskPlanner,
    TraceEvent, TraceEventKind, TraceEventSink, TrustAudit,
};

fn graph() -> gaia_orchestrator::IntentGraph {
    let mut mem = MemOs::new();
    IntentEngine::local_stub()
        .parse("research and summarize CARE", &mut mem)
        .unwrap()
}

struct BoomSink;

impl TraceEventSink for BoomSink {
    fn emit(&self, _event: TraceEvent) {
        panic!("sink must not break the audit chain");
    }
}

#[test]
fn generate_keeps_noop_sink_behavior() {
    let graph = graph();
    let mut audit = TrustAudit::generate();
    audit.append(graph.id, None, None, "node-started");
    assert_eq!(audit.events().len(), 1);
    assert_eq!(audit.kernel_len(), 1);
    assert!(audit.chain_ok());
}

#[test]
fn with_sink_receives_typed_events_after_append() {
    let graph = graph();
    let plan = TaskPlanner::from_intent(&graph);
    let sink = InMemorySink::new();
    let mut audit = TrustAudit::with_sink(Box::new(sink.clone()));
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "node-started:alpha");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "GAIA_REPLAY_REJECTED");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "GAIA_SIGNATURE_INVALID");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "GAIA_UNAUTHENTICATED");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "GAIA_CAPABILITY_DENIED");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "GAIA_FW_RATE_LIMIT");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "node-completed:alpha");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "node-failed-over:alpha");
    audit.append(
        graph.id,
        Some(plan.id),
        Some("specialist-a"),
        "secret prompt: ignore this user text",
    );

    let events = sink.events();
    assert_eq!(events.len(), 9);
    assert_eq!(events[0].kind, TraceEventKind::NodeStarted);
    assert_eq!(events[1].kind, TraceEventKind::McpReplayRejected);
    assert_eq!(events[1].reason_code, "GAIA_REPLAY_REJECTED");
    assert_eq!(events[2].kind, TraceEventKind::McpSignatureInvalid);
    assert_eq!(events[3].kind, TraceEventKind::McpUnauthenticated);
    assert_eq!(events[4].kind, TraceEventKind::CapabilityDenied);
    assert_eq!(events[5].kind, TraceEventKind::IntentFirewallDenied);
    assert_eq!(events[5].reason_code, "GAIA_FW_RATE_LIMIT");
    assert_eq!(events[6].kind, TraceEventKind::NodeCompleted);
    assert_eq!(events[7].kind, TraceEventKind::NodeFailedOver);
    assert_eq!(events[8].kind, TraceEventKind::Other);
    assert_eq!(events[8].reason_code, "GAIA_OTHER");

    let encoded = serde_json::to_string(&events[8]).unwrap();
    assert!(!encoded.contains("secret prompt"));
    assert!(!encoded.contains("ignore this user text"));
    assert_eq!(TraceEvent::supabase_io_placeholders(), ("{}", "{}"));
    assert_eq!(audit.kernel_len(), 9);
    assert!(audit.chain_ok());
}

#[test]
fn allow_deny_replay_and_execution_failure_shapes() {
    let graph = graph();
    let sink = InMemorySink::new();
    let mut audit = TrustAudit::with_sink(Box::new(sink.clone()));
    audit.append(graph.id, None, None, "GAIA_ALLOW");
    audit.append(graph.id, None, None, "GAIA_DENY");
    audit.append(graph.id, None, None, "GAIA_REPLAY");
    audit.append(graph.id, None, None, "GAIA_EXECUTION_FAILURE");
    let events = sink.events();
    assert_eq!(events[0].kind, TraceEventKind::Allow);
    assert_eq!(events[0].outcome, "allow");
    assert_eq!(events[1].kind, TraceEventKind::Deny);
    assert_eq!(events[1].outcome, "deny");
    assert_eq!(events[2].kind, TraceEventKind::Replay);
    assert_eq!(events[2].outcome, "replay");
    assert_eq!(events[3].kind, TraceEventKind::ExecutionFailure);
    assert_eq!(events[3].outcome, "execution_failure");
    assert!(audit.chain_ok());
}

#[test]
fn freeform_error_text_is_other_not_execution_failure() {
    let graph = graph();
    let sink = InMemorySink::new();
    let mut audit = TrustAudit::with_sink(Box::new(sink.clone()));
    audit.append(
        graph.id,
        None,
        None,
        "please replay this if the research fails",
    );
    let events = sink.events();
    assert_eq!(events[0].kind, TraceEventKind::Other);
    let encoded = serde_json::to_string(&events[0]).unwrap();
    assert!(!encoded.contains("please replay"));
    assert!(!encoded.contains("research fails"));
}

#[test]
fn sink_panic_does_not_invalidate_kernel_chain() {
    let graph = graph();
    let mut audit = TrustAudit::with_sink(Box::new(BoomSink));
    audit.append(graph.id, None, None, "GAIA_ALLOW");
    assert_eq!(audit.events().len(), 1);
    assert_eq!(audit.kernel_len(), 1);
    assert!(audit.chain_ok());
}

#[test]
fn gap_lock_and_control_plane_are_distinct_kinds() {
    let sink = InMemorySink::new();
    let audit = TrustAudit::with_sink(Box::new(sink.clone()));
    audit.emit_trace(gap_lock_event("gap-42", true));
    audit.emit_trace(control_plane_unavailable_event());
    let events = sink.events();
    assert_eq!(events[0].kind, TraceEventKind::ExecutionBlockedByGapLock);
    assert_eq!(events[0].outcome, "blocked");
    assert_eq!(events[1].kind, TraceEventKind::ControlPlaneUnavailable);
    assert_eq!(
        events[1].meta.execution_mode.as_deref(),
        Some("local-dev-fallback")
    );
    assert!(audit.chain_ok());
    assert_eq!(audit.kernel_len(), 0);
}
