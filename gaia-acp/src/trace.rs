//! Local trace + gap-lock boundary (#335, #375).
//! No live Supabase, HTTP, or credentials.

use serde::{Deserialize, Serialize};

use crate::types::ReasonCode;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClaimClass {
    #[default]
    Established,
    Experimental,
    Symbolic,
    Prohibited,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TraceKind {
    Allow,
    Deny,
    Replay,
    ExecutionFailure,
    GapLockHeld,
    Kill,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceEvent {
    pub kind: TraceKind,
    pub ts: u64,
    pub actor_id: String,
    pub intent_id: String,
    pub correlation_id: String,
    pub outcome: String,
    pub reason: String,
    pub request_hash: String,
    pub claim_class: ClaimClass,
}

impl TraceEvent {
    pub fn leaks(&self, secrets: &[&str]) -> bool {
        let blob = format!(
            "{}{}{}{}{}",
            self.actor_id, self.intent_id, self.correlation_id, self.outcome, self.request_hash
        );
        secrets.iter().any(|s| !s.is_empty() && blob.contains(s))
    }
}

pub trait TraceSink {
    fn emit(&mut self, event: TraceEvent);
}

#[derive(Debug, Default)]
pub struct MemoryTraceSink {
    pub events: Vec<TraceEvent>,
}

impl TraceSink for MemoryTraceSink {
    fn emit(&mut self, event: TraceEvent) {
        self.events.push(event);
    }
}

impl MemoryTraceSink {
    pub fn kinds(&self) -> Vec<TraceKind> {
        self.events.iter().map(|e| e.kind).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GapLock {
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateMode {
    FailClosed,
    LocalDev,
}

pub fn execution_allowed(lock: Option<GapLock>, mode: GateMode) -> Result<(), ReasonCode> {
    match (lock, mode) {
        (Some(GapLock { active: true }), _) => Err(ReasonCode::StateInvalid),
        (None, GateMode::FailClosed) => Err(ReasonCode::StateInvalid),
        (None, GateMode::LocalDev) => Ok(()),
        (Some(GapLock { active: false }), _) => Ok(()),
    }
}

pub fn refuse_live_supabase() -> Result<(), &'static str> {
    Err("trace sink is local-only; no live Supabase write")
}

/// Prohibited claims cannot be emitted as an allow (#375).
pub fn from_invoke(
    kind: TraceKind,
    ts: u64,
    actor_id: &str,
    intent_id: &str,
    correlation_id: &str,
    reason: ReasonCode,
    request_hash: &str,
    claim_class: ClaimClass,
) -> TraceEvent {
    let kind = if matches!(kind, TraceKind::Allow) && matches!(claim_class, ClaimClass::Prohibited)
    {
        TraceKind::Deny
    } else {
        kind
    };
    TraceEvent {
        kind,
        ts,
        actor_id: actor_id.into(),
        intent_id: intent_id.into(),
        correlation_id: correlation_id.into(),
        outcome: match kind {
            TraceKind::Allow => "allowed",
            TraceKind::Deny => "denied",
            TraceKind::Replay => "replay-denied",
            TraceKind::ExecutionFailure => "failed",
            TraceKind::GapLockHeld => "blocked",
            TraceKind::Kill => "killed",
        }
        .into(),
        reason: reason.as_str().into(),
        request_hash: request_hash.into(),
        claim_class,
    }
}
