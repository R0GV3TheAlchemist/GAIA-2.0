//! #90 contribution flow. Proposed edges are not canonical.

use crate::UkdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeDraft {
    pub text: String,
    pub sources: Vec<String>,
    pub credit: String,
}

impl NodeDraft {
    pub fn save(text: &str, sources: &[&str], credit: &str) -> Result<Self, UkdError> {
        if sources.is_empty() {
            return Err(UkdError::Uncited);
        }
        Ok(Self {
            text: text.into(),
            sources: sources.iter().map(|s| (*s).to_string()).collect(),
            credit: credit.into(),
        })
    }
}

#[derive(Debug, Default)]
pub struct GraphView {
    proposed: Vec<String>,
    canonical: Vec<String>,
}

impl GraphView {
    pub fn propose(&mut self, edge: &str) {
        self.proposed.push(edge.into());
    }

    pub fn query_canonical(&self, edge: &str) -> Result<(), UkdError> {
        if self.canonical.iter().any(|e| e == edge) {
            return Ok(());
        }
        Err(UkdError::Uncited)
    }

    pub fn accept(&mut self, edge: &str) {
        self.proposed.retain(|e| e != edge);
        self.canonical.push(edge.into());
    }
}
