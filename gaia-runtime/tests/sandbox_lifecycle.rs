//! Acceptance tests for gaia-runtime sandbox (#719 / #740).
//!
//! These are pure unit tests — no WASM bytes are needed.  They validate:
//!
//!   - Engine + profile construction
//!   - [`GaiaResourceLimiter`] memory and table enforcement
//!   - [`SandboxManager::classify_trap`] error classification
//!   - Deny-by-default policy fields on [`SandboxProfile`]
//!   - `build_wasi_ctx` returns Ok for default (no-scratch) profile
//!   - `build_wasi_ctx` returns Ok for scratch-enabled profile

use gaia_runtime::{
    GaiaResourceLimiter, ResourceQuota, SandboxError,
    SandboxManager, SandboxProfile, GAIA_CAPABILITY_DENIED,
};
use wasmtime::ResourceLimiter;

// ── 1. Clean run ──────────────────────────────────────────────────────

#[test]
fn clean_run_profile_constructs() {
    let profile = SandboxProfile::default();
    let result = SandboxManager::new(profile);
    assert!(
        result.is_ok(),
        "SandboxManager::new must succeed with default profile: {:?}",
        result.err()
    );
}

// ── 2. OOM termination ───────────────────────────────────────────

#[test]
fn oom_termination_denies_oversized_memory_growth() {
    let quota = ResourceQuota {
        max_memory_bytes: 4 * 1024 * 1024, // 4 MiB ceiling
        ..ResourceQuota::default()
    };
    let mut limiter = GaiaResourceLimiter::new(quota);

    // Request within quota — must be allowed.
    let allowed = limiter.memory_growing(0, 2 * 1024 * 1024, None).unwrap();
    assert!(allowed, "memory growth within quota must be permitted");

    // Request beyond quota — must be denied (triggers OOM trap in Wasmtime).
    let denied = limiter.memory_growing(0, 8 * 1024 * 1024, None).unwrap();
    assert!(!denied, "memory growth beyond quota must be denied");
}

// ── 3. Timeout quota round-trips through profile() ──────────────────

#[test]
fn timeout_quota_propagates_through_profile_accessor() {
    let profile = SandboxProfile {
        quota: ResourceQuota {
            max_epochs: 1_000,
            ..ResourceQuota::default()
        },
        ..SandboxProfile::default()
    };
    let mgr = SandboxManager::new(profile).unwrap();
    assert_eq!(
        mgr.profile().quota.max_epochs,
        1_000,
        "max_epochs must round-trip through SandboxManager::profile()"
    );
}

// ── 4. Escape attempt → CapabilityDenied ──────────────────────────

#[test]
fn escape_attempt_maps_to_capability_denied() {
    let fake_trap = anyhow::anyhow!("permission denied (EACCES): /etc/passwd");
    let err = SandboxManager::classify_trap(&fake_trap);
    match &err {
        SandboxError::CapabilityDenied(code) => {
            assert_eq!(
                code, GAIA_CAPABILITY_DENIED,
                "CapabilityDenied must carry the canonical GAIA error code"
            );
        }
        other => panic!("expected CapabilityDenied, got: {other:?}"),
    }
}

// ── 5. Network blocked by default ──────────────────────────────

#[test]
fn network_blocked_by_default_profile() {
    let profile = SandboxProfile::default();
    assert!(
        !profile.network,
        "SandboxProfile::default() must not grant network capability"
    );
}

// ── 6. No scratch dir in default profile ────────────────────────

#[test]
fn no_scratch_dir_in_default_profile() {
    let profile = SandboxProfile::default();
    assert!(
        !profile.scratch_only_writes,
        "SandboxProfile::default() must not expose the scratch filesystem"
    );
}

// ── 7. Table growth limit ─────────────────────────────────────

#[test]
fn table_growing_enforces_sane_default_ceiling() {
    let mut limiter = GaiaResourceLimiter::new(ResourceQuota::default());

    let within = limiter.table_growing(0usize, 5_000usize, None).unwrap();
    assert!(within, "table growth <= 10 000 must be permitted");

    let at_limit = limiter.table_growing(0usize, 10_000usize, None).unwrap();
    assert!(at_limit, "table growth == 10 000 must be permitted (inclusive)");

    let over = limiter.table_growing(0usize, 10_001usize, None).unwrap();
    assert!(!over, "table growth > 10 000 must be denied");
}

// ── 8. OOM trap classification ─────────────────────────────────

#[test]
fn oom_trap_classified_correctly() {
    let err = anyhow::anyhow!("wasm trap: out of memory");
    let classified = SandboxManager::classify_trap(&err);
    assert!(
        matches!(classified, SandboxError::OomTermination),
        "'out of memory' trap must classify as OomTermination, got: {classified:?}"
    );
}

// ── 9. Timeout trap classification ─────────────────────────────

#[test]
fn timeout_trap_classified_correctly() {
    let err = anyhow::anyhow!("wasm trap: epoch interrupt");
    let classified = SandboxManager::classify_trap(&err);
    assert!(
        matches!(classified, SandboxError::Timeout),
        "epoch interrupt trap must classify as Timeout, got: {classified:?}"
    );
}

// ── 10. Unknown trap falls through to Trap variant ───────────────

#[test]
fn arbitrary_trap_maps_to_trap_variant() {
    let err = anyhow::anyhow!("some unexpected wasm trap: unreachable");
    let classified = SandboxManager::classify_trap(&err);
    assert!(
        matches!(classified, SandboxError::Trap(_)),
        "unrecognised trap must fall through to SandboxError::Trap, got: {classified:?}"
    );
}

// ── 11. build_wasi_ctx succeeds for default (no-scratch) profile ─

#[test]
fn build_wasi_ctx_ok_for_default_profile() {
    let profile = SandboxProfile::default();
    let mgr = SandboxManager::new(profile).unwrap();
    let result = mgr.build_wasi_ctx();
    assert!(
        result.is_ok(),
        "build_wasi_ctx must succeed for the default deny-by-default profile: {:?}",
        result.err()
    );
}

// ── 12. build_wasi_ctx succeeds when scratch_only_writes is true ─

#[test]
fn build_wasi_ctx_ok_with_scratch_enabled() {
    let profile = SandboxProfile {
        scratch_only_writes: true,
        ..SandboxProfile::default()
    };
    let mgr = SandboxManager::new(profile).unwrap();
    // Scratch dir is in temp; creation must not fail in a normal CI environment.
    let result = mgr.build_wasi_ctx();
    assert!(
        result.is_ok(),
        "build_wasi_ctx must succeed when scratch_only_writes is true: {:?}",
        result.err()
    );
}

// ── 13. mem_used tracks the last accepted allocation ────────────

#[test]
fn mem_used_tracks_last_accepted_allocation() {
    let quota = ResourceQuota {
        max_memory_bytes: 16 * 1024 * 1024,
        ..ResourceQuota::default()
    };
    let mut limiter = GaiaResourceLimiter::new(quota);

    let target = 8 * 1024 * 1024;
    let ok = limiter.memory_growing(0, target, None).unwrap();
    assert!(ok);
    assert_eq!(
        limiter.mem_used(),
        target,
        "mem_used() must reflect the last accepted desired size"
    );
}
