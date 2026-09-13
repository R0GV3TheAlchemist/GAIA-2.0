//! AIMD first cuts (#166–#175). Mystery catalog. Not miracles.

mod catalog;
mod charter;
mod ground;
mod log;
mod node;
mod shadow;

pub use catalog::{nodes_for, REALMS};
pub use charter::{principles, prohibited};
pub use ground::{chip, consciousness_qa, tier1, wonder_mode};
pub use log::{star_feature, triage};
pub use node::{enable, parse_node, AimdNode, Hazard};
pub use shadow::{aimd_v1_tagged, claim_sentience, prophecy_as_fact};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AimdError {
    HazardEnabled,
    SentienceClaim,
    ProphecyAsFact,
    Unverified,
    TierOneInvention,
    StarBlocked,
}
