//! #87 path + Bloom + six UN language labels. Not a live OER catalog.

use crate::UkdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Labels {
    pub ar: String,
    pub zh: String,
    pub en: String,
    pub fr: String,
    pub ru: String,
    pub es: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub url: String,
    pub license: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathNode {
    pub id: String,
    pub bloom: String,
    pub difficulty: u8,
    pub eta_hours: u8,
    pub resource: Resource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedPath {
    pub steps: Vec<PathNode>,
    pub eta_hours: u8,
    pub gaps: Vec<String>,
}

pub fn labels_for(topic: &str) -> Labels {
    Labels {
        ar: topic.into(),
        zh: topic.into(),
        en: topic.into(),
        fr: topic.into(),
        ru: topic.into(),
        es: topic.into(),
    }
}

pub fn plan(goal: &str, known: &[&str]) -> Result<PlannedPath, UkdError> {
    if goal.is_empty() {
        return Err(UkdError::UnknownNode);
    }
    let needed = ["linear-algebra", "probability", goal];
    let mut steps = vec![];
    let mut gaps = vec![];
    let mut eta = 0u8;
    for (i, id) in needed.iter().enumerate() {
        if !known.contains(id) && *id != goal {
            gaps.push((*id).into());
        }
        let hours = 4 + i as u8;
        eta = eta.saturating_add(hours);
        steps.push(PathNode {
            id: (*id).into(),
            bloom: if i == 0 { "understand".into() } else { "apply".into() },
            difficulty: (i as u8) + 2,
            eta_hours: hours,
            resource: Resource {
                url: "https://ocw.mit.edu/".into(),
                license: "CC-BY-NC-SA-4.0".into(),
            },
        });
    }
    Ok(PlannedPath {
        steps,
        eta_hours: eta,
        gaps,
    })
}
