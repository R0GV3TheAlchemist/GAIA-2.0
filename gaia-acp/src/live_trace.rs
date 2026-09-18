//! Feature-gated live trace mapper (#335).
//! Default path: off. No HTTP client. No credentials in this crate.

use crate::trace::{ClaimClass, TraceEvent, TraceKind};
use serde::{Deserialize, Serialize};

/// Isolated data boundary. Production service role is never constructed in tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveTraceRole {
    TestBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveTraceMode {
    Off,
    MappedOnly,
}

impl Default for LiveTraceMode {
    fn default() -> Self {
        LiveTraceMode::Off
    }
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

/// Row that would land on public.trace_events. Inputs/outputs stay empty objects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiveTraceRow {
    pub kind: String,
    pub actor_id: String,
    pub intent_id: String,
    pub correlation_id: String,
    pub outcome: String,
    pub reason_code: String,
    pub request_hash: String,
    pub claim_class: String,
    pub inputs: serde_json::Value,
    pub outputs: serde_json::Value,
    pub metadata: serde_json::Value,
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

/// Allow-listed metadata only. No prompts, tokens, or raw errors.
pub fn map_row(event: &TraceEvent) -> LiveTraceRow {
    LiveTraceRow {
        kind: kind_label(event.kind).into(),
        actor_id: event.actor_id.clone(),
        intent_id: event.intent_id.clone(),
        correlation_id: event.correlation_id.clone(),
        outcome: event.outcome.clone(),
        reason_code: event.reason.clone(),
        request_hash: event.request_hash.clone(),
        claim_class: class_label(event.claim_class).into(),
        inputs: serde_json::json!({}),
        outputs: serde_json::json!({}),
        metadata: serde_json::json!({
            "schema": "gaia.trace_events.v1",
            "source": "local-acp"
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
