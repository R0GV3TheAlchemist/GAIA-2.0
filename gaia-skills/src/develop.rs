//! #116 / #118 practice paths. Estimates, not promises. No network for solo path.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevStep {
    pub kind: String,
    pub resource: String,
    pub requires_knowledge: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevPath {
    pub goal: String,
    pub steps: Vec<DevStep>,
    pub ukd_gaps: Vec<String>,
    pub used_network: bool,
}

pub fn develop(goal: &str) -> DevPath {
    DevPath {
        goal: goal.into(),
        steps: vec![
            DevStep {
                kind: "practice".into(),
                resource: "https://www.gutenberg.org/".into(),
                requires_knowledge: Some("ukd:humanities:rhetoric".into()),
            },
            DevStep {
                kind: "practice".into(),
                resource: "https://ocw.mit.edu/".into(),
                requires_knowledge: None,
            },
        ],
        ukd_gaps: vec!["ukd:humanities:rhetoric".into()],
        used_network: false,
    }
}
