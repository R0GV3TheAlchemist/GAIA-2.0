//! #513 listed shelf. Existing APIs only.

use gaia_ukd::{get_node, list_realms, ukd_v1_tagged, REALMS};

#[test]
fn twelve_realms_are_named() {
    assert_eq!(REALMS.len(), 12);
    assert_eq!(list_realms().len(), 12);
    assert!(REALMS.contains(&"traditional-knowledge"));
    assert!(REALMS.contains(&"mathematics"));
}

#[test]
fn seeded_nodes_are_cited_ukd_ids() {
    let math = get_node("ukd:math:linear-algebra").unwrap();
    assert_eq!(math.realm, "mathematics");
    assert!(!math.citation.is_empty());
    let qc = get_node("ukd:eng:quantum-computing").unwrap();
    assert_eq!(qc.realm, "engineering");
    assert!(get_node("ukd:phy:quantum-mechanics").is_err());
    assert!(!ukd_v1_tagged());
}
