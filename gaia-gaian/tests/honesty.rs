use gaia_gaian::honesty::{
    gaian_v1_tagged, live_flutter, live_ollama, live_vrm_runtime, live_whisper, medical_product,
};

#[test]
fn gaian_is_not_a_product() {
    assert!(!live_ollama());
    assert!(!live_flutter());
    assert!(!live_whisper());
    assert!(!live_vrm_runtime());
    assert!(!gaian_v1_tagged());
    assert!(!medical_product());
}
