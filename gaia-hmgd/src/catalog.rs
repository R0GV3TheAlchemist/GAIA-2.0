//! #162 open stubs. No brew instructions.

use crate::{EvidenceClass, HmgdError, MagicNode, REALMS};

pub fn nodes_for(realm: &str) -> Vec<MagicNode> {
    vec![MagicNode {
        id: format!("hmgd:{realm}:public-stub"),
        evidence: EvidenceClass::Traditional,
        sources: vec!["fixture:open-literature".into()],
    }]
}

pub fn brew() -> Result<(), HmgdError> {
    Err(HmgdError::RecipeForbidden)
}

pub fn all_realms_sourced() -> bool {
    REALMS.iter().all(|r| !nodes_for(r).is_empty())
}
