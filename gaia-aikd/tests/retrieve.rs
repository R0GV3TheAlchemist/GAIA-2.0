use gaia_aikd::{Layer, QueryHit};

#[test]
fn offline_query_separates_parametric_and_retrieved() {
    let hit = QueryHit::offline("what is albedo").unwrap();
    assert_eq!(hit.parametric.layer, Layer::Weights);
    assert_eq!(hit.retrieved.layer, Layer::Retrieved);
    assert!(!hit.used_network);
}
