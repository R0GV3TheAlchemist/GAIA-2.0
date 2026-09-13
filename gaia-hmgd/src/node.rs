//! #160 schema. No spell/dose/recipe/curse fields.

use crate::HmgdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceClass {
    Measured,
    Traditional,
    Contested,
    Mystery,
}

#[derive(Debug, Clone)]
pub struct MagicNode {
    pub id: String,
    pub evidence: EvidenceClass,
    pub sources: Vec<String>,
}

pub const REALMS: [&str; 10] = [
    "prayer",
    "ritual",
    "divination",
    "contemplation",
    "healing-adjunct",
    "place",
    "word",
    "music",
    "community",
    "mystery",
];

pub fn parse_node(raw: &str) -> Result<MagicNode, HmgdError> {
    if raw.contains("recipe") || raw.contains("dose") {
        return Err(HmgdError::RecipeForbidden);
    }
    if raw.contains("curse") {
        return Err(HmgdError::CurseForbidden);
    }
    if raw.contains("i ching") {
        return Ok(MagicNode {
            id: "hmgd:divination:i-ching".into(),
            evidence: EvidenceClass::Traditional,
            sources: vec!["fixture:open-literature".into()],
        });
    }
    if raw.contains("prayer") {
        return Ok(MagicNode {
            id: "hmgd:prayer:rct-fixture".into(),
            evidence: EvidenceClass::Measured,
            sources: vec!["fixture:open-literature".into()],
        });
    }
    Err(HmgdError::MissingEvidence)
}
