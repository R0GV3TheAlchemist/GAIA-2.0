//! Backward-compatibility test suite for the frozen GAIA syscall ABI.
//!
//! These tests act as a ratchet: they will break at compile time or at
//! runtime if anyone renumbers an existing syscall, removes a variant,
//! or changes the `SyscallRequest` field layout — all of which are
//! breaking ABI changes that require a major version bump.

use gaia_kernel::syscall::{
    ABI_VERSION, GaiaSyscall, SyscallRequest, SyscallResult, dispatch,
};

// ── Helpers ──────────────────────────────────────────────────────────────────

fn req(syscall: u32) -> SyscallRequest {
    SyscallRequest { syscall, flags: 0, arg0: 0, arg1: 0, arg2: 0, arg3: 0 }
}

// ── ABI version ──────────────────────────────────────────────────────────────

#[test]
fn abi_version_constant_is_semver() {
    // Must parse as semver-compatible major.minor.patch.
    let parts: Vec<&str> = ABI_VERSION.split('.').collect();
    assert_eq!(parts.len(), 3, "ABI_VERSION must be major.minor.patch");
    for part in &parts {
        part.parse::<u32>().expect("each component must be a non-negative integer");
    }
}

#[test]
fn abi_major_version_is_1() {
    let major: u32 = ABI_VERSION.split('.').next().unwrap().parse().unwrap();
    assert_eq!(major, 1, "ABI major version must be 1 for the v1.0 frozen ABI");
}

// ── Frozen discriminant numbers ───────────────────────────────────────────────
// If ANY of these assertions fail, a renumbering has occurred — that is a
// breaking ABI change. Bump ABI_VERSION major and update this table.

#[test]
fn syscall_numbers_are_frozen() {
    assert_eq!(GaiaSyscall::IntentCreate    as u32, 0x01);
    assert_eq!(GaiaSyscall::IntentQuery     as u32, 0x02);
    assert_eq!(GaiaSyscall::ContextRecall   as u32, 0x03);
    assert_eq!(GaiaSyscall::AgentInvoke     as u32, 0x04);
    assert_eq!(GaiaSyscall::MemoryRead      as u32, 0x05);
    assert_eq!(GaiaSyscall::MemoryWrite     as u32, 0x06);
    assert_eq!(GaiaSyscall::ResourceDeclare as u32, 0x07);
    assert_eq!(GaiaSyscall::Observe         as u32, 0x08);
    assert_eq!(GaiaSyscall::CapabilityCheck as u32, 0x09);
}

// ── Dispatcher correctness ────────────────────────────────────────────────────

#[test]
fn all_v1_syscalls_dispatch_to_ok() {
    for n in 0x01u32..=0x09 {
        assert_eq!(
            dispatch(req(n)),
            SyscallResult::Ok,
            "v1 syscall {n:#04x} must dispatch to Ok"
        );
    }
}

#[test]
fn unknown_syscall_is_not_implemented() {
    // 0x00 is reserved; 0x0A onwards are unassigned in v1.0.
    for n in [0x00u32, 0x0A, 0x10, 0xFF, 0x1000, 0xFFFF_FFFF] {
        assert_eq!(
            dispatch(req(n)),
            SyscallResult::NotImplemented,
            "unknown syscall {n:#010x} must return NotImplemented, not panic"
        );
    }
}

#[test]
fn adding_a_new_syscall_number_does_not_break_existing_ones() {
    // Simulate a future caller that only knows about syscalls 0x01-0x09
    // but the kernel has added 0x0A. The old numbers still work.
    for n in 0x01u32..=0x09 {
        assert_eq!(dispatch(req(n)), SyscallResult::Ok);
    }
    // And the new (hypothetical) number gracefully returns NotImplemented
    // on this kernel build, not a panic or an error.
    assert_eq!(dispatch(req(0x0A)), SyscallResult::NotImplemented);
}

#[test]
fn renumbering_is_detectable_at_compile_time() {
    // This test encodes the expected discriminants as constants.
    // If a discriminant changes, the assert below will fail loudly.
    const INTENT_CREATE:    u32 = 0x01;
    const INTENT_QUERY:     u32 = 0x02;
    const CONTEXT_RECALL:   u32 = 0x03;
    const AGENT_INVOKE:     u32 = 0x04;
    const MEMORY_READ:      u32 = 0x05;
    const MEMORY_WRITE:     u32 = 0x06;
    const RESOURCE_DECLARE: u32 = 0x07;
    const OBSERVE:          u32 = 0x08;
    const CAPABILITY_CHECK: u32 = 0x09;

    assert_eq!(GaiaSyscall::IntentCreate    as u32, INTENT_CREATE);
    assert_eq!(GaiaSyscall::IntentQuery     as u32, INTENT_QUERY);
    assert_eq!(GaiaSyscall::ContextRecall   as u32, CONTEXT_RECALL);
    assert_eq!(GaiaSyscall::AgentInvoke     as u32, AGENT_INVOKE);
    assert_eq!(GaiaSyscall::MemoryRead      as u32, MEMORY_READ);
    assert_eq!(GaiaSyscall::MemoryWrite     as u32, MEMORY_WRITE);
    assert_eq!(GaiaSyscall::ResourceDeclare as u32, RESOURCE_DECLARE);
    assert_eq!(GaiaSyscall::Observe         as u32, OBSERVE);
    assert_eq!(GaiaSyscall::CapabilityCheck as u32, CAPABILITY_CHECK);
}

// ── SyscallRequest layout ─────────────────────────────────────────────────────

#[test]
fn syscall_request_fields_are_accessible() {
    // Construct by field name — if a field is renamed this won't compile.
    let r = SyscallRequest {
        syscall: GaiaSyscall::IntentCreate as u32,
        flags:   0,
        arg0:    1,
        arg1:    2,
        arg2:    3,
        arg3:    4,
    };
    assert_eq!(r.syscall, 0x01);
    assert_eq!(r.flags,   0);
    assert_eq!(r.arg0,    1);
    assert_eq!(r.arg1,    2);
    assert_eq!(r.arg2,    3);
    assert_eq!(r.arg3,    4);
}

#[test]
fn nonzero_flags_are_rejected() {
    let bad = SyscallRequest { syscall: 0x01, flags: 0xDEAD, arg0: 0, arg1: 0, arg2: 0, arg3: 0 };
    assert!(matches!(dispatch(bad), SyscallResult::InvalidRequest(_)));
}

#[test]
fn from_u32_returns_none_for_unknown_numbers() {
    assert_eq!(GaiaSyscall::from_u32(0x00), None);
    assert_eq!(GaiaSyscall::from_u32(0x0A), None);
    assert_eq!(GaiaSyscall::from_u32(0xFF), None);
}

#[test]
fn from_u32_returns_correct_variants() {
    assert_eq!(GaiaSyscall::from_u32(0x01), Some(GaiaSyscall::IntentCreate));
    assert_eq!(GaiaSyscall::from_u32(0x05), Some(GaiaSyscall::MemoryRead));
    assert_eq!(GaiaSyscall::from_u32(0x09), Some(GaiaSyscall::CapabilityCheck));
}
