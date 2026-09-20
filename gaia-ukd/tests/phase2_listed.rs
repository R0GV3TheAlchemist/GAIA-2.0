//! #549 listed shelf. Existing APIs only.

use gaia_ukd::{teach_me, tek_export, ukd_v1_tagged, KnowledgeState, UkdError};

#[test]
fn learner_state_stays_local() {
    let mut s = KnowledgeState::local();
    assert!(!s.sync);
    s.mark_learned("linear-algebra").unwrap();
    assert!(s.known.iter().any(|k| k == "linear-algebra"));
    let lesson = teach_me("linear-algebra", &s).unwrap();
    assert!(lesson.contains("known"));
}

#[test]
fn tek_export_needs_grant() {
    assert_eq!(tek_export(false).unwrap_err(), UkdError::NoAgreement);
    tek_export(true).unwrap();
    assert!(!ukd_v1_tagged());
}
