use gaia_aispd::{start, start_rsi, AispdError};

#[test]
fn rsi_denied_and_swarm_needs_owner() {
    assert_eq!(start(None).unwrap_err(), AispdError::NoOwner);
    start(Some("owner-1")).unwrap();
    assert!(start_rsi().is_err());
}
