//! #481 listed shelf. Existing APIs only.

use gaia_aisd::{gap_nodes, protein_structure, realm_stubs, REALMS, REALM_COUNT};

#[test]
fn thirteen_realms_are_named_and_stubbed() {
    assert_eq!(REALM_COUNT, 13);
    assert_eq!(REALMS.len(), 13);
    assert!(REALMS.contains(&"science"));
    assert!(REALMS.contains(&"safety"));
    assert_eq!(realm_stubs().len(), 13);
}

#[test]
fn seed_gaps_are_queryable_and_limitations_exist() {
    let gaps = gap_nodes();
    assert_eq!(gaps.len(), 8);
    assert!(gaps.contains(&"long-horizon-cot"));
    assert!(gaps.contains(&"dexterous-embodiment"));
    assert!(!protein_structure().limitations.is_empty());
}
