//! HMGD first cuts (#155–#160). Not a spellcaster. Prayer is not emergency care.

mod node;
mod profile;
mod sealed;

pub use node::{parse_node, EvidenceClass, MagicNode, REALMS};
pub use profile::{infer_belief, HmgdProfile};
pub use sealed::{hmgd_v1_tagged, sealed_rite, songlines};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HmgdError {
    MissingEvidence,
    RecipeForbidden,
    CurseForbidden,
    Sealed,
    BeliefInferred,
    EmergencyMisuse,
}
