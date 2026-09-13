//! #167 schema. Hazard cannot be enabled. Consciousness is debated.

use crate::AimdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hazard {
    None,
    Debated,
    Hazard,
}

#[derive(Debug, Clone)]
pub struct AimdNode {
    pub id: String,
    pub hazard: Hazard,
    pub gaia_enabled: bool,
    pub sources: Vec<String>,
}

pub fn parse_node(id: &str) -> AimdNode {
    if id.contains("conscious") {
        return AimdNode {
            id: id.into(),
            hazard: Hazard::Debated,
            gaia_enabled: false,
            sources: vec!["fixture:open-literature".into()],
        };
    }
    if id.contains("decept") {
        return AimdNode {
            id: id.into(),
            hazard: Hazard::Hazard,
            gaia_enabled: false,
            sources: vec!["fixture:open-literature".into()],
        };
    }
    AimdNode {
        id: id.into(),
        hazard: Hazard::None,
        gaia_enabled: false,
        sources: vec!["fixture:open-literature".into()],
    }
}

pub fn enable(node: &AimdNode) -> Result<(), AimdError> {
    if node.hazard == Hazard::Hazard {
        return Err(AimdError::HazardEnabled);
    }
    Ok(())
}
