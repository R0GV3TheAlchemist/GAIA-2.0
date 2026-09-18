use serde::{Deserialize, Serialize};

use crate::types::{ActionClass, RiskTier};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentityKind {
    Agent,
    Gateway,
    Server,
    Approver,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrincipalId {
    pub kind: IdentityKind,
    pub id: String,
}

#[derive(Debug, Clone, Default)]
pub struct RevocationList {
    ids: Vec<String>,
}

impl RevocationList {
    pub fn revoke(&mut self, id: impl Into<String>) {
        let id = id.into();
        if !self.ids.contains(&id) {
            self.ids.push(id);
        }
    }

    pub fn is_revoked(&self, id: &str) -> bool {
        self.ids.iter().any(|x| x == id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityManifest {
    pub agent_id: String,
    pub task_id: String,
    pub issued_at: u64,
    pub expires_at: u64,
    pub allowed_tools: Vec<String>,
    pub allowed_paths: Vec<String>,
    pub allowed_destinations: Vec<String>,
    pub max_actions: u32,
    pub actions_used: u32,
    pub max_risk: RiskTier,
    pub policy_version: String,
}

impl CapabilityManifest {
    pub fn local_reader(agent_id: &str, now: u64) -> Self {
        Self {
            agent_id: agent_id.into(),
            task_id: format!("task-{agent_id}"),
            issued_at: now,
            expires_at: now + 3600,
            allowed_tools: vec!["local_parse".into(), "local_read".into(), "scratch_write".into()],
            allowed_paths: vec!["scratch/".into(), "docs/".into()],
            allowed_destinations: vec![],
            max_actions: 8,
            actions_used: 0,
            max_risk: RiskTier::T2,
            policy_version: crate::policy::POLICY_VERSION.into(),
        }
    }

    pub fn expired(&self, now: u64) -> bool {
        now >= self.expires_at
    }

    pub fn budget_exceeded(&self) -> bool {
        self.actions_used >= self.max_actions
    }

    pub fn tool_allowed(&self, tool: &str) -> bool {
        self.allowed_tools.iter().any(|t| t == tool)
    }

    pub fn path_allowed(&self, path: &str) -> bool {
        if path.contains("..") {
            return false;
        }
        self.allowed_paths.iter().any(|p| path.starts_with(p))
    }

    pub fn dest_allowed(&self, dest: &str) -> bool {
        self.allowed_destinations.iter().any(|d| d == dest)
    }

    pub fn risk_allowed(&self, class: ActionClass) -> bool {
        class.risk() <= self.max_risk
    }
}
