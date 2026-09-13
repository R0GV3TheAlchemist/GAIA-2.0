use gaia_aispd::{parse_node, AispdError, Status};

#[test]
fn failures_required_and_asi_cannot_be_active() {
    assert_eq!(parse_node("clock", Status::Active, &[]).unwrap_err(), AispdError::MissingFailures);
    assert_eq!(parse_node("asi-claim", Status::Active, &["unmeasured"]).unwrap_err(), AispdError::AsiActive);
    parse_node("asi-claim", Status::FutureMonitor, &["unmeasured"]).unwrap();
}
