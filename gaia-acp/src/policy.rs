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

/// All inputs required for a single policy evaluation.
///
/// Replaces the previous 9-argument flat signature on
/// [`PolicyEngine::evaluate`]. The policy engine's input domain was already
/// structured — this type formally models it, consistent with `HashInput` in
/// `audit.rs` and `MemStoreParams` in `gaia-earth`.
pub struct PolicyEvaluationContext<'a> {
    /// Current Unix timestamp (seconds). Used for expiry and not-yet-valid
    /// checks against both the intent and the capability manifest.
    pub now: u64,
    /// The signed intent driving this evaluation.
    pub intent: &'a SignedIntent,
    /// Capability manifest issued to the requesting agent.
    pub manifest: &'a CapabilityManifest,
    /// The specific action being proposed.
    pub action: &'a ProposedAction,
    /// Human approval receipt, if the action class requires one.
    pub approval: Option<&'a HumanApprovalReceipt>,
    /// Current revocation list.
    pub revoked: &'a RevocationList,
    /// When `true`, all evaluations return `Deny { EmergencyStop }` immediately.
    pub emergency_stop: bool,
    /// Approval tokens already consumed in this session, used to prevent replay.
    pub consumed_approvals: &'a [String],
    /// Untrusted content associated with this call, if any.
    pub untrusted: Option<&'a UntrustedContent>,
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(ctx: PolicyEvaluationContext<'_>) -> PolicyDecision {
        if ctx.emergency_stop {
            return deny(ReasonCode::EmergencyStop);
        }
        if ctx.action.tool.is_empty() || ctx.action.target.is_empty() {
            return deny(ReasonCode::Malformed);
        }
        if ctx.revoked.is_revoked(&ctx.action.agent_id)
            || ctx.revoked.is_revoked(&ctx.manifest.agent_id)
            || ctx.revoked.is_revoked(&ctx.manifest.manifest_id)
            || ctx.revoked.is_revoked(&ctx.manifest.gateway_id)
            || ctx.revoked.is_revoked(&ctx.manifest.server_id)
            || ctx.revoked.is_revoked(&ctx.manifest.issuer_id)
        {
            return deny(ReasonCode::Revoked);
        }
        if ctx.action.agent_id != ctx.manifest.agent_id {
            return deny(ReasonCode::CrossAgent);
        }
        if ctx.action.gateway_id != ctx.manifest.gateway_id
            || ctx.action.server_id != ctx.manifest.server_id
            || ctx.action.resource_id != ctx.manifest.resource_id
        {
            return deny(ReasonCode::ContextMismatch);
        }
        if !ctx.action.nonce.is_empty() && ctx.action.nonce != ctx.manifest.nonce {
            return deny(ReasonCode::NonceMismatch);
        }
        if ctx.action.wants_delegation && !ctx.manifest.allow_delegation {
            return deny(ReasonCode::DelegationDenied);
        }
        if ctx.manifest.not_yet_valid(ctx.now) {
            return deny(ReasonCode::NotYetValid);
        }
        if ctx.now >= ctx.intent.expires_at || ctx.manifest.expired(ctx.now) {
            return deny(ReasonCode::Expired);
        }
        if ctx.manifest.budget_exceeded() {
            return deny(ReasonCode::BudgetExceeded);
        }
        if let Some(u) = ctx.untrusted {
            if u.contains_authority_claim() {
                return deny(ReasonCode::UntrustedAuthority);
            }
        }
        if !ctx.manifest.tool_allowed(&ctx.action.tool) {
            return deny(ReasonCode::ToolNotListed);
        }
        if ctx.action.target.contains("..") || ctx.action.target.contains('\0') {
            return deny(ReasonCode::TraversalDenied);
        }
        if is_protected_path(&ctx.action.target) {
            return deny(ReasonCode::ProtectedPath);
        }
        if matches!(
            ctx.action.action_class,
            ActionClass::LocalRead | ActionClass::ScratchWrite | ActionClass::RepoWrite
        ) && !ctx.manifest.path_allowed(&ctx.action.target)
        {
            return deny(ReasonCode::PathDenied);
        }
        if ctx.action.action_class == ActionClass::IdentityCreate {
            return deny(ReasonCode::IdentityCreateDenied);
        }
        if ctx.action.action_class == ActionClass::SecretAccess {
            return deny(ReasonCode::SecretDenied);
        }
        if ctx.action.action_class.agent_forbidden()
            || !ctx.manifest.risk_allowed(ctx.action.action_class)
        {
            return deny(ReasonCode::TierForbidden);
        }
        if ctx.action.action_class == ActionClass::NetworkEgress {
            match classify_destination(&ctx.action.target) {
                EgressClass::ForbiddenSsrf => return deny(ReasonCode::SsrfDenied),
                EgressClass::PublicOrUnknown => {
                    if !ctx.manifest.dest_allowed(&ctx.action.target) {
                        return deny(ReasonCode::EgressDenied);
                    }
                }
                EgressClass::Allowlisted => {}
            }
        }

        if ctx.action.action_class.requires_approval() {
            return match ctx.approval {
                None => deny(ReasonCode::ApprovalMissing),
                Some(r) => {
                    match r.validate(
                        ctx.now,
                        ctx.intent,
                        ctx.manifest,
                        ctx.action,
                        ctx.revoked,
                        ctx.consumed_approvals,
                    ) {
                        Ok(()) => PolicyDecision::Allow {
                            reason: ReasonCode::Allow,
                        },
                        Err(reason) => deny(reason),
                    }
                }
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
