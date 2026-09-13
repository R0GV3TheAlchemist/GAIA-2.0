//! #81 cited synthesis. No silent hallucination. No auto-publish.

use crate::UkdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    pub text: String,
    pub sources: Vec<String>,
}

impl Claim {
    pub fn synthesize(text: &str, sources: &[&str]) -> Result<Self, UkdError> {
        if sources.is_empty() {
            return Err(UkdError::Uncited);
        }
        Ok(Self {
            text: text.into(),
            sources: sources.iter().map(|s| (*s).to_string()).collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triple {
    pub subject: String,
    pub relation: String,
    pub object: String,
}

#[derive(Debug, Default)]
pub struct ReviewQueue {
    items: Vec<Triple>,
}

impl ReviewQueue {
    pub fn propose(&mut self, triple: Triple) {
        self.items.push(triple);
    }

    pub fn pending(&self) -> &[Triple] {
        &self.items
    }

    pub fn auto_publish(&self, _triple: &Triple) -> Result<(), UkdError> {
        Err(UkdError::Uncited)
    }
}
