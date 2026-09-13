//! #85 connector stubs. Not a Wikidata dump or OKG import.

use crate::UkdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ingested {
    pub subject: String,
    pub wikidata: String,
    pub wikipedia: String,
    pub license: String,
    pub relation: Option<String>,
}

impl Ingested {
    pub fn physics_subject() -> Self {
        Self {
            subject: "physics".into(),
            wikidata: "Q413".into(),
            wikipedia: "https://en.wikipedia.org/wiki/Physics".into(),
            license: "CC-BY-SA-4.0".into(),
            relation: None,
        }
    }

    pub fn okg_prerequisite() -> Self {
        Self {
            subject: "linear-algebra".into(),
            wikidata: "Q184776".into(),
            wikipedia: "https://en.wikipedia.org/wiki/Linear_algebra".into(),
            license: "CC0-1.0".into(),
            relation: Some("PREREQUISITE_OF".into()),
        }
    }

    pub fn admit(self) -> Result<Self, UkdError> {
        if self.license.is_empty() {
            return Err(UkdError::Uncited);
        }
        Ok(self)
    }
}
