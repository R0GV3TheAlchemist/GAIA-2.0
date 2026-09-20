//! #547 listed shelf. Existing APIs only.

use gaia_ukd::{
    indigenous_parallel, path_to_quantum_computing, subjects, ukd_v1_tagged, UkdError, UN_LANGS,
};

#[test]
fn path_is_ordered_and_sourced() {
    let p = path_to_quantum_computing();
    assert_eq!(p.goal, "quantum computing");
    assert_eq!(p.steps.len(), 3);
    assert!(p.steps[0].id.contains("linear-algebra"));
    assert!(!p.steps[0].resource.is_empty());
    assert!(p.steps[0].difficulty <= p.steps[2].difficulty);
}

#[test]
fn subjects_cover_realms_tek_stays_sealed() {
    assert_eq!(subjects().len(), 12);
    assert_eq!(indigenous_parallel().unwrap_err(), UkdError::TekSealed);
    assert!(!UN_LANGS.is_empty());
    assert!(!ukd_v1_tagged());
}
