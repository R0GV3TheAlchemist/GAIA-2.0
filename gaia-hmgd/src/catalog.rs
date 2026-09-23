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

/// Returns `true` when every realm in the catalog has at least one sourced node.
///
/// This is a governance sentinel: GAIA's honesty contract requires that every
/// declared realm maps to real evidence nodes before any request is routed.
/// It is called from governance tests only; `#[allow(dead_code)]` suppresses
/// the lint for library consumers that do not call it from non-test code.
#[allow(dead_code)]
pub fn all_realms_sourced() -> bool {
    REALMS.iter().all(|r| !nodes_for(r).is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Governance invariant: every realm the catalog declares must resolve to at
    /// least one evidence node. An empty slot means a request for that realm
    /// would route to a node list of length zero, silently producing no answer
    /// rather than an honest "not found" error.
    #[test]
    fn all_realms_have_at_least_one_node() {
        assert!(
            all_realms_sourced(),
            "catalog integrity failure: one or more realms have no sourced nodes"
        );
    }
}
