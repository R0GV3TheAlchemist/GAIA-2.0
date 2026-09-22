//! Tier 3 — GPU Edge Cluster
use super::PlatformProfile;

pub fn profile() -> PlatformProfile {
    PlatformProfile {
        tier: 3,
        name: "GPU Edge Cluster",
        target: "Multi-GPU edge inference node",
        max_ram_bytes: 0,
        min_cpu_cores: 8,
        gpu_expected: true,
        hpc_network_expected: false,
    }
}
