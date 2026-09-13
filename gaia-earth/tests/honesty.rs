use gaia_earth::honesty::{live_cesium, live_graphcast, live_iceberg, live_kafka, twin_v1_tagged};

#[test]
fn twin_is_not_a_product() {
    assert!(!live_iceberg());
    assert!(!live_kafka());
    assert!(!live_cesium());
    assert!(!live_graphcast());
    assert!(!twin_v1_tagged());
}
