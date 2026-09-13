//! #26 A2A handoff, signed local install, federated redaction.
//! There is no Git/OCI registry and no off-box transport.

use gaia_kernel::identity::{self, Principal, PrincipalKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyMode {
    LocalOnly,
    Federated,
    Cloud,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Handoff {
    pub intent_id: Uuid,
    pub from: String,
    pub to: String,
    pub privacy: PrivacyMode,
    pub memory_scope: Vec<String>,
    pub cube_ids: Vec<Uuid>,
}

impl Handoff {
    pub fn new(
        intent_id: Uuid,
        from: &str,
        to: &str,
        privacy: PrivacyMode,
        memory_scope: Vec<String>,
        cube_ids: Vec<Uuid>,
    ) -> Self {
        Self {
            intent_id,
            from: from.into(),
            to: to.into(),
            privacy,
            memory_scope,
            cube_ids,
        }
    }

    pub fn bundle(&self, plaintext: Option<&str>, opt_in_plaintext: bool) -> ContextBundle {
        let redact = !opt_in_plaintext;
        ContextBundle {
            intent_id: self.intent_id,
            cube_ids: self.cube_ids.clone(),
            memory_scope: self.memory_scope.clone(),
            redacted: redact,
            plaintext: if redact {
                None
            } else {
                plaintext.map(str::to_owned)
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextBundle {
    pub intent_id: Uuid,
    pub cube_ids: Vec<Uuid>,
    pub memory_scope: Vec<String>,
    pub redacted: bool,
    pub plaintext: Option<String>,
}

impl ContextBundle {
    pub fn ships_plaintext(&self) -> bool {
        !self.redacted && self.plaintext.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub payload: String,
    pub signature: Option<String>,
}

impl Package {
    pub fn unsigned(name: &str, payload: &str) -> Self {
        Self {
            name: name.into(),
            payload: payload.into(),
            signature: None,
        }
    }

    pub fn signed_by(signer: &Principal, name: &str, payload: &str) -> Self {
        let body = format!("{name}\n{payload}");
        Self {
            name: name.into(),
            payload: payload.into(),
            signature: Some(format!(
                "ed25519:{}:{}",
                signer.public_hex(),
                hex::encode(signer.sign(body.as_bytes()))
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketError {
    Unsigned,
    BadSignature,
    AlreadyInstalled(String),
}

pub struct PackageMarket {
    pub reject_unsigned: bool,
    installed: Vec<String>,
}

impl Default for PackageMarket {
    fn default() -> Self {
        Self {
            reject_unsigned: true,
            installed: Vec::new(),
        }
    }
}

impl PackageMarket {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn signer() -> Principal {
        Principal::generate(PrincipalKind::Agent)
    }

    pub fn install(&mut self, package: &Package) -> Result<(), MarketError> {
        match &package.signature {
            None if self.reject_unsigned => return Err(MarketError::Unsigned),
            None => {}
            Some(tag) => verify_package(package, tag)?,
        }
        if self.installed.iter().any(|n| n == &package.name) {
            return Err(MarketError::AlreadyInstalled(package.name.clone()));
        }
        self.installed.push(package.name.clone());
        Ok(())
    }

    pub fn installed(&self) -> &[String] {
        &self.installed
    }
}

fn verify_package(package: &Package, tagged: &str) -> Result<(), MarketError> {
    let mut parts = tagged.splitn(3, ':');
    let alg = parts.next().unwrap_or_default();
    let public_hex = parts.next().unwrap_or_default();
    let signature = parts.next().unwrap_or_default();
    if alg != "ed25519" || public_hex.is_empty() || signature.is_empty() {
        return Err(MarketError::BadSignature);
    }
    let Ok(sig) = hex::decode(signature) else {
        return Err(MarketError::BadSignature);
    };
    let body = format!("{}\n{}", package.name, package.payload);
    if !identity::verify(public_hex, body.as_bytes(), &sig) {
        return Err(MarketError::BadSignature);
    }
    Ok(())
}

pub fn registry_layout() -> &'static str {
    "git/oci registry is not on the wire; packages stay in-process"
}
