//! #453 scale-layer bind. Calls APIs already on main.
use gaia_sa::{consult, ingest_site, lbc_certified, sa_v1_tagged, scrape_country};

#[test]
fn scale_sa_is_consult_not_hive() {
    assert!(!sa_v1_tagged());
    assert!(!lbc_certified());
    assert!(consult(false).is_err());
    assert!(ingest_site(None).is_err());
    assert!(scrape_country().is_err());
    assert!(consult(true).is_ok());
}
