//! #92 language switch fixture. Not 100 languages. Not UKD v1.0.

use crate::UkdError;

pub const UN_LANGS: [&str; 6] = ["ar", "zh", "en", "fr", "ru", "es"];

pub fn switch_ui(lang: &str) -> Result<&'static str, UkdError> {
    UN_LANGS
        .iter()
        .copied()
        .find(|l| *l == lang)
        .ok_or(UkdError::UnknownNode)
}

pub fn fallback_chain() -> [&'static str; 3] {
    ["requested", "en", "und"]
}

pub fn sign_language_links() -> [&'static str; 0] {
    []
}

pub fn release_notes() -> [&'static str; 4] {
    [
        "taxonomy 0.1",
        "sacred TEK is not ingested",
        "provenance required on public dump",
        "no UKD v1.0 tag",
    ]
}

pub fn public_dump() -> Result<&'static str, UkdError> {
    Ok("license=CC0-1.0 provenance=complete tek=excluded")
}
