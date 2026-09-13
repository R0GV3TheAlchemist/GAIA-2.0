//! #109 skill path fixture. Links to UKD are typed requires_knowledge.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillStep {
    pub id: String,
    pub resource: String,
    pub requires_knowledge: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillPath {
    pub goal: String,
    pub steps: Vec<SkillStep>,
}

pub fn novice_public_speaking() -> SkillPath {
    SkillPath {
        goal: "novice public speaking".into(),
        steps: vec![
            SkillStep {
                id: "skill:breath".into(),
                resource: "https://www.gutenberg.org/".into(),
                requires_knowledge: Some("ukd:health:breath".into()),
            },
            SkillStep {
                id: "skill:active-listening".into(),
                resource: "https://ocw.mit.edu/".into(),
                requires_knowledge: Some("ukd:humanities:rhetoric".into()),
            },
            SkillStep {
                id: "skill:public-speaking".into(),
                resource: "https://www.khanacademy.org/".into(),
                requires_knowledge: None,
            },
        ],
    }
}
