use gaia_ukd::{labels_for, plan};

#[test]
fn path_lists_eta_gaps_and_licensed_resources() {
    let planned = plan("quantum-computing", &[]).unwrap();
    assert!(!planned.steps.is_empty());
    assert!(planned.eta_hours > 0);
    assert!(!planned.gaps.is_empty());
    assert!(planned.steps.iter().all(|s| !s.resource.license.is_empty()));
    let labels = labels_for("quantum-computing");
    assert_eq!(labels.en, "quantum-computing");
    assert_eq!(labels.ar, labels.es);
}
