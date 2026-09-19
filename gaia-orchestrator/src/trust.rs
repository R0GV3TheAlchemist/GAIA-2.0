//! Intent signing and audit via the Phase 1 kernel Ed25519 identity and hash-chained log.

use gaia_kernel::audit::AuditLog;
use gaia_kernel::identity::{self, Principal, PrincipalKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::intent::IntentGraph;
use crate::trace::{NoopSink, TraceEvent, TraceEventSink};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedIntent {
    pub intent_id: Uuid,
    pub canonical_payload: String,
    pub signature: String,
    pub algorithm: String,
    pub public_hex: String,
    pub issuer_did: String,
}

pub struct IntentSigner {
    principal: Principal,
}

impl IntentSigner {
    pub fn generate() -> Self {
        Self {
            principal: Principal::generate(PrincipalKind::Service),
        }
    }

    pub fn from_principal(principal: Principal) -> Self {
        Self { principal }
    }

    pub fn principal(&self) -> &Principal {
        &self.principal
    }

    pub fn canonical_payload(graph: &IntentGraph) -> String {
        let nodes = graph
            .sub_intents
            .iter()
            .map(|n| format!("{}:{}", n.id, n.goal))
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "intent_id={}\ngoal={}\nprivacy={:?}\ncompute={:?}\nnodes={}",
            graph.id, graph.goal, graph.constraints.privacy, graph.constraints.compute, nodes
        )
    }

    pub fn sign(&self, graph: &IntentGraph) -> SignedIntent {
        let canonical_payload = Self::canonical_payload(graph);
        let signature = hex::encode(self.principal.sign(canonical_payload.as_bytes()));
        SignedIntent {
            intent_id: graph.id,
            canonical_payload,
            signature,
            algorithm: "ed25519".into(),
            public_hex: self.principal.public_hex(),
            issuer_did: self.principal.did(),
        }
    }

    pub fn verify(&self, signed: &SignedIntent) -> Result<(), String> {
        Self::verify_detached(signed)
    }

    pub fn verify_detached(signed: &SignedIntent) -> Result<(), String> {
        if signed.algorithm != "ed25519" {
            return Err("unsupported signature algorithm".into());
        }
        let Ok(sig) = hex::decode(&signed.signature) else {
            return Err("invalid signature encoding".into());
        };
        if !identity::verify(
            &signed.public_hex,
            signed.canonical_payload.as_bytes(),
            &sig,
        ) {
            return Err("unsigned or tampered intent rejected".into());
        }
        Ok(())
    }

    pub fn sign_bytes(&self, payload: &[u8]) -> String {
        format!(
            "ed25519:{}:{}",
            self.principal.public_hex(),
            hex::encode(self.principal.sign(payload))
        )
    }
}

pub fn verify_tagged_signature(payload: &[u8], tagged: &str) -> Result<(), String> {
    let mut parts = tagged.splitn(3, ':');
    let alg = parts.next().unwrap_or_default();
    let public_hex = parts.next().unwrap_or_default();
    let signature = parts.next().unwrap_or_default();
    if alg != "ed25519" || public_hex.is_empty() || signature.is_empty() {
        return Err("unsigned or malformed signature rejected".into());
    }
    let Ok(sig) = hex::decode(signature) else {
        return Err("invalid signature encoding".into());
    };
    if !identity::verify(public_hex, payload, &sig) {
        return Err("unsigned or tampered payload rejected".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEvent {
    pub sequence: u64,
    pub intent_id: Uuid,
    pub plan_id: Option<Uuid>,
    pub executor_id: Option<String>,
    pub event: String,
}

/// Hash-chained, Ed25519-signed audit. Events() remains the orchestrator view.
/// Optional telemetry sink is best-effort and never authoritative.
pub struct TrustAudit {
    principal: Principal,
    log: AuditLog,
    events: Vec<AuditEvent>,
    sink: Box<dyn TraceEventSink>,
}

impl Default for TrustAudit {
    fn default() -> Self {
        Self::generate()
    }
}

impl TrustAudit {
    pub fn generate() -> Self {
        Self {
            principal: Principal::generate(PrincipalKind::Service),
            log: AuditLog::default(),
            events: Vec::new(),
            sink: Box::new(NoopSink),
        }
    }

    pub fn with_sink(sink: Box<dyn TraceEventSink>) -> Self {
        Self {
            principal: Principal::generate(PrincipalKind::Service),
            log: AuditLog::default(),
            events: Vec::new(),
            sink,
        }
    }

    pub fn append(
        &mut self,
        intent_id: Uuid,
        plan_id: Option<Uuid>,
        executor_id: Option<&str>,
        event: impl Into<String>,
    ) -> AuditEvent {
        let event = event.into();
        let entry = AuditEvent {
            sequence: self.events.len() as u64 + 1,
            intent_id,
            plan_id,
            executor_id: executor_id.map(str::to_owned),
            event: event.clone(),
        };
        let detail = format!(
            "intent={} plan={} executor={} event={}",
            intent_id,
            plan_id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "-".into()),
            executor_id.unwrap_or("-"),
            event
        );
        self.log.append(&self.principal, "orchestrator", &detail);
        self.events.push(entry.clone());
        let trace = TraceEvent::classify(&event, intent_id, plan_id, executor_id, entry.sequence);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.sink.emit(trace);
        }));
        entry
    }

    pub fn emit_trace(&self, event: TraceEvent) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.sink.emit(event);
        }));
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    pub fn chain_ok(&self) -> bool {
        self.log.chain_ok()
    }

    pub fn kernel_len(&self) -> usize {
        self.log.len()
    }
}
