//! #515 listed shelf. Existing APIs only.

use gaia_aikd::{
    aikd_v1_tagged, cannot_know_catalog, knows_everything, query_cannot_know, KnowledgeType,
};

#[test]
fn seven_types_and_five_cannot_know_keys() {
    assert_eq!(KnowledgeType::all().len(), 7);
    let cat = cannot_know_catalog();
    assert_eq!(cat.len(), 5);
    assert!(query_cannot_know("undecidable"));
    assert!(query_cannot_know("genuine-novelty"));
    assert!(!query_cannot_know("quantum-mechanics"));
}

#[test]
fn not_everything_and_not_v1() {
    assert!(!knows_everything());
    assert!(!aikd_v1_tagged());
}
