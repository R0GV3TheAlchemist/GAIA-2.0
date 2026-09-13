use gaia_earth::{ProfileGap, SystemProfile};

#[test]
fn nine_profiles_list_variables_sources_models_cadence_phase() {
    let profiles = SystemProfile::all();
    assert_eq!(profiles.len(), 9);
    assert!(profiles.iter().all(|p| !p.variables.is_empty()));
    assert!(profiles.iter().all(|p| !p.sources.is_empty()));
    assert!(profiles.iter().all(|p| !p.models.is_empty()));
    assert!(profiles.iter().all(|p| !p.cadence.is_empty()));
    assert!(profiles.iter().all(|p| p.phase >= 1));
}

#[test]
fn gaps_name_noosphere_and_commercial_imagery() {
    let gaps = SystemProfile::gaps();
    assert!(gaps.contains(&ProfileGap::Noosphere));
    assert!(gaps.contains(&ProfileGap::CommercialImagery));
}
