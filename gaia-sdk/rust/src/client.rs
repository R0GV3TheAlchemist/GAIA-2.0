use ed25519_dalek::{Keypair, PublicKey, Signature as DalekSig, Signer, Verifier};
use rand::rngs::OsRng;
use uuid::Uuid;

use crate::error::{GaiaError, Result};
use crate::types::*;

pub struct GaiaClient {
    keypair: Keypair,
}

impl Default for GaiaClient {
    fn default() -> Self { Self::new() }
}

impl std::fmt::Debug for GaiaClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GaiaClient").finish_non_exhaustive()
    }
}

impl GaiaClient {
    pub fn new() -> Self {
        let mut rng = OsRng {};
        Self { keypair: Keypair::generate(&mut rng) }
    }

    pub fn intent(&self, goal: impl Into<String>) -> Result<TaskHandle> {
        let goal = goal.into();
        if goal.trim().is_empty() {
            return Err(GaiaError::InvalidArgument("goal must not be empty".into()));
        }
        Ok(TaskHandle { intent_id: Uuid::new_v4(), state: "admitted".into() })
    }

    pub fn context(&self, query: SemanticQuery) -> Result<MemCube> {
        if query.text.trim().is_empty() {
            return Err(GaiaError::InvalidArgument("query must not be empty".into()));
        }
        Ok(MemCube { id: Uuid::new_v4(), cube_type: "plaintext".into(), lifecycle: "active".into() })
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
        let sig = self.keypair.sign(payload);
        let mut bytes = self.keypair.public.as_bytes().to_vec();
        bytes.extend_from_slice(&sig.to_bytes());
        Ok(Signature { algorithm: "ed25519".into(), bytes })
    }

    pub fn verify(&self, payload: &[u8], sig: &Signature) -> Result<bool> {
        if payload.is_empty() {
            return Err(GaiaError::InvalidArgument("payload required".into()));
        }
        if sig.algorithm != "ed25519" || sig.bytes.len() != 32 + 64 {
            return Ok(false);
        }
        let Ok(pk) = PublicKey::from_bytes(&sig.bytes[..32]) else { return Ok(false); };
        let Ok(ds) = DalekSig::from_bytes(&sig.bytes[32..]) else { return Ok(false); };
        Ok(pk.verify(payload, &ds).is_ok())
    }

    pub fn declare(&self, resource: ResourceSpec) -> Result<ResourceHandle> {
        if resource.name.is_empty() {
            return Err(GaiaError::InvalidArgument("resource name required".into()));
        }
        Ok(ResourceHandle { id: Uuid::new_v4(), name: resource.name })
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
    fn sign_verify_roundtrip() {
        let c = GaiaClient::new();
        let sig = c.sign(b"intent-proof").unwrap();
        assert!(c.verify(b"intent-proof", &sig).unwrap());
        assert!(!c.verify(b"other", &sig).unwrap());
    }
}
