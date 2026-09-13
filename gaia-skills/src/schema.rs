//! #108 skill node + 12 realms. Not ESCO/O*NET live ingest.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillNode {
    pub id: String,
    pub realm: String,
    pub dreyfus: String,
    pub esco: Option<String>,
    pub onet: Option<String>,
    pub license: String,
}

pub const REALMS: [&str; 12] = [
    "communication",
    "cognition",
    "digital",
    "physical",
    "craft",
    "care",
    "social-emotional",
    "civic",
    "creative",
    "leadership",
    "traditional",
    "learning",
];

pub fn active_listening() -> SkillNode {
    SkillNode {
        id: "skill:active-listening".into(),
        realm: "communication".into(),
        dreyfus: "novice".into(),
        esco: None,
        onet: None,
        license: "CC0-1.0".into(),
    }
}
