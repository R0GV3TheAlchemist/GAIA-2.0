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

// ── Tests (#26 acceptance criteria) ─────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_handoff(privacy: PrivacyMode) -> Handoff {
        let id = Uuid::new_v4();
        let cube1 = Uuid::new_v4();
        let cube2 = Uuid::new_v4();
        Handoff::new(
            id,
            "agent-alpha",
            "agent-beta",
            privacy,
            vec!["work".into(), "health".into()],
            vec![cube1, cube2],
        )
    }

    // ── AC1: Handoff preserves intent_id and allowed memory scope ────────────

    #[test]
    fn handoff_preserves_intent_id() {
        let h = make_handoff(PrivacyMode::LocalOnly);
        let bundle = h.bundle(Some("some memory"), true);
        assert_eq!(bundle.intent_id, h.intent_id,
            "intent_id must survive the handoff boundary");
    }

    #[test]
    fn handoff_preserves_memory_scope() {
        let h = make_handoff(PrivacyMode::LocalOnly);
        let bundle = h.bundle(None, false);
        assert_eq!(bundle.memory_scope, h.memory_scope,
            "allowed memory scope must be forwarded unchanged");
        assert_eq!(bundle.cube_ids, h.cube_ids,
            "cube ids must be forwarded unchanged");
    }

    // ── AC2: Unsigned package cannot be installed under default policy ────────

    #[test]
    fn unsigned_package_rejected_by_default() {
        let mut market = PackageMarket::new();
        let pkg = Package::unsigned("my-skill", "v1");
        assert_eq!(
            market.install(&pkg),
            Err(MarketError::Unsigned),
            "default policy must reject packages with no signature"
        );
    }

    #[test]
    fn signed_package_accepted() {
        let mut market = PackageMarket::new();
        let signer = PackageMarket::signer();
        let pkg = Package::signed_by(&signer, "my-skill", "v1");
        assert!(market.install(&pkg).is_ok(),
            "a validly signed package must be accepted");
        assert!(market.installed().contains(&"my-skill".to_string()));
    }

    #[test]
    fn tampered_signature_rejected() {
        let mut market = PackageMarket::new();
        let signer = PackageMarket::signer();
        let mut pkg = Package::signed_by(&signer, "my-skill", "v1");
        // Corrupt the payload after signing.
        pkg.payload = "v2-tampered".into();
        assert_eq!(
            market.install(&pkg),
            Err(MarketError::BadSignature),
            "a package whose payload was altered after signing must be rejected"
        );
    }

    #[test]
    fn duplicate_install_rejected() {
        let mut market = PackageMarket::new();
        let signer = PackageMarket::signer();
        let pkg = Package::signed_by(&signer, "my-skill", "v1");
        market.install(&pkg).unwrap();
        assert!(matches!(
            market.install(&pkg),
            Err(MarketError::AlreadyInstalled(_))
        ));
    }

    // ── AC3: Federated job redacts plaintext unless user opts in ─────────────

    #[test]
    fn federated_bundle_redacts_plaintext_by_default() {
        let h = make_handoff(PrivacyMode::Federated);
        let bundle = h.bundle(Some("sensitive memory"), false /* no opt-in */);
        assert!(bundle.redacted, "bundle must be marked redacted");
        assert!(bundle.plaintext.is_none(),
            "plaintext must not be present without opt-in");
        assert!(!bundle.ships_plaintext(),
            "ships_plaintext() must return false");
    }

    #[test]
    fn federated_bundle_ships_plaintext_when_opted_in() {
        let h = make_handoff(PrivacyMode::Federated);
        let bundle = h.bundle(Some("sensitive memory"), true /* opted in */);
        assert!(!bundle.redacted, "bundle must not be marked redacted");
        assert_eq!(bundle.plaintext.as_deref(), Some("sensitive memory"));
        assert!(bundle.ships_plaintext(),
            "ships_plaintext() must return true when opted in");
    }

    #[test]
    fn local_only_bundle_always_redacts_without_opt_in() {
        let h = make_handoff(PrivacyMode::LocalOnly);
        let bundle = h.bundle(Some("private data"), false);
        assert!(bundle.plaintext.is_none());
        assert!(bundle.redacted);
    }
}
