//! AISPD first cuts (#144–#150). Jagged extremes. Not ASI.

mod charter;
mod node;
mod swarm;
mod watch;

pub use charter::{aispd_v1_tagged, principles, prohibited};
pub use node::{parse_node, AispdNode, Status};
pub use swarm::{start_rsi, Swarm};
pub use watch::{active_weather, wet_lab, Watch};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AispdError {
    MissingFailures,
    AsiActive,
    Prohibited,
    Unbounded,
}
