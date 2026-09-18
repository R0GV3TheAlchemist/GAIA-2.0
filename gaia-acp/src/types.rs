use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskTier {
    T0 = 0,
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionClass {
    LocalParse,
    LocalRead,
    ScratchWrite,
    RepoWrite,
    ExternalRead,
    ExternalWrite,
    NetworkEgress,
    IdentityCreate,
    MergeDeployPublish,
    SecretAccess,
    Destructive,
}

impl ActionClass {
    pub fn risk(self) -> RiskTier {
        match self {
            ActionClass::LocalParse => RiskTier::T0,
            ActionClass::LocalRead => RiskTier::T1,
            ActionClass::ScratchWrite => RiskTier::T2,
            ActionClass::RepoWrite | ActionClass::ExternalRead => RiskTier::T2,
            ActionClass::ExternalWrite | ActionClass::NetworkEgress => RiskTier::T3,
            ActionClass::MergeDeployPublish => RiskTier::T4,
            ActionClass::IdentityCreate | ActionClass::SecretAccess | ActionClass::Destructive => {
                RiskTier::T5
            }
        }
    }

    pub fn requires_approval(self) -> bool {
        self.risk() as u8 >= RiskTier::T3 as u8
    }

    pub fn agent_forbidden(self) -> bool {
        matches!(
            self,
            ActionClass::IdentityCreate | ActionClass::SecretAccess | ActionClass::Destructive
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReasonCode {
    Allow,
    DefaultDeny,
    ToolNotListed,
    PathDenied,
    HostDenied,
    Expired,
    BudgetExceeded,
    Revoked,
    EmergencyStop,
    ApprovalRequired,
    ApprovalMissing,
    ApprovalMismatch,
    ApprovalReplay,
    ApprovalExpired,
    EgressDenied,
    SsrfDenied,
    TraversalDenied,
    ProtectedPath,
    IdentityCreateDenied,
    SecretDenied,
    TierForbidden,
    UntrustedAuthority,
    Malformed,
    StateInvalid,
    ConfigRejected,
    CrossAgent,
    ContextMismatch,
    NotYetValid,
    DelegationDenied,
    HashTampered,
    NonceMismatch,
}

impl ReasonCode {
    pub fn as_str(self) -> &'static str {
        match self {
            ReasonCode::Allow => "GAIA_ACP_ALLOW",
            ReasonCode::DefaultDeny => "GAIA_ACP_DEFAULT_DENY",
            ReasonCode::ToolNotListed => "GAIA_ACP_TOOL_NOT_LISTED",
            ReasonCode::PathDenied => "GAIA_ACP_PATH_DENIED",
            ReasonCode::HostDenied => "GAIA_ACP_HOST_DENIED",
            ReasonCode::Expired => "GAIA_ACP_EXPIRED",
            ReasonCode::BudgetExceeded => "GAIA_ACP_BUDGET_EXCEEDED",
            ReasonCode::Revoked => "GAIA_ACP_REVOKED",
            ReasonCode::EmergencyStop => "GAIA_ACP_EMERGENCY_STOP",
            ReasonCode::ApprovalRequired => "GAIA_ACP_APPROVAL_REQUIRED",
            ReasonCode::ApprovalMissing => "GAIA_ACP_APPROVAL_MISSING",
            ReasonCode::ApprovalMismatch => "GAIA_ACP_APPROVAL_MISMATCH",
            ReasonCode::ApprovalReplay => "GAIA_ACP_APPROVAL_REPLAY",
            ReasonCode::ApprovalExpired => "GAIA_ACP_APPROVAL_EXPIRED",
            ReasonCode::EgressDenied => "GAIA_ACP_EGRESS_DENIED",
            ReasonCode::SsrfDenied => "GAIA_ACP_SSRF_DENIED",
            ReasonCode::TraversalDenied => "GAIA_ACP_TRAVERSAL_DENIED",
            ReasonCode::ProtectedPath => "GAIA_ACP_PROTECTED_PATH",
            ReasonCode::IdentityCreateDenied => "GAIA_ACP_IDENTITY_CREATE_DENIED",
            ReasonCode::SecretDenied => "GAIA_ACP_SECRET_DENIED",
            ReasonCode::TierForbidden => "GAIA_ACP_TIER_FORBIDDEN",
            ReasonCode::UntrustedAuthority => "GAIA_ACP_UNTRUSTED_AUTHORITY",
            ReasonCode::Malformed => "GAIA_ACP_MALFORMED",
            ReasonCode::StateInvalid => "GAIA_ACP_STATE_INVALID",
            ReasonCode::ConfigRejected => "GAIA_ACP_CONFIG_REJECTED",
            ReasonCode::CrossAgent => "GAIA_ACP_CROSS_AGENT",
            ReasonCode::ContextMismatch => "GAIA_ACP_CONTEXT_MISMATCH",
            ReasonCode::NotYetValid => "GAIA_ACP_NOT_YET_VALID",
            ReasonCode::DelegationDenied => "GAIA_ACP_DELEGATION_DENIED",
            ReasonCode::HashTampered => "GAIA_ACP_HASH_TAMPERED",
            ReasonCode::NonceMismatch => "GAIA_ACP_NONCE_MISMATCH",
        }
    }
}

/// Trusted only when constructed by gateway / signer code, never from model text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedIntent {
    pub intent_id: String,
    pub goal_class: String,
    pub issued_at: u64,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustedPolicy {
    pub version: String,
    pub default_deny: bool,
}

/// Wrapper so untrusted bytes cannot be mistaken for policy facts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UntrustedContent {
    pub source: String,
    pub body: String,
}

impl UntrustedContent {
    pub fn contains_authority_claim(&self) -> bool {
        let b = self.body.to_ascii_lowercase();
        b.contains("allow tool")
            || b.contains("grant capability")
            || b.contains("approved")
            || b.contains("ignore previous")
            || b.contains("you are now authorized")
            || b.contains("set policy")
            || b.contains("add destination")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UntrustedToolOutput {
    pub tool: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProposedAction {
    pub agent_id: String,
    pub tool: String,
    pub method: String,
    pub target: String,
    pub action_class: ActionClass,
    pub payload: String,
    pub nonce: String,
    pub gateway_id: String,
    pub server_id: String,
    pub resource_id: String,
    pub wants_delegation: bool,
}

impl ProposedAction {
    pub fn request_hash(&self) -> String {
        let mut h = Sha256::new();
        h.update(self.agent_id.as_bytes());
        h.update(b"|");
        h.update(self.tool.as_bytes());
        h.update(b"|");
        h.update(self.method.as_bytes());
        h.update(b"|");
        h.update(self.target.as_bytes());
        h.update(b"|");
        h.update(format!("{:?}", self.action_class).as_bytes());
        h.update(b"|");
        h.update(self.payload.as_bytes());
        h.update(b"|");
        h.update(self.nonce.as_bytes());
        hex::encode(h.finalize())
    }
}
