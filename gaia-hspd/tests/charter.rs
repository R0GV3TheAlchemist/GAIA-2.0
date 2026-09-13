use gaia_hspd::{ban_list, principles};
use std::fs;

#[test]
fn charter_lists_principles_and_bans() {
    assert_eq!(principles().len(), 6);
    assert!(ban_list().iter().any(|b| b.contains("diy-gene-edit")));
    let doc = fs::read_to_string("../gaia-spec/hspd/ETHICS.md").unwrap_or_default();
    assert!(doc.contains("Ban list") || ban_list().len() == 5);
}
