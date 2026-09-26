//! Tool invocation authorization (#933).

use crate::tool_audit::{ToolAuditEntry, ToolAuditLog, ToolOutcome};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalToken(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolPermissionTier {
    Unrestricted,
    AgentRestricted,
    ElevationRequired,
    Dangerous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    PermissionDenied { agent_id: String, tool_id: String },
    ElevationRequired,
    HumanApprovalRequired,
    ToolUnknown,
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub agent_id: AgentId,
    pub tool_id: ToolId,
    pub allowed_agents: Option<Vec<AgentId>>,
    pub tier: ToolPermissionTier,
    pub explicit_approval: bool,
    pub human_approval_token: Option<ApprovalToken>,
    pub params: String,
}

pub fn authorize(call: &ToolCall, log: &mut ToolAuditLog) -> Result<ToolAuditEntry, AuthError> {
    let params_hash = {
        let mut h = Sha256::new();
        h.update(call.params.as_bytes());
        hex::encode(h.finalize())
    };

    let result = match call.tier {
        ToolPermissionTier::Unrestricted => Ok(ToolOutcome::Permitted),
        ToolPermissionTier::AgentRestricted => {
            let allowed = call
                .allowed_agents
                .as_ref()
                .is_some_and(|ids| ids.contains(&call.agent_id));
            if allowed {
                Ok(ToolOutcome::Permitted)
            } else {
                Err(AuthError::PermissionDenied {
                    agent_id: call.agent_id.0.clone(),
                    tool_id: call.tool_id.0.clone(),
                })
            }
        }
        ToolPermissionTier::ElevationRequired => {
            if call.explicit_approval {
                Ok(ToolOutcome::Elevated)
            } else {
                Err(AuthError::ElevationRequired)
            }
        }
        ToolPermissionTier::Dangerous => {
            if call.human_approval_token.is_some() {
                Ok(ToolOutcome::Permitted)
            } else {
                Err(AuthError::HumanApprovalRequired)
            }
        }
    };

    let outcome = match &result {
        Ok(o) => *o,
        Err(AuthError::PermissionDenied { .. }) => ToolOutcome::Denied,
        Err(AuthError::ElevationRequired) => ToolOutcome::Denied,
        Err(AuthError::HumanApprovalRequired) => ToolOutcome::HumanApprovalRequired,
        Err(AuthError::ToolUnknown) => ToolOutcome::Denied,
    };

    let entry = log.append(ToolAuditEntry {
        agent_id: call.agent_id.0.clone(),
        tool_id: call.tool_id.0.clone(),
        timestamp_unix: 0,
        params_hash,
        outcome,
    });

    match result {
        Ok(_) => Ok(entry),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(tier: ToolPermissionTier) -> ToolCall {
        ToolCall {
            agent_id: AgentId("a1".into()),
            tool_id: ToolId("t1".into()),
            allowed_agents: Some(vec![AgentId("other".into())]),
            tier,
            explicit_approval: false,
            human_approval_token: None,
            params: "{}".into(),
        }
    }

    #[test]
    fn unauthorized_agent_denied() {
        let mut log = ToolAuditLog::default();
        let err = authorize(&call(ToolPermissionTier::AgentRestricted), &mut log)
            .expect_err("restricted agent must be denied");
        assert!(matches!(err, AuthError::PermissionDenied { .. }));
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn elevation_without_flag() {
        let mut log = ToolAuditLog::default();
        let err = authorize(&call(ToolPermissionTier::ElevationRequired), &mut log)
            .expect_err("missing elevation flag");
        assert_eq!(err, AuthError::ElevationRequired);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn permitted_and_denied_both_audit() {
        let mut log = ToolAuditLog::default();
        let mut ok = call(ToolPermissionTier::Unrestricted);
        authorize(&ok, &mut log).expect("unrestricted ok");
        ok.tier = ToolPermissionTier::AgentRestricted;
        let _ = authorize(&ok, &mut log);
        assert_eq!(log.len(), 2);
    }
}
