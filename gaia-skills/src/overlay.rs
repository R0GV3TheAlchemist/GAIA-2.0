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

pub fn wef_resolves(tag: &str) -> String {
    format!("skill:{tag}")
}
