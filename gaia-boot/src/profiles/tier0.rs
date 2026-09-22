//! Tier 0 — IoT / ARM Cortex-M (<64 KB RAM)
use super::PlatformProfile;

pub fn profile() -> PlatformProfile {
    PlatformProfile {
        tier: 0,
        name: "IoT / Cortex-M",
        target: "ARM Cortex-M series microcontrollers",
        max_ram_bytes: 64 * 1024,
        min_cpu_cores: 1,
        gpu_expected: false,
        hpc_network_expected: false,
    }
}
