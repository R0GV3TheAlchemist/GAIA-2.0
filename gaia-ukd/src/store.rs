//! #84 in-process graph API. Not Neo4j, SPARQL live, or GraphQL server.

use crate::{list_realms, UkdError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub rel: String,
    pub to: String,
}

#[derive(Debug, Default)]
pub struct GraphApi {
    pub signed_write: bool,
}

impl GraphApi {
    pub fn realm_list(&self) -> [&'static str; 12] {
        list_realms()
    }

    pub fn write(&self, _edge: &Edge) -> Result<(), UkdError> {
        if !self.signed_write {
            return Err(UkdError::Uncited);
        }
        Ok(())
    }

    pub fn jsonld_sample(&self) -> String {
        "{\"@context\":\"https://schema.org\",\"@id\":\"ukd:eng:quantum-computing\",\"license\":\"CC0-1.0\"}".into()
    }
}
