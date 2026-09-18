use crate::approval::HumanApprovalReceipt;
use crate::manifest::{CapabilityManifest, RevocationList};
use crate::sandbox::{classify_destination, EgressClass};
use crate::types::{ActionClass, ProposedAction, ReasonCode, SignedIntent, UntrustedContent};

pub const POLICY_VERSION: &str = "gaia-acp-policy-v0.1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow { reason: ReasonCode },
    Deny { reason: ReasonCode },
    RequireApproval { reason: ReasonCode },
}

impl PolicyDecision {
    pub fn reason(&self) -> ReasonCode {
        match self {
            PolicyDecision::Allow { reason }
            | PolicyDecision::Deny { reason }
            | PolicyDecision::RequireApproval { reason } => *reason,
        }
    }

    pub fn is_allow(&self) -> bool {
        matches!(self, PolicyDecision::Allow { .. })
    }
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(
        now: u64,
        intent: &SignedIntent,
        manifest: &CapabilityManifest,
        action: &ProposedAction,
        approval: Option<&HumanApprovalReceipt>,
        revoked: &RevocationList,
        emergency_stop: bool,
        consumed_approvals: &[String],
        untrusted: Option<&UntrustedContent>,
    ) -> PolicyDecision {
        if emergency_stop {
            return deny(ReasonCode::EmergencyStop);
        }
        if action.tool.is_empty() || action.target.is_empty() {
            return deny(ReasonCode::Malformed);
        }
        if revoked.is_revoked(&action.agent_id)
            || revoked.is_revoked(&manifest.agent_id)
            || revoked.is_revoked(&manifest.manifest_id)
            || revoked.is_revoked(&manifest.gateway_id)
            || revoked.is_revoked(&manifest.server_id)
            || revoked.is_revoked(&manifest.issuer_id)
        {
            return deny(ReasonCode::Revoked);
        }
        if action.agent_id != manifest.agent_id {
            return deny(ReasonCode::CrossAgent);
        }
        if action.gateway_id != manifest.gateway_id
            || action.server_id != manifest.server_id
            || action.resource_id != manifest.resource_id
        {
            return deny(ReasonCode::ContextMismatch);
        }
        if !action.nonce.is_empty() && action.nonce != manifest.nonce {
            return deny(ReasonCode::NonceMismatch);
        }
        if action.wants_delegation && !manifest.allow_delegation {
            return deny(ReasonCode::DelegationDenied);
        }
        if manifest.not_yet_valid(now) {
            return deny(ReasonCode::NotYetValid);
        }
        if now >= intent.expires_at || manifest.expired(now) {
            return deny(ReasonCode::Expired);
        }
        if manifest.budget_exceeded() {
            return deny(ReasonCode::BudgetExceeded);
        }
        if let Some(u) = untrusted {
            if u.contains_authority_claim() {
                return deny(ReasonCode::UntrustedAuthority);
            }
        }
        if !manifest.tool_allowed(&action.tool) {
            return deny(ReasonCode::ToolNotListed);
        }
        if action.target.contains("..") || action.target.contains('\0') {
            return deny(ReasonCode::TraversalDenied);
        }
        if is_protected_path(&action.target) {
            return deny(ReasonCode::ProtectedPath);
        }
        if matches!(
            action.action_class,
            ActionClass::LocalRead | ActionClass::ScratchWrite | ActionClass::RepoWrite
        ) && !manifest.path_allowed(&action.target)
        {
            return deny(ReasonCode::PathDenied);
        }
        if action.action_class == ActionClass::IdentityCreate {
            return deny(ReasonCode::IdentityCreateDenied);
        }
        if action.action_class == ActionClass::SecretAccess {
            return deny(ReasonCode::SecretDenied);
        }
        if action.action_class.agent_forbidden() || !manifest.risk_allowed(action.action_class) {
            return deny(ReasonCode::TierForbidden);
        }
        if action.action_class == ActionClass::NetworkEgress {
            match classify_destination(&action.target) {
                EgressClass::ForbiddenSsrf => return deny(ReasonCode::SsrfDenied),
                EgressClass::PublicOrUnknown => {
                    if !manifest.dest_allowed(&action.target) {
                        return deny(ReasonCode::EgressDenied);
                    }
                }
                EgressClass::Allowlisted => {}
            }
        }

        if action.action_class.requires_approval() {
            return match approval {
                None => deny(ReasonCode::ApprovalMissing),
                Some(r) => match r.validate(now, intent, manifest, action, revoked, consumed_approvals) {
                    Ok(()) => PolicyDecision::Allow {
                        reason: ReasonCode::Allow,
                    },
                    Err(reason) => deny(reason),
                },
            };
        }

        PolicyDecision::Allow {
            reason: ReasonCode::Allow,
        }
    }
}

fn deny(reason: ReasonCode) -> PolicyDecision {
    PolicyDecision::Deny { reason }
}

fn is_protected_path(path: &str) -> bool {
    let p = path.to_ascii_lowercase();
    p.contains(".git/")
        || p.contains(".github/workflows")
        || p.ends_with(".env")
        || p.contains("id_rsa")
        || p.contains("/secrets/")
        || p == "/"
        || p.starts_with("/etc/")
}
