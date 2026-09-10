//! PLACEHOLDER ONLY — this module is not cryptography and provides no security.
//! Replace `PlaceholderSigner` with the real gaia-kernel #19 identity/audit API.

use crate::intent::IntentGraph;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedIntent {
    pub intent_id: Uuid,
    pub canonical_payload: String,
    pub signature: String,
    pub algorithm: String,
}

pub struct PlaceholderSigner {
    key_label: String,
}

impl PlaceholderSigner {
    /// Deterministic stand-in, deliberately named to prevent a security claim.
    pub fn new(key_label: impl Into<String>) -> Self {
        Self { key_label: key_label.into() }
    }

    pub fn canonical_payload(graph: &IntentGraph) -> String {
        let nodes = graph
            .sub_intents
            .iter()
            .map(|n| format!("{}:{}", n.id, n.goal))
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "intent_id={}\\ngoal={}\\nprivacy={:?}\\ncompute={:?}\\nnodes={}",
            graph.id, graph.goal, graph.constraints.privacy, graph.constraints.compute, nodes
        )
    }

    pub fn sign(&self, graph: &IntentGraph) -> SignedIntent {
        let canonical_payload = Self::canonical_payload(graph);
        let signature = format!("PLACEHOLDER:{}:{}", self.key_label, checksum(&canonical_payload));
        SignedIntent {
            intent_id: graph.id,
            canonical_payload,
            signature,
            algorithm: "PLACEHOLDER-NOT-CRYPTOGRAPHY".into(),
        }
    }

    pub fn verify(&self, signed: &SignedIntent) -> Result<(), String> {
        if signed.algorithm != "PLACEHOLDER-NOT-CRYPTOGRAPHY" {
            return Err("unsupported placeholder algorithm".into());
        }
        let expected = format!("PLACEHOLDER:{}:{}", self.key_label, checksum(&signed.canonical_payload));
        if signed.signature != expected {
            return Err("unsigned or tampered intent rejected".into());
        }
        Ok(())
    }
}

fn checksum(input: &str) -> u64 {
    input
        .bytes()
        .fold(0xcbf29ce484222325_u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x100000001b3))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEvent {
    pub sequence: u64,
    pub intent_id: Uuid,
    pub plan_id: Option<Uuid>,
    pub executor_id: Option<String>,
    pub event: String,
}

/// Append-only in-memory placeholder; persistence belongs to kernel audit integration.
#[derive(Debug, Default)]
pub struct TrustAudit {
    events: Vec<AuditEvent>,
}

impl TrustAudit {
    pub fn append(
        &mut self,
        intent_id: Uuid,
        plan_id: Option<Uuid>,
        executor_id: Option<&str>,
        event: impl Into<String>,
    ) -> AuditEvent {
        let entry = AuditEvent {
            sequence: self.events.len() as u64 + 1,
            intent_id,
            plan_id,
            executor_id: executor_id.map(str::to_owned),
            event: event.into(),
        };
        self.events.push(entry.clone());
        entry
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }
}
