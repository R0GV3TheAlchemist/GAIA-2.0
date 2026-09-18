//! Privacy-minimized operational telemetry (#335 Phase 1).
//!
//! The sink is best-effort and is not an authorization mechanism.
//! It must never replace the kernel hash-chained audit log.

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TraceEventKind {
    Allow,
    Deny,
    Replay,
    ExecutionFailure,
    McpReplayRejected,
    McpSignatureInvalid,
    McpUnauthenticated,
    CapabilityDenied,
    IntentFirewallDenied,
    NodeStarted,
    NodeCompleted,
    NodeFailedOver,
    ExecutionBlockedByGapLock,
    ControlPlaneUnavailable,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SafeTraceMeta {
    pub audit_sequence: Option<u64>,
    pub plan_id: Option<Uuid>,
    pub executor_ref: Option<String>,
    pub rule_id: Option<String>,
    pub key_fingerprint: Option<String>,
    pub gap_id: Option<String>,
    pub node_count: Option<u32>,
    pub execution_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceEvent {
    pub kind: TraceEventKind,
    pub reason_code: String,
    pub ts_unix_ms: u64,
    pub intent_ref: Option<Uuid>,
    pub correlation_id: Option<String>,
    pub outcome: String,
    pub meta: SafeTraceMeta,
}

impl TraceEvent {
    pub fn classify(
        raw_event: &str,
        intent_id: Uuid,
        plan_id: Option<Uuid>,
        executor_id: Option<&str>,
        sequence: u64,
    ) -> Self {
        let (kind, reason_code, outcome) = classify_event(raw_event);
        Self {
            kind,
            reason_code,
            ts_unix_ms: now_ms(),
            intent_ref: Some(intent_id),
            correlation_id: Some(format!("audit-{sequence}")),
            outcome,
            meta: SafeTraceMeta {
                audit_sequence: Some(sequence),
                plan_id,
                executor_ref: executor_id.map(sanitize_ref),
                rule_id: firewall_rule(raw_event),
                key_fingerprint: None,
                gap_id: gap_id(raw_event),
                node_count: None,
                execution_mode: Some("local".into()),
            },
        }
    }

    /// Mapping contract for a future Supabase sink. Inputs/outputs stay empty.
    pub fn supabase_io_placeholders() -> (&'static str, &'static str) {
        ("{}", "{}")
    }
}

pub trait TraceEventSink: Send + Sync {
    fn emit(&self, event: TraceEvent);
}

#[derive(Debug, Default)]
pub struct NoopSink;

impl TraceEventSink for NoopSink {
    fn emit(&self, _event: TraceEvent) {}
}

#[derive(Debug, Clone, Default)]
pub struct InMemorySink {
    events: Arc<Mutex<Vec<TraceEvent>>>,
}

impl InMemorySink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn events(&self) -> Vec<TraceEvent> {
        self.events
            .lock()
            .map(|g| g.clone())
            .unwrap_or_default()
    }
}

impl TraceEventSink for InMemorySink {
    fn emit(&self, event: TraceEvent) {
        if let Ok(mut g) = self.events.lock() {
            g.push(event);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateState {
    Clear,
    BlockedByGapLock { gap_id: String },
    ControlPlaneUnavailable,
}

pub trait ExecutionGate: Send + Sync {
    fn state(&self) -> GateState;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryGate {
    state: GateState,
}

impl InMemoryGate {
    pub fn clear() -> Self {
        Self {
            state: GateState::Clear,
        }
    }

    pub fn locked(gap_id: impl Into<String>) -> Self {
        Self {
            state: GateState::BlockedByGapLock {
                gap_id: sanitize_ref(&gap_id.into()),
            },
        }
    }

    pub fn unavailable() -> Self {
        Self {
            state: GateState::ControlPlaneUnavailable,
        }
    }
}

impl ExecutionGate for InMemoryGate {
    fn state(&self) -> GateState {
        self.state.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunPermit {
    Allow { telemetry: Option<TraceEvent> },
    Deny { event: TraceEvent },
}

/// Local execution-gate contract. No remote I/O.
/// Deployed mode fail-closes on gap lock and control-plane unavailability.
/// Local-dev unavailability does not block the kernel audit path.
pub fn permit_execution(gate: &dyn ExecutionGate, deployed: bool) -> RunPermit {
    match gate.state() {
        GateState::Clear => RunPermit::Allow { telemetry: None },
        GateState::BlockedByGapLock { gap_id } => RunPermit::Deny {
            event: gap_lock_event(&gap_id, true),
        },
        GateState::ControlPlaneUnavailable => {
            let event = control_plane_unavailable_event();
            if deployed {
                RunPermit::Deny { event }
            } else {
                RunPermit::Allow {
                    telemetry: Some(event),
                }
            }
        }
    }
}

/// Distinct typed events for the execution-gate contract (#335).
pub fn gap_lock_event(gap_id: &str, blocked: bool) -> TraceEvent {
    TraceEvent {
        kind: TraceEventKind::ExecutionBlockedByGapLock,
        reason_code: "GAIA_GAP_LOCK_ACTIVE".into(),
        ts_unix_ms: now_ms(),
        intent_ref: None,
        correlation_id: None,
        outcome: if blocked {
            "blocked".into()
        } else {
            "clear".into()
        },
        meta: SafeTraceMeta {
            gap_id: Some(sanitize_ref(gap_id)),
            execution_mode: Some("local".into()),
            ..SafeTraceMeta::default()
        },
    }
}

pub fn control_plane_unavailable_event() -> TraceEvent {
    TraceEvent {
        kind: TraceEventKind::ControlPlaneUnavailable,
        reason_code: "GAIA_CONTROL_PLANE_UNAVAILABLE".into(),
        ts_unix_ms: now_ms(),
        intent_ref: None,
        correlation_id: None,
        outcome: "unavailable".into(),
        meta: SafeTraceMeta {
            execution_mode: Some("local-dev-fallback".into()),
            ..SafeTraceMeta::default()
        },
    }
}

fn classify_event(raw: &str) -> (TraceEventKind, String, String) {
    let token = raw.split_whitespace().next().unwrap_or(raw);
    if token == "GAIA_ALLOW" {
        return (TraceEventKind::Allow, "GAIA_ALLOW".into(), "allow".into());
    }
    if token == "GAIA_DENY" {
        return (TraceEventKind::Deny, "GAIA_DENY".into(), "deny".into());
    }
    if token == "GAIA_REPLAY" {
        return (TraceEventKind::Replay, "GAIA_REPLAY".into(), "replay".into());
    }
    if token == "GAIA_EXECUTION_FAILURE" {
        return (
            TraceEventKind::ExecutionFailure,
            "GAIA_EXECUTION_FAILURE".into(),
            "execution_failure".into(),
        );
    }
    if token == "GAIA_REPLAY_REJECTED" {
        return (
            TraceEventKind::McpReplayRejected,
            "GAIA_REPLAY_REJECTED".into(),
            "deny".into(),
        );
    }
    if token == "GAIA_SIGNATURE_INVALID" {
        return (
            TraceEventKind::McpSignatureInvalid,
            "GAIA_SIGNATURE_INVALID".into(),
            "deny".into(),
        );
    }
    if token == "GAIA_UNAUTHENTICATED" {
        return (
            TraceEventKind::McpUnauthenticated,
            "GAIA_UNAUTHENTICATED".into(),
            "deny".into(),
        );
    }
    if token == "GAIA_CAPABILITY_DENIED" {
        return (
            TraceEventKind::CapabilityDenied,
            "GAIA_CAPABILITY_DENIED".into(),
            "deny".into(),
        );
    }
    if token.starts_with("GAIA_FW_") {
        return (
            TraceEventKind::IntentFirewallDenied,
            sanitize_ref(token),
            "deny".into(),
        );
    }
    if token == "GAIA_GAP_LOCK_ACTIVE" || token == "gap-lock" {
        return (
            TraceEventKind::ExecutionBlockedByGapLock,
            "GAIA_GAP_LOCK_ACTIVE".into(),
            "blocked".into(),
        );
    }
    if token == "GAIA_CONTROL_PLANE_UNAVAILABLE" {
        return (
            TraceEventKind::ControlPlaneUnavailable,
            "GAIA_CONTROL_PLANE_UNAVAILABLE".into(),
            "unavailable".into(),
        );
    }
    if token.starts_with("node-started") {
        return (
            TraceEventKind::NodeStarted,
            "GAIA_NODE_STARTED".into(),
            "allow".into(),
        );
    }
    if token.starts_with("node-completed") {
        return (
            TraceEventKind::NodeCompleted,
            "GAIA_NODE_COMPLETED".into(),
            "allow".into(),
        );
    }
    if token.starts_with("node-failed-over") {
        return (
            TraceEventKind::NodeFailedOver,
            "GAIA_NODE_FAILED_OVER".into(),
            "execution_failure".into(),
        );
    }
    (
        TraceEventKind::Other,
        "GAIA_OTHER".into(),
        "other".into(),
    )
}

fn firewall_rule(raw: &str) -> Option<String> {
    raw.split_whitespace()
        .next()
        .filter(|t| t.starts_with("GAIA_FW_"))
        .map(sanitize_ref)
}

fn gap_id(raw: &str) -> Option<String> {
    let mut parts = raw.split_whitespace();
    match parts.next()? {
        "GAIA_GAP_LOCK_ACTIVE" | "gap-lock" => parts
            .find(|t| t.starts_with("gap:"))
            .map(|s| sanitize_ref(s.trim_start_matches("gap:"))),
        _ => None,
    }
}

fn sanitize_ref(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ':'))
        .take(64)
        .collect()
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freeform_text_does_not_classify_as_failure_or_replay() {
        let (kind, code, _) = classify_event("please replay this if the research fails");
        assert_eq!(kind, TraceEventKind::Other);
        assert_eq!(code, "GAIA_OTHER");
    }

    #[test]
    fn permit_execution_fail_closes_on_gap_lock() {
        let gate = InMemoryGate::locked("gap-42");
        match permit_execution(&gate, true) {
            RunPermit::Deny { event } => {
                assert_eq!(event.kind, TraceEventKind::ExecutionBlockedByGapLock);
                assert_eq!(event.reason_code, "GAIA_GAP_LOCK_ACTIVE");
            }
            RunPermit::Allow { .. } => panic!("gap lock must deny"),
        }
    }

    #[test]
    fn local_dev_unavailability_does_not_block() {
        let gate = InMemoryGate::unavailable();
        match permit_execution(&gate, false) {
            RunPermit::Allow { telemetry } => {
                let event = telemetry.expect("emit unavailable");
                assert_eq!(event.kind, TraceEventKind::ControlPlaneUnavailable);
            }
            RunPermit::Deny { .. } => panic!("local-dev must not block"),
        }
    }

    #[test]
    fn deployed_unavailability_fail_closes() {
        let gate = InMemoryGate::unavailable();
        assert!(matches!(
            permit_execution(&gate, true),
            RunPermit::Deny { .. }
        ));
    }
}
