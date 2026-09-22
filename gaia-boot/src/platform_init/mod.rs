//! Phase 2 — Platform initialisation.
//!
//! Brings up peripherals in dependency order according to the active platform
//! tier.  The initialisation sequence is feature-flag gated so that Tier-0
//! firmware images do not pull in Tier-4 HPC subsystems.

use crate::bootloader::BootManifest;
use std::time::Instant;

/// Report produced by platform initialisation.
#[derive(Debug, Clone)]
pub struct PlatformInitReport {
    /// Ordered list of subsystems that were initialised.
    pub initialised: Vec<String>,
    /// Elapsed nanoseconds for the entire Phase 2.
    pub elapsed_ns: u64,
    /// Active platform tier (0–4).
    pub tier: u8,
}

/// Run Phase 2: initialise peripherals appropriate for the compiled tier.
pub fn run(_manifest: &BootManifest) -> PlatformInitReport {
    let t0 = Instant::now();
    let tier = active_tier();

    eprintln!("[gaia-boot] phase=2 status=platform_init tier={tier}");

    let mut initialised: Vec<String> = Vec::new();

    // Every tier initialises the interrupt controller and serial console.
    initialised.push(init_interrupt_controller());
    initialised.push(init_serial_console());

    #[cfg(any(feature = "tier1", feature = "tier2", feature = "tier3", feature = "tier4"))]
    {
        initialised.push(init_usb_controller());
        initialised.push(init_storage_controller());
    }

    #[cfg(any(feature = "tier2", feature = "tier3", feature = "tier4"))]
    {
        initialised.push(init_network_controller());
        initialised.push(init_display_controller());
    }

    #[cfg(any(feature = "tier3", feature = "tier4"))]
    {
        initialised.push(init_gpu_controller());
    }

    #[cfg(feature = "tier4")]
    {
        initialised.push(init_hpc_fabric());
        initialised.push(init_npu_array());
    }

    let elapsed_ns = t0.elapsed().as_nanos() as u64;
    eprintln!("[gaia-boot] phase=2 status=platform_init_complete elapsed_ns={elapsed_ns}");

    PlatformInitReport { initialised, elapsed_ns, tier }
}

fn active_tier() -> u8 {
    #[cfg(feature = "tier4")] { return 4; }
    #[cfg(feature = "tier3")] { return 3; }
    #[cfg(feature = "tier2")] { return 2; }
    #[cfg(feature = "tier1")] { return 1; }
    #[cfg(feature = "tier0")] { return 0; }
    #[allow(unreachable_code)] 2
}

fn init_interrupt_controller() -> String {
    eprintln!("[gaia-boot]   init: interrupt_controller");
    "interrupt_controller".to_string()
}
fn init_serial_console() -> String {
    eprintln!("[gaia-boot]   init: serial_console");
    "serial_console".to_string()
}
#[cfg(any(feature = "tier1", feature = "tier2", feature = "tier3", feature = "tier4"))]
fn init_usb_controller() -> String {
    eprintln!("[gaia-boot]   init: usb_controller");
    "usb_controller".to_string()
}
#[cfg(any(feature = "tier1", feature = "tier2", feature = "tier3", feature = "tier4"))]
fn init_storage_controller() -> String {
    eprintln!("[gaia-boot]   init: storage_controller");
    "storage_controller".to_string()
}
#[cfg(any(feature = "tier2", feature = "tier3", feature = "tier4"))]
fn init_network_controller() -> String {
    eprintln!("[gaia-boot]   init: network_controller");
    "network_controller".to_string()
}
#[cfg(any(feature = "tier2", feature = "tier3", feature = "tier4"))]
fn init_display_controller() -> String {
    eprintln!("[gaia-boot]   init: display_controller");
    "display_controller".to_string()
}
#[cfg(any(feature = "tier3", feature = "tier4"))]
fn init_gpu_controller() -> String {
    eprintln!("[gaia-boot]   init: gpu_controller");
    "gpu_controller".to_string()
}
#[cfg(feature = "tier4")]
fn init_hpc_fabric() -> String {
    eprintln!("[gaia-boot]   init: hpc_fabric");
    "hpc_fabric".to_string()
}
#[cfg(feature = "tier4")]
fn init_npu_array() -> String {
    eprintln!("[gaia-boot]   init: npu_array");
    "npu_array".to_string()
}
