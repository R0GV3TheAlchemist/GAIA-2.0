//! Live-trace forwarding stub — local-dev / fake-MCP only (#335).
//! Real Supabase / HTTP is prohibited in this crate. All transport
//! implementations here are in-memory recording adapters.

use serde::{Deserialize, Serialize};

use crate::trace::{ClaimClass, TraceEvent};

// ── Transport abstraction ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiveTraceTransport {
    /// No forwarding — events are dropped after local recording.
    None,
    /// In-process recording bus used by tests.
    InMemory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiveTraceMode {
    Disabled,
    LocalDev,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiveTraceRole {
    Observer,
    Publisher,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiveTraceConfig {
    pub mode: LiveTraceMode,
    pub transport: LiveTraceTransport,
    pub role: LiveTraceRole,
}

impl Default for LiveTraceConfig {
    fn default() -> Self {
        Self {
            mode: LiveTraceMode::Disabled,
            transport: LiveTraceTransport::None,
            role: LiveTraceRole::Observer,
        }
    }
}

// ── Row ───────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiveTraceRow {
    pub ts: u64,
    pub actor_id: String,
    pub intent_id: String,
    pub correlation_id: String,
    pub outcome: String,
    pub reason: String,
    pub request_hash: String,
    pub claim_class: String,
}

pub fn map_row(event: &TraceEvent) -> LiveTraceRow {
    LiveTraceRow {
        ts: event.ts,
        actor_id: event.actor_id.clone(),
        intent_id: event.intent_id.clone(),
        correlation_id: event.correlation_id.clone(),
        outcome: event.outcome.clone(),
        reason: event.reason.clone(),
        request_hash: event.request_hash.clone(),
        claim_class: class_label(event.claim_class).into(),
    }
}

fn class_label(c: ClaimClass) -> &'static str {
    match c {
        ClaimClass::Established => "established",
        ClaimClass::Experimental => "experimental",
        ClaimClass::Symbolic => "symbolic",
        ClaimClass::Prohibited => "prohibited",
        ClaimClass::Synthetic => "synthetic",
        ClaimClass::Observed => "observed",
    }
}

// ── Send error ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveSendError {
    Disabled,
    TransportError(String),
}

impl std::fmt::Display for LiveSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveSendError::Disabled => write!(f, "live trace is disabled"),
            LiveSendError::TransportError(msg) => write!(f, "transport error: {msg}"),
        }
    }
}

// ── Recording transport ───────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct RecordingLiveTransport {
    pub rows: Vec<LiveTraceRow>,
}

impl RecordingLiveTransport {
    pub fn send(&mut self, row: LiveTraceRow) {
        self.rows.push(row);
    }
}

// ── try_forward ───────────────────────────────────────────────────────────────

/// Attempt to forward a trace event to a live transport.
///
/// Returns `Err(LiveSendError::Disabled)` when the config mode is `Disabled`.
/// In `LocalDev` mode with `InMemory` transport, maps and appends to the
/// recording transport. In all other configurations the event is dropped and
/// `Ok(())` is returned (fire-and-forget semantics for unknown transports).
pub fn try_forward(
    event: &TraceEvent,
    config: &LiveTraceConfig,
    sink: Option<&mut RecordingLiveTransport>,
) -> Result<(), LiveSendError> {
    if config.mode == LiveTraceMode::Disabled {
        return Err(LiveSendError::Disabled);
    }
    if config.transport == LiveTraceTransport::InMemory {
        if let Some(s) = sink {
            s.send(map_row(event));
        }
    }
    Ok(())
}

// ── Credential absence guard ──────────────────────────────────────────────────

/// Asserts that no live credential is present in the current process environment.
/// Returns `Err` with a description of the leaked variable if any is found.
pub fn credential_is_absent() -> Result<(), String> {
    let guarded = [
        "SUPABASE_URL",
        "SUPABASE_ANON_KEY",
        "SUPABASE_SERVICE_KEY",
        "DATABASE_URL",
        "POSTGRES_URL",
    ];
    for var in guarded {
        if let Ok(val) = std::env::var(var) {
            if !val.is_empty() {
                return Err(format!("{var} is set — live credentials must not be present"));
            }
        }
    }
    Ok(())
}
