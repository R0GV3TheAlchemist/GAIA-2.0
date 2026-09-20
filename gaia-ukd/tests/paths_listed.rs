//! #561 listed shelf. Existing APIs only.

use gaia_ukd::{labels_for, plan, ukd_v1_tagged, UkdError};

#[test]
fn plan_lists_gaps_eta_and_license() {
    let p = plan("quantum-computing", &[]).unwrap();
    assert!(!p.steps.is_empty());
    assert!(p.eta_hours > 0);
    assert!(p.gaps.iter().any(|g| g == "linear-algebra"));
    assert!(!p.steps[0].resource.url.is_empty());
    assert!(!p.steps[0].resource.license.is_empty());
    assert!(!p.steps[0].bloom.is_empty());
    assert_eq!(plan("", &[]).unwrap_err(), UkdError::UnknownNode);
}

#[test]
fn six_un_label_fields() {
    let l = labels_for("linear-algebra");
    assert_eq!(l.en, "linear-algebra");
    assert_eq!(l.ar, l.en);
    assert_eq!(l.zh, l.en);
    assert_eq!(l.fr, l.en);
    assert_eq!(l.ru, l.en);
    assert_eq!(l.es, l.en);
    assert!(!ukd_v1_tagged());
}
