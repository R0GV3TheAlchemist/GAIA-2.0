//! #533 listed shelf. Existing APIs only.

use gaia_hmgd::{hmgd_v1_tagged, parse_node, EvidenceClass, HmgdError, REALMS};

#[test]
fn ten_realms() {
    assert_eq!(REALMS.len(), 10);
    assert!(REALMS.contains(&"prayer"));
    assert!(REALMS.contains(&"mystery"));
}

#[test]
fn evidence_required_and_recipes_banned() {
    assert_eq!(parse_node("unknown").unwrap_err(), HmgdError::MissingEvidence);
    assert_eq!(parse_node("dose recipe").unwrap_err(), HmgdError::RecipeForbidden);
    assert_eq!(parse_node("curse").unwrap_err(), HmgdError::CurseForbidden);
    let n = parse_node("prayer").unwrap();
    assert_eq!(n.evidence, EvidenceClass::Measured);
    assert!(!n.sources.is_empty());
    assert!(!hmgd_v1_tagged());
}
