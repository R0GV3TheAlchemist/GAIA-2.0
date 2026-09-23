//! Feature-gated live trace mapper (#335).
//! Default path: off. No HTTP client. No credentials in this crate.

use crate::trace::{ClaimClass, TraceEvent, TraceKind};
use serde::{Deserialize, Serialize};

/// Isolated data boundary. Production service role is never constructed in tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveTraceRole {
    TestBoundary,
}

/// Operating mode for the live trace forwarder.
///
/// `Off` is the default: no rows leave the local process. `MappedOnly` enables
/// forwarding of allow-listed, sanitised rows to the configured transport.
/// The `#[default]` attribute and `#[derive(Default)]` replace the previous
/// manual `impl Default` (clippy::derivable_impls).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LiveTraceMode {
    #[default]
    Off,
    MappedOnly,
}

/// Server-side config. URL and secret live outside this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveTraceConfig {
    pub mode: LiveTraceMode,
    pub role: LiveTraceRole,
}

impl LiveTraceConfig {
    pub fn off() -> Self {
        Self {
            mode: LiveTraceMode::Off,
            role: LiveTraceRole::TestBoundary,
        }
    }

    pub fn mapped_test() -> Self {
        Self {
            mode: LiveTraceMode::MappedOnly,
            role: LiveTraceRole::TestBoundary,
        }
    }
}

/// Row shaped like deployed `public.trace_events`.
/// Inputs/outputs stay empty objects. `error` is a stable reason code only.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiveTraceRow {
    pub event: String,
    pub gaian_id: Option<String>,
    pub correlation_id: Option<String>,
    pub canon_refs: Vec<String>,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub latency_ms: Option<u64>,
    pub inputs: serde_json::Value,
    pub outputs: serde_json::Value,
    pub error: Option<String>,
    pub meta: serde_json::Value,
}

fn kind_label(k: TraceKind) -> &'static str {
    match k {
        TraceKind::Allow => "allow",
        TraceKind::Deny => "deny",
        TraceKind::Replay => "replay",
        TraceKind::ExecutionFailure => "execution_failure",
        TraceKind::GapLockHeld => "gap_lock",
        TraceKind::Kill => "kill",
    }
}

fn class_label(c: ClaimClass) -> &'static str {
    match c {
        ClaimClass::Established => "established",
        ClaimClass::Experimental => "experimental",
        ClaimClass::Symbolic => "symbolic",
        ClaimClass::Prohibited => "prohibited",
    }
}

fn error_field(kind: TraceKind, reason: &str) -> Option<String> {
    match kind {
        TraceKind::Allow => None,
        _ => Some(reason.to_string()),
    }
}

/// Allow-listed metadata only. No prompts, tokens, or raw errors.
pub fn map_row(event: &TraceEvent) -> LiveTraceRow {
    LiveTraceRow {
        event: kind_label(event.kind).into(),
        gaian_id: Some(event.actor_id.clone()),
        correlation_id: Some(event.correlation_id.clone()),
        canon_refs: Vec::new(),
        started_at: None,
        ended_at: None,
        latency_ms: None,
        inputs: serde_json::json!({}),
        outputs: serde_json::json!({}),
        error: error_field(event.kind, event.reason.as_str()),
        meta: serde_json::json!({
            "schema": "gaia.trace_events.v1",
            "source": "local-acp",
            "intent_id": event.intent_id,
            "outcome": event.outcome,
            "reason_code": event.reason,
            "request_hash": event.request_hash,
            "claim_class": class_label(event.claim_class)
        }),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveSendError {
    Disabled,
    Transport,
}

pub trait LiveTraceTransport {
    fn send(&mut self, row: &LiveTraceRow) -> Result<(), LiveSendError>;
}

/// Records mapped rows. Never holds a credential.
#[derive(Debug, Default)]
pub struct RecordingLiveTransport {
    pub rows: Vec<LiveTraceRow>,
    pub fail_next: bool,
}

impl LiveTraceTransport for RecordingLiveTransport {
    fn send(&mut self, row: &LiveTraceRow) -> Result<(), LiveSendError> {
        if self.fail_next {
            self.fail_next = false;
            return Err(LiveSendError::Transport);
        }
        self.rows.push(row.clone());
        Ok(())
    }
}

/// Local sink is authoritative. Live failure does not roll back local events.
pub fn try_forward(
    cfg: &LiveTraceConfig,
    transport: &mut impl LiveTraceTransport,
    event: &TraceEvent,
) -> Result<(), LiveSendError> {
    if cfg.mode == LiveTraceMode::Off {
        return Err(LiveSendError::Disabled);
    }
    if !matches!(cfg.role, LiveTraceRole::TestBoundary) {
        return Err(LiveSendError::Disabled);
    }
    transport.send(&map_row(event))
}

/// Credential must never appear in Debug/Display of this crate.
pub fn credential_is_absent(debug_blob: &str) -> bool {
    let banned = ["SERVICE_ROLE", "eyJ", "supabase.co", "Bearer ", "sb_secret"];
    !banned.iter().any(|s| debug_blob.contains(s))
}
