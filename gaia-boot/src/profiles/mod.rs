//! Platform profiles — feature-flag-gated tier definitions.
//!
//! Each tier module exposes a `profile()` function returning a `PlatformProfile`
//! describing the hardware envelope for that class of device.  Only the module
//! matching the active feature flag is compiled in; downstream code selects the
//! correct profile via `active_profile()`.

pub mod tier0;
pub mod tier1;
pub mod tier2;
pub mod tier3;
pub mod tier4;

/// Static description of a platform tier's hardware envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformProfile {
    /// Tier index (0 = IoT, 4 = HPC).
    pub tier: u8,
    /// Human-readable name.
    pub name: &'static str,
    /// Representative target hardware.
    pub target: &'static str,
    /// Maximum addressable RAM in bytes (0 = unlimited).
    pub max_ram_bytes: u64,
    /// Minimum CPU cores expected.
    pub min_cpu_cores: u32,
    /// Whether a GPU/NPU accelerator is expected.
    pub gpu_expected: bool,
    /// Whether high-speed networking (>=10 GbE) is expected.
    pub hpc_network_expected: bool,
}

/// Return the profile for the currently compiled platform tier.
pub fn active_profile() -> PlatformProfile {
    #[cfg(feature = "tier4")] { return tier4::profile(); }
    #[cfg(feature = "tier3")] { return tier3::profile(); }
    #[cfg(feature = "tier2")] { return tier2::profile(); }
    #[cfg(feature = "tier1")] { return tier1::profile(); }
    #[cfg(feature = "tier0")] { return tier0::profile(); }
    #[allow(unreachable_code)] tier2::profile()
}
