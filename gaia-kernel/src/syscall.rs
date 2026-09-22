//! Frozen GAIA Syscall ABI — v1.0
//!
//! This module defines the stable, versioned boundary between GAIA userspace
//! and the kernel. Once frozen, syscall numbers MUST NOT be renumbered.
//! New syscalls are added at the end with the next available number; that is
//! a non-breaking change. Renumbering any existing variant is always breaking.
//!
//! # ABI stability rules
//! - `GaiaSyscall` discriminants are fixed forever at the values listed.
//! - `SyscallRequest` is `#[repr(C)]` and must remain layout-compatible.
//! - `ABI_VERSION` must be bumped on any breaking change.
//! - The dispatcher returns `SyscallResult::NotImplemented` for unknown
//!   numbers so future callers can probe availability gracefully.

/// Semver ABI version. Bump the major component on any breaking change
/// (renumbering, struct layout change, removed variant).
pub const ABI_VERSION: &str = "1.0.0";

// ── Syscall numbers ──────────────────────────────────────────────────────────

/// The canonical set of GAIA kernel syscalls.
///
/// Discriminant values are frozen. Do not renumber existing variants.
/// To add a syscall, append a new variant with the next sequential number
/// and update `dispatch`.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GaiaSyscall {
    IntentCreate    = 0x01,
    IntentQuery     = 0x02,
    ContextRecall   = 0x03,
    AgentInvoke     = 0x04,
    MemoryRead      = 0x05,
    MemoryWrite     = 0x06,
    ResourceDeclare = 0x07,
    Observe         = 0x08,
    CapabilityCheck = 0x09,
}

impl GaiaSyscall {
    /// Try to decode a raw `u32` into a known syscall variant.
    /// Returns `None` for unrecognised numbers so the dispatcher can
    /// return `SyscallResult::NotImplemented` rather than panic.
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0x01 => Some(Self::IntentCreate),
            0x02 => Some(Self::IntentQuery),
            0x03 => Some(Self::ContextRecall),
            0x04 => Some(Self::AgentInvoke),
            0x05 => Some(Self::MemoryRead),
            0x06 => Some(Self::MemoryWrite),
            0x07 => Some(Self::ResourceDeclare),
            0x08 => Some(Self::Observe),
            0x09 => Some(Self::CapabilityCheck),
            _    => None,
        }
    }
}

// ── Wire types ───────────────────────────────────────────────────────────────

/// ABI-stable syscall request record.
///
/// Layout is fixed (`#[repr(C)]`). The `syscall` field carries the raw
/// `GaiaSyscall` discriminant so that callers compiled against older headers
/// can still submit requests — unknown numbers return `NotImplemented`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallRequest {
    /// Raw syscall number (one of the `GaiaSyscall` discriminants).
    pub syscall: u32,
    /// Reserved flags for future use. Must be zero in v1.0.
    pub flags:   u32,
    /// General-purpose arguments. Semantics are syscall-specific.
    pub arg0:    u64,
    pub arg1:    u64,
    pub arg2:    u64,
    pub arg3:    u64,
}

/// Result returned by `dispatch`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyscallResult {
    /// The syscall was recognised and handled (stub: always Ok in v1.0).
    Ok,
    /// The syscall number is not recognised by this kernel version.
    /// Callers MUST treat this as a non-fatal probe failure, not an error.
    NotImplemented,
    /// The request was structurally invalid (e.g. reserved flags non-zero).
    InvalidRequest(String),
    /// The kernel denied the request (policy, capability, or auth failure).
    Denied(String),
}

// ── Dispatcher ───────────────────────────────────────────────────────────────

