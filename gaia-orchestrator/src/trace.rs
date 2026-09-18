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
        self.events.lock().expect("trace sink lock").clone()
    }
}

impl TraceEventSink for InMemorySink {
    fn emit(&self, event: TraceEvent) {
        self.events.lock().expect("trace sink lock").push(event);
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
        outcome: if blocked { "blocked".into() } else { "clear".into() },
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
    if raw == "GAIA_REPLAY_REJECTED" || raw.contains("GAIA_REPLAY_REJECTED") {
        return (TraceEventKind::McpReplayRejected, "GAIA_REPLAY_REJECTED".into(), "deny".into());
    }
    if raw == "GAIA_SIGNATURE_INVALID" || raw.contains("GAIA_SIGNATURE_INVALID") {
        return (
            TraceEventKind::McpSignatureInvalid,
            "GAIA_SIGNATURE_INVALID".into(),
            "deny".into(),
        );
    }
    if raw == "GAIA_UNAUTHENTICATED" || raw.contains("GAIA_UNAUTHENTICATED") {
        return (
            TraceEventKind::McpUnauthenticated,
            "GAIA_UNAUTHENTICATED".into(),
            "deny".into(),
        );
    }
    if raw == "GAIA_CAPABILITY_DENIED" || raw.contains("GAIA_CAPABILITY_DENIED") {
        return (
            TraceEventKind::CapabilityDenied,
            "GAIA_CAPABILITY_DENIED".into(),
            "deny".into(),
        );
    }
    if raw.starts_with("GAIA_FW_") || raw.contains("GAIA_FW_") {
        let code = raw
            .split_whitespace()
            .find(|t| t.starts_with("GAIA_FW_"))
            .unwrap_or("GAIA_FW_DENIED")
            .to_string();
        return (TraceEventKind::IntentFirewallDenied, code, "deny".into());
    }
    if raw.contains("GAIA_GAP_LOCK") || raw.starts_with("gap-lock") {
        return (
            TraceEventKind::ExecutionBlockedByGapLock,
            "GAIA_GAP_LOCK_ACTIVE".into(),
            "blocked".into(),
        );
    }
    if raw.contains("CONTROL_PLANE_UNAVAILABLE") {
        return (
            TraceEventKind::ControlPlaneUnavailable,
            "GAIA_CONTROL_PLANE_UNAVAILABLE".into(),
            "unavailable".into(),
        );
    }
    if raw.starts_with("node-started") {
        return (TraceEventKind::NodeStarted, "GAIA_NODE_STARTED".into(), "allow".into());
    }
    if raw.starts_with("node-completed") {
        return (TraceEventKind::NodeCompleted, "GAIA_NODE_COMPLETED".into(), "allow".into());
    }
    if raw.starts_with("node-failed-over") {
        return (
            TraceEventKind::NodeFailedOver,
            "GAIA_NODE_FAILED_OVER".into(),
            "execution_failure".into(),
        );
    }
    if raw.contains("replay") {
        return (TraceEventKind::Replay, "GAIA_REPLAY".into(), "replay".into());
    }
    if raw.contains("fail") || raw.contains("error") {
        return (
            TraceEventKind::ExecutionFailure,
            "GAIA_EXECUTION_FAILURE".into(),
            "execution_failure".into(),
        );
    }
    (TraceEventKind::Other, "GAIA_OTHER".into(), "other".into())
}

fn firewall_rule(raw: &str) -> Option<String> {
    raw.split_whitespace()
        .find(|t| t.starts_with("GAIA_FW_"))
        .map(|s| s.to_string())
}

fn gap_id(raw: &str) -> Option<String> {
    raw.split_whitespace()
        .find(|t| t.starts_with("gap:"))
        .map(|s| s.trim_start_matches("gap:").to_string())
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
