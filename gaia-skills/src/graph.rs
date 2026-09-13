//! #114 in-process skill graph. Not ESCO/O*NET dump or Neo4j.

use crate::{active_listening, SkillError, SkillNode};

#[derive(Debug, Default)]
pub struct SkillGraph {
    nodes: Vec<SkillNode>,
}

impl SkillGraph {
    pub fn seed() -> Self {
        Self {
            nodes: vec![active_listening()],
        }
    }

    pub fn by_esco(&self, id: &str) -> Result<&SkillNode, SkillError> {
        self.nodes
            .iter()
            .find(|n| n.esco.as_deref() == Some(id))
            .ok_or(SkillError::UnknownSkill)
    }

    pub fn by_onet(&self, id: &str) -> Result<&SkillNode, SkillError> {
        self.nodes
            .iter()
            .find(|n| n.onet.as_deref() == Some(id))
            .ok_or(SkillError::UnknownSkill)
    }

    pub fn unmapped(&self) -> Vec<&SkillNode> {
        self.nodes
            .iter()
            .filter(|n| n.esco.is_none() && n.onet.is_none())
            .collect()
    }
}
