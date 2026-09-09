use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrincipalKind {
    Human,
    Gaian,
    Agent,
    Node,
    Service,
}

pub struct Principal {
    pub kind: PrincipalKind,
    pub keypair: Keypair,
}

impl Principal {
    pub fn generate(kind: PrincipalKind) -> Self {
        let mut rng = OsRng {};
        Self { kind, keypair: Keypair::generate(&mut rng) }
    }

    pub fn public_hex(&self) -> String {
        hex::encode(self.keypair.public.as_bytes())
    }

    pub fn did(&self) -> String {
        format!("did:key:gaia:ed25519:{}", self.public_hex())
    }

    pub fn sign(&self, payload: &[u8]) -> Vec<u8> {
        self.keypair.sign(payload).to_bytes().to_vec()
    }
}

pub fn verify(public_hex: &str, payload: &[u8], signature: &[u8]) -> bool {
    let Ok(pk_bytes) = hex::decode(public_hex) else { return false; };
    let Ok(pk) = PublicKey::from_bytes(&pk_bytes) else { return false; };
    let Ok(sig) = Signature::from_bytes(signature) else { return false; };
    pk.verify(payload, &sig).is_ok()
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}
