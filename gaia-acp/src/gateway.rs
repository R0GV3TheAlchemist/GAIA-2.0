use crate::approval::HumanApprovalReceipt;
use crate::audit::{ActionReceipt, AuditChain, PlaneEvent, PlaneState};
use crate::manifest::{CapabilityManifest, RevocationList};
use crate::policy::{PolicyDecision, PolicyEngine};
use crate::types::{ProposedAction, ReasonCode, SignedIntent, UntrustedContent};

#[derive(Debug)]
pub struct ControlPlane {
    pub now: u64,
    pub state: PlaneState,
    pub intent: SignedIntent,
    pub revoked: RevocationList,
    pub consumed_approvals: Vec<String>,
    pub emergency_stop: bool,
    pub audit: AuditChain,
    deny_streak: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeResult {
    pub allowed: bool,
    pub executed: bool,
    pub reason: ReasonCode,
    pub receipt: ActionReceipt,
}

impl ControlPlane {
    pub fn start(now: u64, agent_id: &str) -> Result<Self, ReasonCode> {
        let mut plane = Self {
            now,
            state: PlaneState::Unregistered,
            intent: SignedIntent {
                intent_id: format!("intent-{agent_id}"),
                goal_class: "local-research".into(),
                issued_at: now,
                expires_at: now + 3600,
            },
            revoked: RevocationList::default(),
            consumed_approvals: Vec::new(),
            emergency_stop: false,
            audit: AuditChain::default(),
            deny_streak: 0,
        };
        plane.transition(PlaneState::Registered, agent_id)?;
        plane.transition(PlaneState::Verified, agent_id)?;
        plane.transition(PlaneState::SessionStarted, agent_id)?;
        plane.transition(PlaneState::ManifestIssued, agent_id)?;
        Ok(plane)
    }

    pub fn transition(&mut self, next: PlaneState, agent_id: &str) -> Result<(), ReasonCode> {
        if !self.state.can_transition(next) {
            return Err(ReasonCode::StateInvalid);
        }
        self.state = next;
        let ev = match next {
            PlaneState::Registered => PlaneEvent::ServerRegistered,
            PlaneState::Verified => PlaneEvent::ServerVerified,
            PlaneState::SessionStarted => PlaneEvent::SessionStarted,
            PlaneState::ManifestIssued => PlaneEvent::ManifestIssued,
            PlaneState::Stopped => PlaneEvent::ServerStopped,
            PlaneState::Killed => PlaneEvent::ServerKilled,
            PlaneState::Unregistered => PlaneEvent::AnomalyDetected,
        };
        self.audit.push(
            ev,
            agent_id,
            "-",
            "lifecycle",
            "0",
            ReasonCode::Allow,
            "state",
        );
        Ok(())
    }

    pub fn revoke(&mut self, id: &str) {
        self.revoked.revoke(id);
        self.audit.push(
            PlaneEvent::CredentialRevoked,
            id,
            "-",
            "revoke",
            "0",
            ReasonCode::Revoked,
            "revoked",
        );
    }

    pub fn kill(&mut self, agent_id: &str) {
        self.emergency_stop = true;
        let _ = self.transition(PlaneState::Killed, agent_id);
        self.audit.push(
            PlaneEvent::EmergencyStop,
            agent_id,
            "-",
            "stop",
            "0",
            ReasonCode::EmergencyStop,
            "killed",
        );
    }

    /// Sole fake-adapter invocation path.
    pub fn invoke(
        &mut self,
        manifest: &mut CapabilityManifest,
        action: &ProposedAction,
        approval: Option<&HumanApprovalReceipt>,
        untrusted: Option<&UntrustedContent>,
    ) -> InvokeResult {
        self.audit.push(
            PlaneEvent::ToolProposed,
            &action.agent_id,
            &action.tool,
            format!("{:?}", action.action_class).as_str(),
            &action.request_hash(),
            ReasonCode::Allow,
            "proposed",
        );

        if !self.state.allows_calls() {
            let receipt = self.deny(action, ReasonCode::StateInvalid);
            return InvokeResult {
                allowed: false,
                executed: false,
                reason: ReasonCode::StateInvalid,
                receipt,
            };
        }

        let decision = PolicyEngine::evaluate(
            self.now,
            &self.intent,
            manifest,
            action,
            approval,
            &self.revoked,
            self.emergency_stop,
            &self.consumed_approvals,
            untrusted,
        );
        self.audit.push(
            PlaneEvent::PolicyEvaluated,
            &action.agent_id,
            &action.tool,
            format!("{:?}", action.action_class).as_str(),
            &action.request_hash(),
            decision.reason(),
            "evaluated",
        );

        match decision {
            PolicyDecision::Allow { reason } => {
                if let Some(a) = approval {
                    if a.single_use {
                        self.consumed_approvals.push(a.id.clone());
                    }
                }
                manifest.actions_used = manifest.actions_used.saturating_add(1);
                self.deny_streak = 0;
                let receipt = self.audit.push(
                    PlaneEvent::CallAllowed,
                    &action.agent_id,
                    &action.tool,
                    format!("{:?}", action.action_class).as_str(),
                    &action.request_hash(),
                    reason,
                    "allowed",
                );
                InvokeResult {
                    allowed: true,
                    executed: true,
                    reason,
                    receipt,
                }
            }
            PolicyDecision::Deny { reason } | PolicyDecision::RequireApproval { reason } => {
                InvokeResult {
                    allowed: false,
                    executed: false,
                    reason,
                    receipt: self.deny(action, reason),
                }
            }
        }
    }

    fn deny(&mut self, action: &ProposedAction, reason: ReasonCode) -> ActionReceipt {
        self.deny_streak += 1;
        if self.deny_streak >= 5 {
            self.audit.push(
                PlaneEvent::AnomalyDetected,
                &action.agent_id,
                &action.tool,
                "anomaly",
                &action.request_hash(),
                reason,
                "repeated-deny",
            );
        }
        let ev = if matches!(
            reason,
            ReasonCode::EgressDenied | ReasonCode::SsrfDenied
        ) {
            PlaneEvent::EgressDenied
        } else {
            PlaneEvent::CallDenied
        };
        self.audit.push(
            ev,
            &action.agent_id,
            &action.tool,
            format!("{:?}", action.action_class).as_str(),
            &action.request_hash(),
            reason,
            "denied",
        )
    }
}
