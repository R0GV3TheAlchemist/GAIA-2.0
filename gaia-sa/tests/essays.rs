//! #449 essay bind. Calls APIs already on main.
use gaia_sa::{consult, lbc_certified, sa_v1_tagged, scrape_country};

#[test]
essay_sa_is_practice_not_certification() {
    assert!(!sa_v1_tagged());
    assert!(!lbc_certified());
    assert!(scrape_country().is_err());
    assert!(consult(false).is_err());
}
