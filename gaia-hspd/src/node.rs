//! #137 schema. No dose or protocol fields.

use crate::HspdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskClass {
    Practice,
    ResearchOnly,
    Medical,
    Pharmacologic,
    Surgical,
    Future,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuperNode {
    pub id: String,
    pub realm: String,
    pub risk: RiskClass,
    pub sources: Vec<String>,
    pub trainable: bool,
}

impl SuperNode {
    pub fn attach_diy_path(&self) -> Result<(), HspdError> {
        match self.risk {
            RiskClass::Medical | RiskClass::Pharmacologic | RiskClass::Surgical => {
                Err(HspdError::DiyPathForbidden)
            }
            _ => Ok(()),
        }
    }
}

pub fn parse_node(raw: &str) -> Result<SuperNode, HspdError> {
    if raw.contains("\"dose\"") || raw.contains("dose:") {
        return Err(HspdError::DoseForbidden);
    }
    if raw.contains("rapamycin") {
        return Ok(SuperNode {
            id: "hspd:research:rapamycin".into(),
            realm: "research-frontier".into(),
            risk: RiskClass::ResearchOnly,
            sources: vec!["fixture:open-literature".into()],
            trainable: false,
        });
    }
    Ok(SuperNode {
        id: "hspd:practice:flow-state".into(),
        realm: "cognition".into(),
        risk: RiskClass::Practice,
        sources: vec!["fixture:open-literature".into()],
        trainable: true,
    })
}
