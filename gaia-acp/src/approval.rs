use serde::{Deserialize, Serialize};

use crate::manifest::{CapabilityManifest, RevocationList};
use crate::types::{ActionClass, ProposedAction, ReasonCode, SignedIntent};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalDecision {
    Grant,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumanApprovalReceipt {
    pub id: String,
    pub approver_id: String,
    pub intent_id: String,
    pub action_class: ActionClass,
    pub tool: String,
    pub target: String,
    pub request_hash: String,
    pub policy_version: String,
    pub expires_at: u64,
    pub single_use: bool,
    pub decision: ApprovalDecision,
}

impl HumanApprovalReceipt {
    pub fn grant_for(
        id: &str,
        approver_id: &str,
        intent: &SignedIntent,
        action: &ProposedAction,
        expires_at: u64,
    ) -> Self {
        Self {
            id: id.into(),
            approver_id: approver_id.into(),
            intent_id: intent.intent_id.clone(),
            action_class: action.action_class,
            tool: action.tool.clone(),
            target: action.target.clone(),
            request_hash: action.request_hash(),
            policy_version: crate::policy::POLICY_VERSION.into(),
            expires_at,
            single_use: true,
            decision: ApprovalDecision::Grant,
        }
    }

    pub fn validate(
        &self,
        now: u64,
        intent: &SignedIntent,
        manifest: &CapabilityManifest,
        action: &ProposedAction,
        revoked: &RevocationList,
        consumed: &[String],
    ) -> Result<(), ReasonCode> {
        if self.decision != ApprovalDecision::Grant {
            return Err(ReasonCode::ApprovalMissing);
        }
        if revoked.is_revoked(&self.id) || revoked.is_revoked(&self.approver_id) {
            return Err(ReasonCode::Revoked);
        }
        if now >= self.expires_at {
            return Err(ReasonCode::ApprovalExpired);
        }
        if self.single_use && consumed.iter().any(|id| id == &self.id) {
            return Err(ReasonCode::ApprovalReplay);
        }
        if self.intent_id != intent.intent_id {
            return Err(ReasonCode::ApprovalMismatch);
        }
        if self.tool != action.tool
            || self.target != action.target
            || self.action_class != action.action_class
            || self.request_hash != action.request_hash()
            || self.policy_version != manifest.policy_version
        {
            return Err(ReasonCode::ApprovalMismatch);
        }
        Ok(())
    }

    /// Conversational text is never an approval.
    pub fn from_chat(_text: &str) -> Result<Self, ReasonCode> {
        Err(ReasonCode::ApprovalMissing)
    }
}
