//! Acceptance tests for `gaia-hal` — covers all 7 checklist items from #716.
//!
//! These tests are intentionally conservative: they run in pure userspace
//! (Tier 0/1) and do not require hardware privileges, kernel modules, or
//! specific physical devices. They prove the API surface compiles and
//! behaves correctly on the CI baseline (`x86_64-unknown-linux-gnu` and
//! `aarch64-unknown-linux-gnu`).

use gaia_hal::{
    cpu::{CpuArch, CpuInfo},
    platform::{HalTier, PlatformFeatures},
    sensors::{SensorBus, SensorKind, inject_mock_events},
    storage::{BlockDevice, FileBackedDevice},
    timers::MonotonicClock,
    HalCapabilities,
};
use tempfile::tempdir;

// ── Checklist item 1 & 2: crate compiles for x86_64 and aarch64 ─────────────
// Compilation success IS the test — if this file compiles the crate compiled.

#[test]
fn hal_capabilities_detect_does_not_panic() {
    let caps = HalCapabilities::detect();
    // Sanity-check the aggregate struct.
    assert!(caps.cpu.logical_cores >= 1);
    assert_eq!(caps.clock_resolution_ns, 1);
}

// ── Checklist item 3: CPU feature query returns correct flags ────────────────

#[test]
fn cpu_arch_is_known_and_consistent() {
    let info = CpuInfo::detect();
    let arch = info.features.arch;

    // On CI this will be X86_64 or Aarch64; never Other on a Tier-1 host.
    assert!(
        arch == CpuArch::X86_64
        || arch == CpuArch::Aarch64
        || arch == CpuArch::RiscV64,
        "unexpected arch: {arch:?}"
    );

    // Cross-check: x86_64 build should not report NEON.
    #[cfg(target_arch = "x86_64")]
    assert!(!info.features.simd.neon, "x86_64 must not report NEON");

    // Cross-check: aarch64 build should not report SSE2.
    #[cfg(target_arch = "aarch64")]
    assert!(!info.features.simd.sse2, "aarch64 must not report SSE2");
}

#[test]
fn cpu_logical_cores_is_positive() {
    let info = CpuInfo::detect();
    assert!(info.logical_cores >= 1, "must have at least one logical core");
}

// ── Checklist item 4: monotonic timer returns nanosecond-resolution timestamps

#[test]
fn monotonic_clock_is_nanosecond_resolution() {
    assert_eq!(MonotonicClock::resolution_ns(), 1);
}

#[test]
fn monotonic_clock_advances() {
    let clk = MonotonicClock::new();
    let t0  = clk.now_ns();
    std::thread::sleep(std::time::Duration::from_millis(2));
    let t1  = clk.now_ns();
    assert!(t1 > t0, "clock must advance over 2 ms");
    // Must have advanced at least 1 ms = 1_000_000 ns.
    assert!(t1 - t0 >= 1_000_000, "expected ≥1 ms elapsed, got {} ns", t1 - t0);
}

// ── Checklist item 5: block storage trait with file-backed device ─────────────

#[test]
fn file_backed_device_read_write_and_info() {
    let dir  = tempdir().unwrap();
    let path = dir.path().join("hal-test.img");
    let dev  = FileBackedDevice::open(&path, 512, 32, "hal-test").unwrap();

    assert_eq!(dev.info().block_size,  512);
    assert_eq!(dev.info().block_count,  32);
    assert_eq!(dev.info().capacity_bytes(), 512 * 32);

    let pattern: Vec<u8> = (0..512).map(|i| (i % 251) as u8).collect();
    dev.write_block(0,  &pattern).unwrap();
    dev.write_block(31, &pattern).unwrap();

    let mut buf = vec![0u8; 512];
    dev.read_block(0,  &mut buf).unwrap();
    assert_eq!(buf, pattern);
    dev.read_block(31, &mut buf).unwrap();
    assert_eq!(buf, pattern);
}

#[test]
fn file_backed_device_out_of_range_is_err() {
    let dir  = tempdir().unwrap();
    let path = dir.path().join("oor.img");
    let dev  = FileBackedDevice::open(&path, 512, 4, "oor").unwrap();
    assert!(dev.read_block(4, &mut vec![0u8; 512]).is_err());
    assert!(dev.write_block(4, &vec![0u8; 512]).is_err());
}

// ── Checklist item 6: sensor bus accepts typed events from a mock source ──────

#[test]
fn sensor_bus_accepts_mock_events() {
    let bus = SensorBus::new();
    inject_mock_events(&bus, SensorKind::Audio,       2);
    inject_mock_events(&bus, SensorKind::Imu,         3);
    inject_mock_events(&bus, SensorKind::Environmental, 1);

    assert_eq!(bus.pending(), 6);
    let events = bus.drain();
    assert_eq!(events.len(), 6);
    assert_eq!(bus.pending(), 0);

    let audio_count = events.iter().filter(|e| e.kind == SensorKind::Audio).count();
    let imu_count   = events.iter().filter(|e| e.kind == SensorKind::Imu).count();
    assert_eq!(audio_count, 2);
    assert_eq!(imu_count,   3);
}

// ── Checklist item 7: HAL tier matrix has 1:1 mapping to platform feature flags

#[test]
fn platform_features_map_to_tier_matrix() {
    let f = PlatformFeatures::detect();

    // T0 baseline is always present in GAIA userspace.
    assert!(f.sandbox,          "T0 requires sandbox");
    assert!(f.signed_bundles,   "T0 requires signed_bundles");
    assert!(f.capability_check, "T0 requires capability_check");

    // Max tier must be at least T0 on any host.
    assert!(f.max_tier() >= HalTier::T0);

    // On a standard Linux/macOS CI host we expect at least T1.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    assert!(
        f.max_tier() >= HalTier::T1,
        "Linux/macOS CI host should reach T1, got {:?}", f.max_tier()
    );
}

#[test]
fn tier_descriptions_match_hal_tiers_spec() {
    // Verify the Rust descriptions match the prose in hal-tiers.md.
    assert!(HalTier::T0.description().contains("Wasm agent"));
    assert!(HalTier::T1.description().contains("Desktop"));
    assert!(HalTier::T2.description().contains("Gateway"));
    assert!(HalTier::T3.description().contains("cluster"));
    assert!(HalTier::T4.description().contains("Federated"));
}
