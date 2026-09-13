//! #26 A2A: handoff scope, unsigned install reject, federated redact.

use gaia_agents::{registry_layout, Handoff, MarketError, Package, PackageMarket, PrivacyMode};
use uuid::Uuid;

#[test]
fn handoff_preserves_intent_and_memory_scope() {
    let intent = Uuid::new_v4();
    let cube = Uuid::new_v4();
    let handoff = Handoff::new(
        intent,
        "researcher",
        "critic",
        PrivacyMode::LocalOnly,
        vec!["semantic".into()],
        vec![cube],
    );
    assert_eq!(handoff.intent_id, intent);
    assert_eq!(handoff.memory_scope, vec!["semantic".to_string()]);
    let bundle = handoff.bundle(Some("raw user note"), false);
    assert_eq!(bundle.intent_id, intent);
    assert_eq!(bundle.cube_ids, vec![cube]);
    assert_eq!(bundle.memory_scope, vec!["semantic".to_string()]);
    assert!(bundle.redacted);
    assert!(bundle.plaintext.is_none());
}

#[test]
fn unsigned_package_cannot_install() {
    let mut market = PackageMarket::new();
    let err = market
        .install(&Package::unsigned("critic", "wat"))
        .unwrap_err();
    assert_eq!(err, MarketError::Unsigned);
    assert!(market.installed().is_empty());
}

#[test]
fn signed_package_installs_and_forged_tag_fails() {
    let mut market = PackageMarket::new();
    let signer = PackageMarket::signer();
    let pkg = Package::signed_by(&signer, "critic", "wat-v0");
    market.install(&pkg).unwrap();
    assert_eq!(market.installed(), &["critic".to_string()]);
    let forged = Package {
        name: "writer".into(),
        payload: "wat-v0".into(),
        signature: Some("ed25519:00:00".into()),
    };
    assert_eq!(market.install(&forged).unwrap_err(), MarketError::BadSignature);
}

#[test]
fn federated_job_redacts_plaintext_unless_opt_in() {
    let handoff = Handoff::new(
        Uuid::new_v4(),
        "researcher",
        "peer",
        PrivacyMode::Federated,
        vec!["episodic".into()],
        vec![Uuid::new_v4()],
    );
    let redacted = handoff.bundle(Some("I live at 123 Main"), false);
    assert!(redacted.redacted);
    assert!(redacted.plaintext.is_none());
    assert!(!redacted.ships_plaintext());
    let opted = handoff.bundle(Some("I live at 123 Main"), true);
    assert!(!opted.redacted);
    assert_eq!(opted.plaintext.as_deref(), Some("I live at 123 Main"));
}

#[test]
fn registry_is_not_on_the_wire() {
    assert!(registry_layout().contains("not on the wire"));
}
