use gaia_aisd::{graphcast, reference_published};

#[test]
fn graphcast_is_a_tool_and_mmlu_is_reference() {
    let tool = graphcast();
    assert!(!tool.is_llm_card);
    assert!(tool.kind.contains("weather"));
    let row = reference_published("MMLU", "0.00");
    assert!(row.contains("reference_published"));
    assert!(row.contains("gaia_measured="));
}
