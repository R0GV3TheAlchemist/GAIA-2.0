//! `gaia-boot` — Bootloader, platform-init, and hardware-discovery for GAIA 2.0.
//!
//! Implements the 6-phase boot sequence described in `gaia-spec/sos/BOOT.md`:
//!
//! | Phase | Name                  | Module                  |
//! |-------|-----------------------|-------------------------|
//! | 0     | Firmware hand-off     | `bootloader::stage0`    |
//! | 1     | Memory map ingestion  | `bootloader::stage1`    |
//! | 2     | Platform init         | `platform_init`         |
//! | 3     | Hardware discovery    | `hardware_discovery`    |
//! | 4     | Capability manifest   | `hardware_discovery`    |
//! | 5     | Runtime ready         | `boot` (top-level)      |

pub mod bootloader;
pub mod hardware_discovery;
pub mod platform_init;
pub mod profiles;

use std::time::Instant;

pub use hardware_discovery::GaiaCapabilityManifest;
pub use bootloader::BootManifest;

/// Result of a complete GAIA boot sequence.
#[derive(Debug, Clone)]
pub struct BootResult {
    /// Phase 0-1 output: firmware + memory-map hand-off summary.
    pub boot_manifest: BootManifest,
    /// Phase 2 output: peripheral initialisation report.
    pub platform_report: platform_init::PlatformInitReport,
    /// Phase 3-4 output: enumerated hardware capabilities.
    pub capability_manifest: GaiaCapabilityManifest,
    /// Total elapsed nanoseconds from cold start to runtime-ready.
    pub total_boot_ns: u64,
}

/// Run the full 6-phase GAIA boot sequence and return a `BootResult`.
///
/// In production this would be driven by the bootloader entry point;
/// here it is a library function callable from tests and the kernel.
pub fn boot() -> BootResult {
    let t0 = Instant::now();

    // Phase 0 + 1 — firmware hand-off and memory-map ingestion.
    let boot_manifest = bootloader::run();

    // Phase 2 — peripheral initialisation.
    let platform_report = platform_init::run(&boot_manifest);

    // Phase 3 + 4 — hardware discovery and capability manifest.
    let capability_manifest = hardware_discovery::discover();

    // Phase 5 — runtime ready.
    let total_boot_ns = t0.elapsed().as_nanos() as u64;

    log_boot_complete(total_boot_ns);

    BootResult {
        boot_manifest,
        platform_report,
        capability_manifest,
        total_boot_ns,
    }
}

fn log_boot_complete(ns: u64) {
    // Lightweight structured log — no external logging dependency.
    eprintln!(
        "[gaia-boot] phase=5 status=runtime_ready boot_time_ns={ns} boot_time_ms={}",
        ns / 1_000_000
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_returns_result() {
        let result = boot();
        assert!(result.total_boot_ns < 60_000_000_000);
    }

    #[test]
    fn default_profile_is_tier2() {
        let p = profiles::active_profile();
        assert_eq!(p.tier, 2);
        assert_eq!(p.name, "Desktop / Server");
    }

    #[test]
    fn five_profiles_have_distinct_tiers() {
        let tiers = [
            profiles::tier0::profile().tier,
            profiles::tier1::profile().tier,
            profiles::tier2::profile().tier,
            profiles::tier3::profile().tier,
            profiles::tier4::profile().tier,
        ];
        assert_eq!(tiers, [0, 1, 2, 3, 4]);
    }
}
