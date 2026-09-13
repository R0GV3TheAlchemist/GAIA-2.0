use gaia_ukd::{public_dump, release_notes, sign_language_links, switch_ui, ukd_v1_tagged, UN_LANGS};

#[test]
fn six_un_languages_and_no_v1() {
    assert_eq!(UN_LANGS.len(), 6);
    for lang in UN_LANGS {
        switch_ui(lang).unwrap();
    }
    assert!(release_notes().iter().any(|n| n.contains("sacred TEK")));
    assert!(public_dump().unwrap().contains("license="));
    assert!(sign_language_links().is_empty());
    assert!(!ukd_v1_tagged());
}
