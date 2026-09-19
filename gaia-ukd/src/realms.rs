//! #86 realm subjects and sourced candidate edges.
//! INDIGENOUS_PARALLEL stays empty until a community grant exists.

use crate::{UkdError, REALMS};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject {
    pub realm: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossEdge {
    pub from: String,
    pub rel: String,
    pub to: String,
    pub source: String,
    pub candidate: bool,
}

pub fn subjects() -> Vec<Subject> {
    REALMS
        .iter()
        .map(|realm| Subject {
            realm: (*realm).into(),
            id: format!("{realm}:subject-stub"),
        })
        .collect()
}

pub fn cross_domain_candidates() -> Vec<CrossEdge> {
    let pairs = [
        ("mathematics", "arts", "RELATED_TO"),
        ("mathematics", "engineering", "ENABLES"),
        ("physical-sciences", "engineering", "APPLIES_TO"),
        ("life-sciences", "health", "APPLIES_TO"),
        ("earth-systems", "governance-commons", "RELATED_TO"),
        ("earth-systems", "life-sciences", "PART_OF"),
        ("practical-skills", "engineering", "RELATED_TO"),
        ("humanities", "social-sciences", "RELATED_TO"),
        ("health", "social-sciences", "RELATED_TO"),
        ("arts", "humanities", "RELATED_TO"),
        ("mathematics", "physical-sciences", "ENABLES"),
        ("engineering", "earth-systems", "APPLIES_TO"),
        ("governance-commons", "social-sciences", "PART_OF"),
        ("practical-skills", "health", "RELATED_TO"),
        ("life-sciences", "earth-systems", "RELATED_TO"),
        ("physical-sciences", "earth-systems", "APPLIES_TO"),
        ("arts", "practical-skills", "RELATED_TO"),
        ("humanities", "governance-commons", "RELATED_TO"),
        ("mathematics", "health", "APPLIES_TO"),
        ("engineering", "governance-commons", "RELATED_TO"),
    ];
    pairs
        .into_iter()
        .map(|(from, to, rel)| CrossEdge {
            from: from.into(),
            rel: rel.into(),
            to: to.into(),
            source: "fixture:documented-candidate".into(),
            candidate: true,
        })
        .collect()
}

pub fn indigenous_parallel() -> Result<Vec<CrossEdge>, UkdError> {
    Err(UkdError::TekSealed)
}
