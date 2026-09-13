//! #196 identity + HAL. T0 is not a 500KLOC kernel.

#[derive(Debug, Clone)]
pub struct Capability {
    pub revoke: &'static str,
}

impl Capability {
    pub fn new() -> Self {
        Self {
            revoke: "strongly-consistent",
        }
    }
}

pub fn revoke_mode() -> &'static str {
    "strongly-consistent"
}

pub fn t0_kernel_kloc() -> u32 {
    0
}
