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
    /// Facts are constructed here from typed inputs. Agent/model text is not a fact source.
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
            return PolicyDecision::Deny {
                reason: ReasonCode::EmergencyStop,
            };
        }
        if action.tool.is_empty() || action.target.is_empty() {
            return PolicyDecision::Deny {
                reason: ReasonCode::Malformed,
            };
        }
        if revoked.is_revoked(&action.agent_id) || revoked.is_revoked(&manifest.agent_id) {
            return PolicyDecision::Deny {
                reason: ReasonCode::Revoked,
            };
        }
        if action.agent_id != manifest.agent_id {
            return PolicyDecision::Deny {
                reason: ReasonCode::CrossAgent,
            };
        }
        if now >= intent.expires_at || manifest.expired(now) {
            return PolicyDecision::Deny {
                reason: ReasonCode::Expired,
            };
        }
        if manifest.budget_exceeded() {
            return PolicyDecision::Deny {
                reason: ReasonCode::BudgetExceeded,
            };
        }
        if let Some(u) = untrusted {
            if u.contains_authority_claim() {
                return PolicyDecision::Deny {
                    reason: ReasonCode::UntrustedAuthority,
                };
            }
        }
        if !manifest.tool_allowed(&action.tool) {
            return PolicyDecision::Deny {
                reason: ReasonCode::ToolNotListed,
            };
        }
        if action.target.contains("..") || action.target.contains('\0') {
            return PolicyDecision::Deny {
                reason: ReasonCode::TraversalDenied,
            };
        }
        if is_protected_path(&action.target) {
            return PolicyDecision::Deny {
                reason: ReasonCode::ProtectedPath,
            };
        }
        if matches!(
            action.action_class,
            ActionClass::LocalRead | ActionClass::ScratchWrite | ActionClass::RepoWrite
        ) && !manifest.path_allowed(&action.target)
        {
            return PolicyDecision::Deny {
                reason: ReasonCode::PathDenied,
            };
        }
        if action.action_class == ActionClass::IdentityCreate {
            return PolicyDecision::Deny {
                reason: ReasonCode::IdentityCreateDenied,
            };
        }
        if action.action_class == ActionClass::SecretAccess {
            return PolicyDecision::Deny {
                reason: ReasonCode::SecretDenied,
            };
        }
        if action.action_class.agent_forbidden() || !manifest.risk_allowed(action.action_class) {
            return PolicyDecision::Deny {
                reason: ReasonCode::TierForbidden,
            };
        }
        if action.action_class == ActionClass::NetworkEgress {
            match classify_destination(&action.target) {
                EgressClass::ForbiddenSsrf => {
                    return PolicyDecision::Deny {
                        reason: ReasonCode::SsrfDenied,
                    }
                }
                EgressClass::PublicOrUnknown => {
                    if !manifest.dest_allowed(&action.target) {
                        return PolicyDecision::Deny {
                            reason: ReasonCode::EgressDenied,
                        };
                    }
                }
                EgressClass::Allowlisted => {}
            }
        }

        if action.action_class.requires_approval() {
            return match approval {
                None => PolicyDecision::Deny {
                    reason: ReasonCode::ApprovalMissing,
                },
                Some(r) => match r.validate(now, intent, manifest, action, revoked, consumed_approvals)
                {
                    Ok(()) => PolicyDecision::Allow {
                        reason: ReasonCode::Allow,
                    },
                    Err(reason) => PolicyDecision::Deny { reason },
                },
            };
        }

        PolicyDecision::Allow {
            reason: ReasonCode::Allow,
        }
    }
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
