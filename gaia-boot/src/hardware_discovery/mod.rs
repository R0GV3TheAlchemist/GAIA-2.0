//! Phase 3 + 4 — Hardware discovery and GAIA capability manifest.
//!
//! Enumerates CPU cores, RAM, GPU/NPU presence, and sensor buses via
//! `gaia-hal`, then packages the results into a `GaiaCapabilityManifest`
//! that subsequent kernel and runtime layers consume.

use gaia_hal::{
    cpu::CpuInfo,
    memory::MemoryMap,
    HalCapabilities,
};

/// The GAIA capability manifest produced at the end of Phase 4.
#[derive(Debug, Clone)]
pub struct GaiaCapabilityManifest {
    /// Number of logical CPU cores detected.
    pub cpu_cores: u32,
    /// CPU architecture identifier.
    pub cpu_arch: String,
    /// Whether any SIMD extensions are available.
    pub simd_available: bool,
    /// Total usable RAM in bytes as reported by the HAL.
    pub total_ram_bytes: u64,
    /// Number of RAM regions in the physical memory map.
    pub ram_region_count: usize,
    /// Whether at least one GPU/NPU accelerator was detected.
    pub accelerator_present: bool,
    /// Whether at least one sensor bus was detected.
    pub sensors_present: bool,
    /// Whether a network interface was detected.
    pub network_present: bool,
    /// Raw HAL capabilities for downstream consumers.
    pub hal: HalCapabilities,
}

/// Run Phases 3 and 4: enumerate hardware and produce a `GaiaCapabilityManifest`.
pub fn discover() -> GaiaCapabilityManifest {
    eprintln!("[gaia-boot] phase=3 status=hardware_discovery");

    let hal = HalCapabilities::detect();

    let cpu_cores      = hal.cpu.logical_cores;
    let cpu_arch       = hal.cpu.arch.to_string();
    let simd_available = !hal.cpu.simd_features.is_empty();
    let total_ram_bytes   = hal.memory.total_bytes();
    let ram_region_count  = hal.memory.regions.len();
    let accelerator_present = hal.gpu.is_some();
    let sensors_present     = hal.sensors.source_count() > 0;
    let network_present     = hal.network.is_some();

    eprintln!(
        "[gaia-boot] phase=4 status=capability_manifest \
         cpu_cores={cpu_cores} cpu_arch={cpu_arch} \
         total_ram_bytes={total_ram_bytes} \
         accelerator={accelerator_present} sensors={sensors_present} network={network_present}"
    );

    GaiaCapabilityManifest {
        cpu_cores,
        cpu_arch,
        simd_available,
        total_ram_bytes,
        ram_region_count,
        accelerator_present,
        sensors_present,
        network_present,
        hal,
    }
}
