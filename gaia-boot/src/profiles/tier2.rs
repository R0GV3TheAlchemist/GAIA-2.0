//! Tier 2 — Desktop / Server x86_64 + ARM64
use super::PlatformProfile;

pub fn profile() -> PlatformProfile {
    PlatformProfile {
        tier: 2,
        name: "Desktop / Server",
        target: "x86_64 and ARM64 desktop/server hardware",
        max_ram_bytes: 0, // unlimited
        min_cpu_cores: 2,
        gpu_expected: false,
        hpc_network_expected: false,
    }
}
