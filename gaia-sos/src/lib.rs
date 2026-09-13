//! Super OS design contracts (#190–#200). No second kernel. Not v1.0.

mod abi;
mod eacn;
mod gate;
mod identity;
mod sched;

pub use abi::{learn, submit_intent, HOST_CALLS};
pub use eacn::{discover, god_coordinator};
pub use gate::{formal_verify_done, sos_v1_tagged, threats};
pub use identity::{revoke_mode, t0_kernel_kloc, Capability};
pub use sched::{kernel_inference, sfs_v1};

pub fn second_kernel() -> bool {
    false
}

pub fn rsi() -> bool {
    false
}

pub fn sentience() -> bool {
    false
}

pub fn five_nines_claimed() -> bool {
    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SosError {
    Unsigned,
    NoCapability,
    WeightRewrite,
    GodCoordinator,
}
