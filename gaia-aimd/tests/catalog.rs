use gaia_aimd::{nodes_for, principles, prohibited, REALMS};

#[test]
fn ten_realms_shadow_hazard_charter() {
    assert_eq!(REALMS.len(), 10);
    assert!(nodes_for("shadow").iter().all(|n| matches!(n.hazard, gaia_aimd::Hazard::Hazard)));
    assert!(nodes_for("emergence").len() >= 3);
    assert_eq!(principles().len(), 6);
    assert!(prohibited().iter().any(|p| p.contains("pip")));
}
