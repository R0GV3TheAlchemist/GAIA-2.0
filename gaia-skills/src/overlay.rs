//! #115 realm stubs + WEF/DigComp/BESSI tags. Not a live overlay ingest.

use crate::REALMS;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealmStub {
    pub realm: String,
    pub domain: String,
}

pub fn realm_stubs() -> Vec<RealmStub> {
    REALMS
        .iter()
        .map(|realm| RealmStub {
            realm: (*realm).into(),
            domain: format!("{realm}:domain-stub"),
        })
        .collect()
}

/// Essay 12-realm names from Documents/ Universal Human Skills DB v0.1.
/// Listed only. Not a rename of `REALMS`.
pub fn research_realms() -> [&'static str; 12] {
    [
        "cognitive-intellectual",
        "physical-motor",
        "social-interpersonal",
        "emotional-self-management",
        "creative-artistic",
        "digital-technological",
        "professional-leadership",
        "practical-life",
        "survival-outdoor",
        "innovation-future",
        "healing-care",
        "spiritual-wisdom",
    ]
}

/// Map an essay realm onto existing crate REALMS. Empty = unknown name.
pub fn research_realm_bind(essay: &str) -> &'static [&'static str] {
    match essay {
        "cognitive-intellectual" => &["cognition", "learning"],
        "physical-motor" => &["physical"],
        "social-interpersonal" => &["communication", "civic"],
        "emotional-self-management" => &["social-emotional"],
        "creative-artistic" => &["creative"],
        "digital-technological" => &["digital"],
        "professional-leadership" => &["leadership"],
        "practical-life" => &["craft"],
        "survival-outdoor" => &["physical", "traditional"],
        "innovation-future" => &["learning", "digital"],
        "healing-care" => &["care"],
        "spiritual-wisdom" => &["traditional"],
        _ => &[],
    }
}

pub fn wef_top10() -> [&'static str; 10] {
    [
        "analytical-thinking",
        "creative-thinking",
        "ai-and-big-data",
        "leadership",
        "curiosity",
        "technological-literacy",
        "resilience",
        "systems-thinking",
        "talent-management",
        "service-orientation",
    ]
}

/// WEF Future of Jobs 2025 rising-skills tags as listed in the essay.
/// Does not replace `wef_top10()`.
pub fn wef_2025_essay() -> [&'static str; 10] {
    [
        "ai-and-big-data",
        "networks-and-cybersecurity",
        "technological-literacy",
        "creative-thinking",
        "resilience",
        "curiosity",
        "leadership",
        "talent-management",
        "analytical-thinking",
        "environmental-stewardship",
    ]
}

pub fn digcomp_areas() -> [&'static str; 5] {
    [
        "information-and-data-literacy",
        "communication-and-collaboration",
        "digital-content-creation",
        "safety",
        "problem-solving",
    ]
}

pub fn bessi_domains() -> [&'static str; 5] {
    [
        "social-engagement",
        "cooperation",
        "self-management",
        "emotional-resilience",
        "innovation",
    ]
}

pub fn wef_resolves(tag: &str) -> String {
    format!("skill:{tag}")
}
