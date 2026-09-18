use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::types::ReasonCode;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaneState {
    Unregistered,
    Registered,
    Verified,
    SessionStarted,
    ManifestIssued,
    Stopped,
    Killed,
}

impl PlaneState {
    pub fn can_transition(self, next: PlaneState) -> bool {
        use PlaneState::*;
        matches!(
            (self, next),
            (Unregistered, Registered)
                | (Registered, Verified)
                | (Verified, SessionStarted)
                | (SessionStarted, ManifestIssued)
                | (ManifestIssued, ManifestIssued)
                | (ManifestIssued, Stopped)
                | (ManifestIssued, Killed)
                | (SessionStarted, Stopped)
                | (SessionStarted, Killed)
                | (Stopped, Killed)
                | (_, Killed)
        )
    }

    pub fn allows_calls(self) -> bool {
        matches!(self, PlaneState::ManifestIssued)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaneEvent {
    ServerRegistered,
    ServerVerified,
    SessionStarted,
    ManifestIssued,
    ToolProposed,
    PolicyEvaluated,
    ApprovalRequested,
    ApprovalGranted,
    ApprovalDenied,
    CallAllowed,
    CallDenied,
    EgressDenied,
    ExecutionCompleted,
    ExecutionFailed,
    CredentialRevoked,
    ServerStopped,
    ServerKilled,
    AnomalyDetected,
    EmergencyStop,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionReceipt {
    pub sequence: u64,
    pub event: PlaneEvent,
    pub previous_hash: String,
    pub hash: String,
    pub agent_id: String,
    pub tool: String,
    pub action_class: String,
    pub request_hash: String,
    pub reason: String,
    pub outcome: String,
    pub policy_version: String,
}

impl ActionReceipt {
    pub fn serialize_public(&self) -> String {
        serde_json::to_string(self).expect("receipt json")
    }

    pub fn leaks_sensitive(&self, secrets: &[&str]) -> bool {
        let s = self.serialize_public();
        secrets.iter().any(|x| !x.is_empty() && s.contains(x))
    }
}

#[derive(Debug, Default)]
pub struct AuditChain {
    receipts: Vec<ActionReceipt>,
}

impl AuditChain {
    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    pub fn last_hash(&self) -> String {
        self.receipts
            .last()
            .map(|r| r.hash.clone())
            .unwrap_or_else(|| "0".into())
    }

    pub fn push(
        &mut self,
        event: PlaneEvent,
        agent_id: &str,
        tool: &str,
        action_class: &str,
        request_hash: &str,
        reason: ReasonCode,
        outcome: &str,
    ) -> ActionReceipt {
        let previous_hash = self.last_hash();
        let sequence = self.receipts.len() as u64 + 1;
        let mut hasher = Sha256::new();
        hasher.update(previous_hash.as_bytes());
        hasher.update(sequence.to_le_bytes());
        hasher.update(format!("{event:?}").as_bytes());
        hasher.update(agent_id.as_bytes());
        hasher.update(tool.as_bytes());
        hasher.update(request_hash.as_bytes());
        hasher.update(reason.as_str().as_bytes());
        let hash = hex::encode(hasher.finalize());
        let receipt = ActionReceipt {
            sequence,
            event,
            previous_hash,
            hash,
            agent_id: agent_id.into(),
            tool: tool.into(),
            action_class: action_class.into(),
            request_hash: request_hash.into(),
            reason: reason.as_str().into(),
            outcome: outcome.into(),
            policy_version: crate::policy::POLICY_VERSION.into(),
        };
        self.receipts.push(receipt.clone());
        receipt
    }

    pub fn receipts(&self) -> &[ActionReceipt] {
        &self.receipts
    }

    pub fn chain_ok(&self) -> bool {
        let mut prev = "0".to_string();
        for (i, r) in self.receipts.iter().enumerate() {
            if r.sequence != i as u64 + 1 || r.previous_hash != prev {
                return false;
            }
            prev = r.hash.clone();
        }
        true
    }
}
