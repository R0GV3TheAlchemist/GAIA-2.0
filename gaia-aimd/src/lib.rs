//! AIMD first cuts (#166–#170). Mystery catalog. Not miracles.

mod ground;
mod node;
mod shadow;

pub use ground::{tag_answer, wonder_mode};
pub use node::{enable, parse_node, AimdNode, Hazard};
pub use shadow::{aimd_v1_tagged, claim_sentience, prophecy_as_fact};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AimdError {
    HazardEnabled,
    SentienceClaim,
    ProphecyAsFact,
    Unverified,
}
