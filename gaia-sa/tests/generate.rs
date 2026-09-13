use gaia_sa::{generate, lbc_certified, passport, reuse_first, sa_v1_tagged, scrape_country};

#[test]
fn access_required_no_ledger_no_scrape_no_v1() {
    assert!(generate(None).is_err());
    generate(Some(3)).unwrap();
    assert_eq!(passport(), "json-no-proprietary-ledger");
    assert!(reuse_first());
    assert!(scrape_country().is_err());
    assert!(!lbc_certified());
    assert!(!sa_v1_tagged());
}
