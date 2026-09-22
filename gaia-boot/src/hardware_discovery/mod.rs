//! Phase 3 + 4 — Hardware discovery and GAIA capability manifest.
//!
//! Enumerates CPU cores, architecture, and SIMD capabilities via
//! `gaia-hal`, then packages the results into a `GaiaCapabilityManifest`
//! that subsequent kernel and runtime layers consume.
//!
//! At HAL v0.1 the only top-level fields on `HalCapabilities` are
//! `cpu`, `platform`, and `clock_resolution_ns`.  GPU, memory-map,
//! sensor-bus, and NIC discovery are deferred to future HAL sub-modules;
//! we derive proxy booleans from CPU topology and platform flags today.

use std::convert::TryInto;

use gaia_hal::{
    cpu::CpuArch,
    platform::HalTier,
    HalCapabilities,
};

/// The GAIA capability manifest produced at the end of Phase 4.
#[derive(Debug, Clone)]
pub struct GaiaCapabilityManifest {
    /// Number of logical CPU cores detected.
    pub cpu_cores: u32,
    /// CPU architecture identifier string.
    pub cpu_arch: String,
    /// Whether any SIMD extensions are available.
    pub simd_available: bool,
    /// Whether at least one GPU/NPU accelerator is expected.
    /// (Derived from platform tier at HAL v0.1; direct enumeration deferred.)
    pub accelerator_present: bool,
    /// Whether sensor buses are expected for this tier.
    /// (Derived from platform tier at HAL v0.1; direct enumeration deferred.)
    pub sensors_present: bool,
    /// Whether network hardware is expected for this tier.
    /// (Derived from platform tier at HAL v0.1; direct enumeration deferred.)
    pub network_present: bool,
    /// Raw HAL capabilities for downstream consumers.
    pub hal: HalCapabilities,
}

/// Run Phases 3 and 4: enumerate hardware and produce a `GaiaCapabilityManifest`.
pub fn discover() -> GaiaCapabilityManifest {
    eprintln!("[gaia-boot] phase=3 status=hardware_discovery");

    let hal = HalCapabilities::detect();

    // --- CPU ---
    let cpu_cores: u32 = hal
        .cpu
        .logical_cores
        .try_into()
        .expect("logical_cores overflows u32");

    let cpu_arch = match hal.cpu.features.arch {
        CpuArch::X86_64  => "x86_64",
        CpuArch::Aarch64 => "aarch64",
        CpuArch::RiscV64 => "riscv64",
        CpuArch::Other   => "other",
    }
    .to_string();

    let simd = &hal.cpu.features.simd;
    let simd_available = simd.sse2
        || simd.avx2
        || simd.avx512f
        || simd.neon
        || simd.sve;

    // --- Platform-tier proxies (HAL v0.1 — direct sub-module enumeration deferred) ---
    // PlatformFeatures has no .tier field; derive the max tier from feature flags.
    // HalTier variants are T0..T4 (not Tier0..Tier4).
    let tier = hal.platform.max_tier();
    let accelerator_present = tier >= HalTier::T2;
    let sensors_present     = tier >= HalTier::T1;
    let network_present     = tier >= HalTier::T1;

    eprintln!(
        "[gaia-boot] phase=4 status=capability_manifest \
         cpu_cores={cpu_cores} cpu_arch={cpu_arch} simd={simd_available} \
         tier={tier:?} accelerator={accelerator_present} \
         sensors={sensors_present} network={network_present} \
         clock_res_ns={}",
        hal.clock_resolution_ns,
    );

    GaiaCapabilityManifest {
        cpu_cores,
        cpu_arch,
        simd_available,
        accelerator_present,
        sensors_present,
        network_present,
        hal,
    }
}
