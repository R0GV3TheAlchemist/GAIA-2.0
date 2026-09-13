use gaia_gaian::{blueprint_example_valid, Consent, PersonaPackage};

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
fn local_package_roundtrip_without_network() {
    let pkg = PersonaPackage::create(&adult()).unwrap();
    assert!(!pkg.used_network);
    assert_eq!(pkg.format, "vrm-1.0");
    let blob = pkg.write_local();
    PersonaPackage::read_local(&blob).unwrap();
    assert!(blueprint_example_valid());
}
