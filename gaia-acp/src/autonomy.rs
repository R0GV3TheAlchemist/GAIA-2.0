//! Local autonomy caps and confirmation domains (#220, #187).
//! No live plant, money rail, or peer vault dump.

use crate::approval::HumanApprovalReceipt;
use crate::types::{ActionClass, ProposedAction, ReasonCode};

/// Software-enforced levels. 4–5 exist only as a named refuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AutonomyLevel {
    Observe = 0,
    Suggest = 1,
    LocalAct = 2,
    BoundedRemediate = 3,
    PlantAct = 4,
    Unbounded = 5,
}

impl AutonomyLevel {
    pub fn default_level() -> Self {
        AutonomyLevel::Suggest
    }

    pub fn software_max() -> Self {
        AutonomyLevel::BoundedRemediate
    }

    pub fn requires_written_artifact(self) -> bool {
        self >= AutonomyLevel::PlantAct
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmDomain {
    Money,
    Legal,
    Medical,
    Irreversible,
    OutboundMessage,
    LifeSafety,
    Vault,
}

impl ConfirmDomain {
    pub fn from_action(action: &ProposedAction) -> Option<Self> {
        let t = action.target.to_ascii_lowercase();
        let tool = action.tool.to_ascii_lowercase();
        if tool.contains("vault") || t.contains("/vault") || t.contains("memory-dump") {
            return Some(ConfirmDomain::Vault);
        }
        if matches!(action.action_class, ActionClass::Destructive) || t.contains("life-safety") {
            return Some(ConfirmDomain::LifeSafety);
        }
        if t.contains("payment") || t.contains("wire") || tool.contains("ledger") {
            return Some(ConfirmDomain::Money);
        }
        if t.contains("legal") || tool.contains("contract") {
            return Some(ConfirmDomain::Legal);
        }
        if t.contains("medical") || t.contains("/phi/") {
            return Some(ConfirmDomain::Medical);
        }
        if matches!(action.action_class, ActionClass::MergeDeployPublish) {
            return Some(ConfirmDomain::Irreversible);
        }
        if matches!(action.action_class, ActionClass::ExternalWrite) {
            return Some(ConfirmDomain::OutboundMessage);
        }
        None
    }
}

/// Peer envelope: identity + intent + purpose. Never raw memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerEnvelope {
    pub did: String,
    pub intent_id: String,
    pub purpose: String,
    pub raw_memory: bool,
    /// Specialist agents report only to Core (#220).
    pub specialist: bool,
}

impl PeerEnvelope {
    pub fn validate(&self) -> Result<(), ReasonCode> {
        if self.did.is_empty() || self.intent_id.is_empty() || self.purpose.is_empty() {
            return Err(ReasonCode::Malformed);
        }
        if self.raw_memory {
            return Err(ReasonCode::VaultDumpDenied);
        }
        if self.purpose.eq_ignore_ascii_case("dump-vault") {
            return Err(ReasonCode::VaultDumpDenied);
        }
        if self.specialist {
            let p = self.purpose.to_ascii_lowercase();
            if p.contains("command") || p.contains("peer-direct") || p.contains("dump") {
                return Err(ReasonCode::CrossAgent);
            }
        }
        Ok(())
    }
}

pub fn gate(
    level: AutonomyLevel,
    action: &ProposedAction,
    approval: Option<&HumanApprovalReceipt>,
) -> Result<(), ReasonCode> {
    if level.requires_written_artifact() {
        return Err(ReasonCode::AutonomyCap);
    }
    if level < AutonomyLevel::LocalAct
        && matches!(
            action.action_class,
            ActionClass::RepoWrite
                | ActionClass::ExternalWrite
                | ActionClass::NetworkEgress
                | ActionClass::MergeDeployPublish
                | ActionClass::Destructive
        )
    {
        return Err(ReasonCode::AutonomyCap);
    }
    if let Some(domain) = ConfirmDomain::from_action(action) {
        match domain {
            ConfirmDomain::LifeSafety | ConfirmDomain::Vault => {
                if approval.is_none() {
                    return Err(if domain == ConfirmDomain::LifeSafety {
                        ReasonCode::LifeSafetyDenied
                    } else {
                        ReasonCode::VaultDumpDenied
                    });
                }
            }
            ConfirmDomain::Money
            | ConfirmDomain::Legal
            | ConfirmDomain::Medical
            | ConfirmDomain::Irreversible
            | ConfirmDomain::OutboundMessage => {
                if approval.is_none() {
                    return Err(ReasonCode::ConfirmRequired);
                }
            }
        }
    }
    Ok(())
}
