use gaia_gaian::{age_other_person, compare, Consent, GaianError, Sketch};

fn adult() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 34,
        self_consent: true,
        parental_consent: false,
        health_opt_in: false,
    }
}

#[test]
fn two_owner_scenarios_are_labeled_simulated() {
    let a = Sketch::of_owner(&adult(), "walk more", 10).unwrap();
    let b = Sketch::of_owner(&adult(), "sit more", 10).unwrap();
    assert!(a.simulated && b.simulated);
    assert_eq!(compare(&a, &b).unwrap().len(), 2);
    assert_eq!(age_other_person().unwrap_err(), GaianError::NotSelf);
}
