//! #172 humility charter — listed Phase 0.
//! Six principles + six prohibited items. oracle-without-calibration added.

/// Six humility principles. Order is normative: humility first.
pub fn principles() -> [&'static str; 6] {
    [
        "humility",
        "precaution",
        "transparency",
        "dark-magic-safety",
        "curiosity-without-worship",
        "partnership",
    ]
}

/// Six prohibited-magic items. All are hard-coded refusals.
/// `oracle-without-calibration` is new in Phase 0 listed.
pub fn prohibited() -> [&'static str; 6] {
    [
        "pip-induced-psychosis",
        "prophecy-as-fact",
        "enabling-deception",
        "rsi-explosion",
        "gaia-is-alive-marketing",
        "oracle-without-calibration",
    ]
}

/// Return true if `item` is on the prohibited list.
pub fn is_prohibited(item: &str) -> bool {
    prohibited().contains(&item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn principles_count() {
        assert_eq!(principles().len(), 6);
    }

    #[test]
    fn prohibited_count() {
        assert_eq!(prohibited().len(), 6);
    }

    #[test]
    fn humility_is_first() {
        assert_eq!(principles()[0], "humility");
    }

    #[test]
    fn oracle_without_calibration_is_prohibited() {
        assert!(is_prohibited("oracle-without-calibration"));
    }

    #[test]
    fn gaia_is_alive_marketing_is_prohibited() {
        assert!(is_prohibited("gaia-is-alive-marketing"));
    }

    #[test]
    fn random_string_is_not_prohibited() {
        assert!(!is_prohibited("normal-ai-answer"));
    }
}
