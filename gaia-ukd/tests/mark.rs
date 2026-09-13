use gaia_ukd::{qa_script, ExportMark};

#[test]
fn mark_is_generated_without_warehouse_and_does_not_claim_c2pa() {
    let mark = ExportMark::sign("gaian-local", "consent-1");
    assert!(mark.generated);
    assert!(!mark.warehouse);
    assert!(mark.detect_tamper(true));
    assert!(!mark.verifies_with_c2pa_tooling());
    assert!(qa_script().contains(&"delete-my-GAIAN"));
    assert!(qa_script().contains(&"no-GAIAN-v1.0-tag"));
}
