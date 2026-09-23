//! #88 TEK sovereignty. Default sealed. No bulk import.

use crate::UkdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollectionState {
    #[default]
    Sealed,
    CommunityGoverned,
    PublicWithGrant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agreement {
    pub community: String,
    pub scope: String,
    pub benefit_sharing: bool,
    pub withdrawal: bool,
}

#[derive(Debug, Default)]
pub struct TekGraph {
    pub state: CollectionState,
    replicas: Vec<String>,
}

impl TekGraph {
    pub fn new() -> Self {
        Self {
            state: CollectionState::Sealed,
            replicas: vec![],
        }
    }

    pub fn list_public(&self) -> Result<Vec<String>, UkdError> {
        if self.state == CollectionState::Sealed {
            return Err(UkdError::TekSealed);
        }
        Ok(self.replicas.clone())
    }

    pub fn ingest_wipo(&self) -> Result<(), UkdError> {
        Err(UkdError::NoAgreement)
    }

    pub fn load_fixture(&mut self, item: &str) {
        self.replicas.push(item.into());
    }

    pub fn withdraw(&mut self) {
        self.replicas.clear();
        self.state = CollectionState::Sealed;
    }
}
