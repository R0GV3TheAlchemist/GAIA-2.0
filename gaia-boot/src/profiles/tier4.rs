//! Tier 4 — HPC / Supercomputer
use super::PlatformProfile;

pub fn profile() -> PlatformProfile {
    PlatformProfile {
        tier: 4,
        name: "HPC / Supercomputer",
        target: "High-performance compute cluster / supercomputer",
        max_ram_bytes: 0,
        min_cpu_cores: 64,
        gpu_expected: true,
        hpc_network_expected: true,
    }
}
