//! #559 listed shelf. Existing APIs only.

use gaia_ukd::{cross_domain_candidates, indigenous_parallel, subjects, ukd_v1_tagged, UkdError};

#[test]
fn twelve_subjects_twenty_candidates() {
    assert_eq!(subjects().len(), 12);
    let edges = cross_domain_candidates();
    assert!(edges.len() >= 20);
    assert!(edges.iter().all(|e| e.candidate && !e.source.is_empty()));
}

#[test]
fn indigenous_parallel_stays_sealed() {
    assert_eq!(indigenous_parallel().unwrap_err(), UkdError::TekSealed);
    assert!(!ukd_v1_tagged());
}
