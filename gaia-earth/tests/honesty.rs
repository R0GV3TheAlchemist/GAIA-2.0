use gaia_earth::honesty::{live_cesium, live_ews_network, live_graphcast, live_iceberg, twin_v1_tagged};

#[test]
fn twin_is_not_a_product() {
    assert!(!live_iceberg());
    assert!(!live_cesium());
    assert!(!live_graphcast());
    assert!(!live_ews_network());
    assert!(!twin_v1_tagged());
}
