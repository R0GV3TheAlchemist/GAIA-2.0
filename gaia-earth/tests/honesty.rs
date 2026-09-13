use gaia_earth::honesty::{live_flink, live_graphcast, live_iceberg, live_kafka, live_minio, live_openda, live_sentinel_harvest, twin_v1_tagged};

#[test]
fn twin_is_not_a_product() {
    assert!(!live_iceberg());
    assert!(!live_minio());
    assert!(!live_kafka());
    assert!(!live_flink());
    assert!(!live_graphcast());
    assert!(!live_sentinel_harvest());
    assert!(!live_openda());
    assert!(!twin_v1_tagged());
}
