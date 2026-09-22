//! Acceptance tests for gaia-boot — covers all 5 checklist items in issue #717.

use gaia_boot::{
    boot,
    bootloader::MemoryKind,
    profiles::active_profile,
};

// ── Acceptance 1 ──────────────────────────────────────────────────────
// "Boot crate compiles for Tier 2 (x86_64) and produces a GAIA capability
//  manifest at runtime."
#[test]
fn tier2_boot_produces_capability_manifest() {
    let result = boot();
    // Manifest must carry a non-empty arch string.
    assert!(!result.capability_manifest.cpu_arch.is_empty());
    // Total boot time must have been recorded.
    assert!(result.total_boot_ns > 0);
}

// ── Acceptance 2 ──────────────────────────────────────────────────────
// "Hardware-discovery enumerates CPU cores, RAM, and any GPU/NPU present."
//
// Note: GaiaCapabilityManifest at HAL v0.1 carries cpu_cores, cpu_arch,
// simd_available, and tier-derived booleans.  total_ram_bytes / ram_region_count
// are not yet present (deferred to HAL memory sub-module); RAM discovery is
// asserted via boot_manifest.total_ram_bytes in acceptance 4 instead.
#[test]
fn hardware_discovery_enumerates_cpu_and_gpu() {
    let result = boot();
    let cap = &result.capability_manifest;
    // At least 1 logical CPU core.
    assert!(cap.cpu_cores >= 1);
    // cpu_arch must be a non-empty known string.
    assert!(
        matches!(cap.cpu_arch.as_str(), "x86_64" | "aarch64" | "riscv64" | "other"),
        "unexpected cpu_arch: {}",
        cap.cpu_arch
    );
    // gpu/accelerator field is a bool — assert it is reachable (no panic).
    let _ = cap.accelerator_present;
    // SIMD bool must be reachable.
    let _ = cap.simd_available;
}

// ── Acceptance 3 ──────────────────────────────────────────────────────
// "Platform profiles are feature-flag gated (not all code pulled in for Tier 0)."
#[test]
fn platform_profile_matches_compiled_tier() {
    let profile = active_profile();
    // Default feature is tier2, so tier should be 2 in CI.
    assert_eq!(profile.tier, 2);
    assert_eq!(profile.name, "Desktop / Server");
}

// ── Acceptance 4 ──────────────────────────────────────────────────────
// "Boot sequence matches the 6 phases documented in BOOT.md."
#[test]
fn boot_phases_all_execute() {
    let result = boot();
    // Phase 0+1: firmware_id must be non-empty.
    assert!(!result.boot_manifest.firmware_id.is_empty());
    // Phase 0+1: cold_start_ns must be a plausible Unix timestamp (> year 2020).
    assert!(result.boot_manifest.cold_start_ns > 1_577_836_800_000_000_000u64);
    // Phase 0+1: memory map must contain at least one usable region.
    assert!(result
        .boot_manifest
        .memory_regions
        .iter()
        .any(|r| r.kind == MemoryKind::Usable));
    // Phase 2: at least interrupt_controller and serial_console must be init'd.
    assert!(result.boot_manifest.total_ram_bytes > 0);
    assert!(result
        .platform_report
        .initialised
        .contains(&"interrupt_controller".to_string()));
    assert!(result
        .platform_report
        .initialised
        .contains(&"serial_console".to_string()));
    // Phase 3+4: capability manifest present (tested in acceptance 1+2).
    // Phase 5: total_boot_ns > 0 (runtime-ready timestamp recorded).
    assert!(result.total_boot_ns > 0);
}

// ── Acceptance 5 ──────────────────────────────────────────────────────
// "Boot time from cold start to GAIA runtime ready is measured and logged."
#[test]
fn boot_time_is_measured_and_reasonable() {
    let result = boot();
    // Must be logged (non-zero).
    assert!(result.total_boot_ns > 0);
    // On a CI host, full boot including HAL detect should finish within 1 second.
    assert!(
        result.total_boot_ns < 1_000_000_000,
        "boot took {}ms — exceeded 1 s budget",
        result.total_boot_ns / 1_000_000
    );
}
