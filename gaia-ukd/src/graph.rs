//! #78 taxonomy and in-process graph. Not Neo4j or a live SPARQL bridge.

use crate::UkdError;

pub const REALMS: [&str; 12] = [
    "physical-sciences",
    "life-sciences",
    "earth-systems",
    "mathematics",
    "engineering",
    "health",
    "social-sciences",
    "humanities",
    "arts",
    "traditional-knowledge",
    "practical-skills",
    "governance-commons",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeNode {
    pub id: String,
    pub realm: String,
    pub citation: String,
}

fn seed() -> Vec<KnowledgeNode> {
    vec![
        KnowledgeNode {
            id: "ukd:math:linear-algebra".into(),
            realm: "mathematics".into(),
            citation: "fixture:open-text".into(),
        },
        KnowledgeNode {
            id: "ukd:eng:quantum-computing".into(),
            realm: "engineering".into(),
            citation: "fixture:open-text".into(),
        },
    ]
}

pub fn list_realms() -> [&'static str; 12] {
    REALMS
}

pub fn get_node(id: &str) -> Result<KnowledgeNode, UkdError> {
    seed()
        .into_iter()
        .find(|n| n.id == id)
        .ok_or(UkdError::UnknownNode)
}

pub fn federated_wikidata(write_tek: bool) -> Result<&'static str, UkdError> {
    if write_tek {
        return Err(UkdError::TekSealed);
    }
    Ok("wikidata-stub; not a live SPARQL result")
}
