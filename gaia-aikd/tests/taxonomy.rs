use gaia_aikd::{query_cannot_know, KnowledgeType};

#[test]
fn seven_types_and_cannot_know_is_queryable() {
    assert_eq!(KnowledgeType::all().len(), 7);
    assert!(query_cannot_know("undecidable"));
    assert!(!query_cannot_know("everything"));
}
