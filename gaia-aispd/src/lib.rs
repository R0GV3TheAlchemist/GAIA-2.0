//! AISPD first cuts (#144–#154). Jagged extremes. Not ASI.

mod catalog;
mod charter;
mod emerge;
mod governance;
mod node;
mod swarm;
mod watch;

pub use catalog::{license_exam, weather_node};
pub use charter::{aispd_v1_tagged, principles, prohibited};
pub use emerge::{close_finding, label_trace, Finding};
pub use governance::{not_asi_line, pause_drill};
pub use node::{parse_node, AispdNode, Status};
pub use swarm::{start, start_rsi, Swarm};
pub use watch::{active_weather, wet_lab, Watch};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AispdError {
    MissingFailures,
    AsiActive,
    Prohibited,
    Unbounded,
    NoOwner,
    WontfixBlocked,
}
