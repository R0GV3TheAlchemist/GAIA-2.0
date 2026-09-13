//! HMGD first cuts (#155–#165). Not a spellcaster. Prayer is not emergency care.

mod catalog;
mod charter;
mod crosswalk;
mod node;
mod profile;
mod room;
mod sealed;

pub use catalog::{brew, nodes_for};
pub use charter::{principles, prohibited};
pub use crosswalk::{climate_as_spirit, sacred_layer};
pub use node::{parse_node, EvidenceClass, MagicNode, REALMS};
pub use profile::{infer_belief, mine_denomination, HmgdProfile};
pub use room::{join_room, sell_closed_rite, Room};
pub use sealed::{hmgd_v1_tagged, sealed_rite, songlines};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HmgdError {
    MissingEvidence,
    RecipeForbidden,
    CurseForbidden,
    Sealed,
    BeliefInferred,
    EmergencyMisuse,
    NotOptIn,
    SaleForbidden,
    ClimateRewrite,
}
