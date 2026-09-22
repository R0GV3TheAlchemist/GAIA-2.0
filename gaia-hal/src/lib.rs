//! GAIA Hardware Abstraction Layer — v0.1.0
//!
//! Provides a platform-independent interface over the hardware continuum
//! from IoT (Tier 0) to HPC clusters (Tier 3/4) as defined in
//! `gaia-spec/sos/hal-tiers.md`.
//!
//! # Design principles
//! - **No unsafe** in this crate at v0.1; platform drivers use the traits.
//! - **Pure-Rust**, `no_std`-compatible traits where possible.
//! - **Tier-gated features** let integrators opt into subsystems.
//! - **Mock implementations** ship alongside traits for testing.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod cpu;
pub mod gpu;
pub mod interrupts;
pub mod memory;
pub mod network;
pub mod platform;
pub mod sensors;
pub mod storage;
pub mod timers;

pub use cpu::{CpuFeatures, CpuInfo};
pub use gpu::{GpuBackend, GpuDevice};
pub use interrupts::{IrqHandler, IrqLine};
pub use memory::{MemoryMap, MemoryRegion, MemoryRegionKind};
pub use network::{NicDevice, NicInfo};
pub use platform::{HalTier, PlatformFeatures};
pub use sensors::{SensorBus, SensorEvent, SensorKind};
pub use storage::{BlockDevice, BlockDeviceInfo, FileBackedDevice};
pub use timers::MonotonicClock;

/// Aggregate capability report for the current platform.
///
/// Returned by [`HalCapabilities::detect`]; gives callers a single
/// snapshot of what the HAL believes is available.
#[derive(Debug, Clone)]
pub struct HalCapabilities {
    /// CPU feature flags and core topology.
    pub cpu: CpuInfo,
    /// Platform tier classification and feature flags.
    pub platform: PlatformFeatures,
    /// Monotonic clock resolution in nanoseconds.
    pub clock_resolution_ns: u64,
}

impl HalCapabilities {
    /// Detect capabilities of the current host.
    ///
    /// This is a pure-userspace detection pass (Tier 0/1 compatible).
    /// It does not require kernel modules, root, or hardware access.
    pub fn detect() -> Self {
        Self {
            cpu: CpuInfo::detect(),
            platform: PlatformFeatures::detect(),
            clock_resolution_ns: MonotonicClock::resolution_ns(),
        }
    }
}
