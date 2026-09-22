//! Stage 0 — Firmware hand-off.
//!
//! In a real implementation this would parse UEFI / BIOS hand-off structures.
//! Here we model the contract with a deterministic stub so that higher layers
//! and tests can rely on a stable interface.

use std::time::{SystemTime, UNIX_EPOCH};

/// Data received from firmware at the very start of Stage 0.
#[derive(Debug, Clone)]
pub struct FirmwareHandoff {
    /// Human-readable identifier for the firmware/bootloader.
    pub firmware_id: String,
    /// Nanoseconds since Unix epoch recorded at the start of Stage 0.
    pub cold_start_ns: u64,
}

/// Perform Stage 0: record the cold-start timestamp and capture firmware info.
pub fn handoff() -> FirmwareHandoff {
    let cold_start_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);

    let firmware_id = detect_firmware_id();

    eprintln!(
        "[gaia-boot] phase=0 status=firmware_handoff firmware={firmware_id} cold_start_ns={cold_start_ns}"
    );

    FirmwareHandoff { firmware_id, cold_start_ns }
}

fn detect_firmware_id() -> String {
    // In production: read SMBIOS/UEFI tables.
    // Stub: use compile-time arch to return a plausible identifier.
    #[cfg(target_arch = "x86_64")]
    { return "UEFI/x86_64-stub-v1.0".to_string(); }
    #[cfg(target_arch = "aarch64")]
    { return "UEFI/aarch64-stub-v1.0".to_string(); }
    #[cfg(target_arch = "riscv64")]
    { return "OpenSBI/riscv64-stub-v1.0".to_string(); }
    #[allow(unreachable_code)]
    "firmware/unknown-stub-v1.0".to_string()
}