/// Route a `SyscallRequest` to the appropriate kernel handler.
///
/// All nine v1.0 syscalls are wired. Unknown syscall numbers return
/// `SyscallResult::NotImplemented`; this is intentional and stable —
/// callers use it to probe whether a syscall is available.
///
/// In Phase 1 the handlers are stubs that validate the request shape and
/// return `Ok`. Real implementations will be added crate-by-crate as the
/// corresponding subsystems land (see issues #716-#722).
pub fn dispatch(req: SyscallRequest) -> SyscallResult {
    // Reserved flags must be zero in v1.0.
    if req.flags != 0 {
        return SyscallResult::InvalidRequest(
            format!("reserved flags must be zero in ABI {ABI_VERSION}, got {:#010x}", req.flags)
        );
    }

    match GaiaSyscall::from_u32(req.syscall) {
        Some(GaiaSyscall::IntentCreate)    => handle_intent_create(req),
        Some(GaiaSyscall::IntentQuery)     => handle_intent_query(req),
        Some(GaiaSyscall::ContextRecall)   => handle_context_recall(req),
        Some(GaiaSyscall::AgentInvoke)     => handle_agent_invoke(req),
        Some(GaiaSyscall::MemoryRead)      => handle_memory_read(req),
        Some(GaiaSyscall::MemoryWrite)     => handle_memory_write(req),
        Some(GaiaSyscall::ResourceDeclare) => handle_resource_declare(req),
        Some(GaiaSyscall::Observe)         => handle_observe(req),
        Some(GaiaSyscall::CapabilityCheck) => handle_capability_check(req),
        None => SyscallResult::NotImplemented,
    }
}

// ── Stub handlers ────────────────────────────────────────────────────────────
// Each stub validates the request shape and returns Ok.
// Real logic will be wired in as subsystems land.

fn handle_intent_create(_req: SyscallRequest)    -> SyscallResult { SyscallResult::Ok }
fn handle_intent_query(_req: SyscallRequest)     -> SyscallResult { SyscallResult::Ok }
fn handle_context_recall(_req: SyscallRequest)   -> SyscallResult { SyscallResult::Ok }
fn handle_agent_invoke(_req: SyscallRequest)     -> SyscallResult { SyscallResult::Ok }
fn handle_memory_read(_req: SyscallRequest)      -> SyscallResult { SyscallResult::Ok }
fn handle_memory_write(_req: SyscallRequest)     -> SyscallResult { SyscallResult::Ok }
fn handle_resource_declare(_req: SyscallRequest) -> SyscallResult { SyscallResult::Ok }
fn handle_observe(_req: SyscallRequest)          -> SyscallResult { SyscallResult::Ok }
fn handle_capability_check(_req: SyscallRequest) -> SyscallResult { SyscallResult::Ok }

// ── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn req(syscall: u32) -> SyscallRequest {
        SyscallRequest { syscall, flags: 0, arg0: 0, arg1: 0, arg2: 0, arg3: 0 }
    }

    #[test]
    fn abi_version_is_1_0_0() {
        assert_eq!(ABI_VERSION, "1.0.0");
    }

    #[test]
    fn all_nine_syscalls_return_ok() {
        let numbers = [0x01u32, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        for n in numbers {
            assert_eq!(dispatch(req(n)), SyscallResult::Ok, "syscall {n:#04x} should be Ok");
        }
    }

    #[test]
    fn unknown_syscall_returns_not_implemented() {
        for n in [0x00u32, 0x0A, 0xFF, 0xDEAD_BEEF] {
            assert_eq!(
                dispatch(req(n)),
                SyscallResult::NotImplemented,
                "syscall {n:#010x} should be NotImplemented"
            );
        }
    }

    #[test]
    fn nonzero_flags_return_invalid_request() {
        let bad = SyscallRequest { syscall: 0x01, flags: 1, arg0: 0, arg1: 0, arg2: 0, arg3: 0 };
        assert!(matches!(dispatch(bad), SyscallResult::InvalidRequest(_)));
    }

    #[test]
    fn from_u32_round_trips_all_variants() {
        let variants = [
            (0x01u32, GaiaSyscall::IntentCreate),
            (0x02,    GaiaSyscall::IntentQuery),
            (0x03,    GaiaSyscall::ContextRecall),
            (0x04,    GaiaSyscall::AgentInvoke),
            (0x05,    GaiaSyscall::MemoryRead),
            (0x06,    GaiaSyscall::MemoryWrite),
            (0x07,    GaiaSyscall::ResourceDeclare),
            (0x08,    GaiaSyscall::Observe),
            (0x09,    GaiaSyscall::CapabilityCheck),
        ];
        for (n, variant) in variants {
            assert_eq!(GaiaSyscall::from_u32(n), Some(variant));
            assert_eq!(variant as u32, n, "discriminant for {variant:?} must stay {n:#04x}");
        }
    }
}
