use uuid::Uuid;

use crate::error::{GaiaError, Result};
use crate::types::*;

/// Local stub client. No kernel transport in Phase 0.
#[derive(Debug, Default, Clone)]
pub struct GaiaClient;

impl GaiaClient {
    pub fn new() -> Self {
        Self
    }

    pub fn intent(&self, goal: impl Into<String>) -> Result<TaskHandle> {
        let goal = goal.into();
        if goal.trim().is_empty() {
            return Err(GaiaError::InvalidArgument("goal must not be empty".into()));
        }
        Ok(TaskHandle {
            intent_id: Uuid::new_v4(),
            state: "admitted".into(),
        })
    }

    pub fn context(&self, query: SemanticQuery) -> Result<MemCube> {
        if query.text.trim().is_empty() {
            return Err(GaiaError::InvalidArgument("query must not be empty".into()));
        }
        Ok(MemCube {
            id: Uuid::new_v4(),
            cube_type: "plaintext".into(),
            lifecycle: "active".into(),
        })
    }

    pub fn invoke(&self, agent: AgentSpec) -> Result<String> {
        if agent.agent_id.is_empty() {
            return Err(GaiaError::InvalidArgument("agent_id required".into()));
        }
        Ok(format!("invoked:{}", agent.name))
    }

    pub fn observe(&self, sensor: &str) -> Result<String> {
        if sensor.is_empty() {
            return Err(GaiaError::InvalidArgument("sensor required".into()));
        }
        Ok(format!("observe:{sensor}"))
    }

    pub fn sign(&self, payload: &[u8]) -> Result<Signature> {
        if payload.is_empty() {
            return Err(GaiaError::InvalidArgument("payload required".into()));
        }
        Err(GaiaError::NotImplemented(
            "Ed25519 signing lands with the Phase 1 executor (#14/#19)".into(),
        ))
    }

    pub fn verify(&self, payload: &[u8], _sig: &Signature) -> Result<bool> {
        if payload.is_empty() {
            return Err(GaiaError::InvalidArgument("payload required".into()));
        }
        Err(GaiaError::NotImplemented(
            "Ed25519 verify lands with the Phase 1 executor (#14/#19)".into(),
        ))
    }

    pub fn declare(&self, resource: ResourceSpec) -> Result<ResourceHandle> {
        if resource.name.is_empty() {
            return Err(GaiaError::InvalidArgument("resource name required".into()));
        }
        Ok(ResourceHandle {
            id: Uuid::new_v4(),
            name: resource.name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_admits() {
        let h = GaiaClient::new().intent("hello").unwrap();
        assert_eq!(h.state, "admitted");
    }

    #[test]
    fn intent_rejects_empty() {
        let err = GaiaClient::new().intent("  ").unwrap_err();
        assert!(matches!(err, GaiaError::InvalidArgument(_)));
    }

    #[test]
    fn sign_is_explicitly_unimplemented() {
        let err = GaiaClient::new().sign(b"x").unwrap_err();
        assert!(matches!(err, GaiaError::NotImplemented(_)));
    }
}
