//! Tool permission registry (#933). Listed tiers only.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolPermissionTier {
    Unrestricted,
    AgentRestricted(Vec<AgentId>),
    ElevationRequired,
    Dangerous,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolRegistration {
    pub tool_id: ToolId,
    pub name: String,
    pub permission_tier: ToolPermissionTier,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct ToolRegistry {
    tools: Vec<ToolRegistration>,
}

impl ToolRegistry {
    pub fn register(&mut self, tool: ToolRegistration) {
        self.tools.push(tool);
    }

    pub fn get(&self, tool_id: &ToolId) -> Option<&ToolRegistration> {
        self.tools.iter().find(|t| t.tool_id == *tool_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_stores_tier() {
        let mut r = ToolRegistry::default();
        r.register(ToolRegistration {
            tool_id: ToolId("echo".into()),
            name: "echo".into(),
            permission_tier: ToolPermissionTier::Unrestricted,
            description: "echo".into(),
        });
        assert!(r.get(&ToolId("echo".into())).is_some());
    }
}
