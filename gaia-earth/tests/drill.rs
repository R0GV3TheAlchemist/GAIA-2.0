use gaia_earth::*;

#[test]
fn bleaching_drill_is_five_steps_tied_to_a_cube() {
    let d = HistoricalDrill::bleaching("public flood forecast").unwrap();
    assert_eq!(d.steps.len(), 5);
    assert!(d.cube_id.contains("memcube"));
    assert!(!d.moved_indicator);
    assert_eq!(
        HistoricalDrill::bleaching("weapon targeting grid").unwrap_err(),
        TwinError::WeaponizedUse
    );
}

#[test]
fn release_checklist_forbids_v1_and_weapons() {
    let c = release_checklist();
    assert!(c.contains(&"non-weaponization check"));
    assert!(c.contains(&"sovereignty check"));
    assert!(c.contains(&"no Twin v1.0 tag"));
    assert!(!twin_v1_tagged());
    assert!(!live_ews_network());
}
