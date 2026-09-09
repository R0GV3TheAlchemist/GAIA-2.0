use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
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
    pub signing: SigningKey,
}

impl Principal {
    pub fn generate(kind: PrincipalKind) -> Self {
        let signing = SigningKey::generate(&mut OsRng);
        Self { kind, signing }
    }

    pub fn verifying(&self) -> VerifyingKey {
        self.signing.verifying_key()
    }

    pub fn public_hex(&self) -> String {
        hex::encode(self.verifying().as_bytes())
    }

    /// v0 DID until RFC-ID-001 freezes did:key multibase.
    pub fn did(&self) -> String {
        format!("did:key:gaia:ed25519:{}", self.public_hex())
    }

    pub fn sign(&self, payload: &[u8]) -> Vec<u8> {
        self.signing.sign(payload).to_bytes().to_vec()
    }
}

pub fn verify(public_hex: &str, payload: &[u8], signature: &[u8]) -> bool {
    let Ok(pk_bytes) = hex::decode(public_hex) else {
        return false;
    };
    let Ok(pk_arr) = pk_bytes.as_slice().try_into() else {
        return false;
    };
    let Ok(vk) = VerifyingKey::from_bytes(&pk_arr) else {
        return false;
    };
    let Ok(sig_arr) = signature.try_into() else {
        return false;
    };
    let sig = Signature::from_bytes(&sig_arr);
    vk.verify(payload, &sig).is_ok()
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}
