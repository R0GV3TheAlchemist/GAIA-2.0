//! #569 listed shelf. Existing APIs only.

use gaia_ukd::{
    fallback_chain, public_dump, release_notes, sign_language_links, switch_ui, ukd_v1_tagged,
    UkdError, UN_LANGS,
};

#[test]
fn six_un_langs_only() {
    assert_eq!(UN_LANGS.len(), 6);
    assert_eq!(switch_ui("en").unwrap(), "en");
    assert_eq!(switch_ui("tlh").unwrap_err(), UkdError::UnknownNode);
    assert_eq!(fallback_chain(), ["requested", "en", "und"]);
    assert!(sign_language_links().is_empty());
}

#[test]
fn dump_excludes_tek_and_v1_stays_off() {
    let dump = public_dump().unwrap();
    assert!(dump.contains("tek=excluded"));
    assert!(dump.contains("provenance"));
    assert!(release_notes().iter().any(|l| l.contains("no UKD v1.0")));
    assert!(!ukd_v1_tagged());
}
