use gaia_ukd::{cross_domain_candidates, indigenous_parallel, subjects, UkdError};

#[test]
fn every_realm_has_a_subject_and_cross_edges_are_sourced() {
    assert_eq!(subjects().len(), 12);
    let edges = cross_domain_candidates();
    assert!(edges.len() >= 20);
    assert!(edges.iter().all(|e| !e.source.is_empty() && e.candidate));
    assert_eq!(indigenous_parallel().unwrap_err(), UkdError::TekSealed);
}
