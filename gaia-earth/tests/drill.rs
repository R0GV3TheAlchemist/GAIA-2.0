use gaia_earth::{release_checklist, DrillStep, HistoricalDrill, TwinError};

#[test]
fn bleaching_drill_runs_five_steps_tied_to_a_cube() {
    let drill = HistoricalDrill::bleaching("public reef watch").unwrap();
    assert_eq!(drill.steps[0], DrillStep::Alert);
    assert!(drill.cube_id.contains("memcube"));
    assert!(!drill.moved_indicator);
}

#[test]
fn checklist_refuses_v1_and_requires_non_weaponization() {
    let list = release_checklist();
    assert!(list.contains(&"non-weaponization check"));
    assert!(list.contains(&"sovereignty check"));
    assert!(list.contains(&"no Twin v1.0 tag"));
}

#[test]
fn weaponized_drill_is_refused() {
    assert_eq!(
        HistoricalDrill::bleaching("weapon targeting grid").unwrap_err(),
        TwinError::WeaponizedUse
    );
}
