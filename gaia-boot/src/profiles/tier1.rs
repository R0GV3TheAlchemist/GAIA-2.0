//! Tier 1 — SBC / Raspberry Pi
use super::PlatformProfile;

pub fn profile() -> PlatformProfile {
    PlatformProfile {
        tier: 1,
        name: "SBC / Raspberry Pi",
        target: "Raspberry Pi 4/5, Jetson Nano",
        max_ram_bytes: 8 * 1024 * 1024 * 1024,
        min_cpu_cores: 4,
        gpu_expected: false,
        hpc_network_expected: false,
    }
}
