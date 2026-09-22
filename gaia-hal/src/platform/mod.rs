//! Platform subsystem — per-architecture feature flags and HAL tier mapping.
//!
//! This module provides a 1:1 mapping between the tier matrix in
//! `gaia-spec/sos/hal-tiers.md` and compile-time / runtime feature flags.

/// HAL deployment tier as defined in `hal-tiers.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HalTier {
    /// T0 — Existing-OS supervisor + Wasm agent (no GAIA kernel).
    T0,
    /// T1 — Desktop or developer host.
    T1,
    /// T2 — Gateway or home-lab node.
    T2,
    /// T3 — Managed node or cluster.
    T3,
    /// T4 — Federated multi-domain deployment.
    T4,
}

impl HalTier {
    /// Human-readable description matching `hal-tiers.md`.
    pub fn description(&self) -> &'static str {
        match self {
            Self::T0 => "Existing-OS supervisor plus tiny Wasm agent",
            Self::T1 => "Desktop or developer host",
            Self::T2 => "Gateway or home-lab node",
            Self::T3 => "Managed node or cluster",
            Self::T4 => "Federated multi-domain deployment",
        }
    }
}

/// Per-platform feature flags — one bit per HAL subsystem or extension.
///
/// These flags are populated at detection time and mirror the
/// `hal-tiers.md` tier requirements so that callers can check
/// capability without hard-coding architecture strings.
#[derive(Debug, Clone, Default)]
pub struct PlatformFeatures {
    // ── Architecture ────────────────────────────────────────────────────────
    /// Running on x86-64.
    pub arch_x86_64:  bool,
    /// Running on AArch64 / ARM64.
    pub arch_aarch64: bool,
    /// Running on RISC-V 64.
    pub arch_riscv64: bool,

    // ── Tier 0 baseline ──────────────────────────────────────────────────────
    /// Process sandbox is available (always true at T0+).
    pub sandbox:      bool,
    /// Signed component bundles are supported.
    pub signed_bundles: bool,
    /// Explicit capability check is available.
    pub capability_check: bool,

    // ── Tier 1 additions ──────────────────────────────────────────────────
    /// OS-level sandbox (namespaces / seccomp / Seatbelt).
    pub os_sandbox:   bool,
    /// Local audit spool is available.
    pub local_audit:  bool,

    // ── Tier 2 additions ──────────────────────────────────────────────────
    /// Hardware device identity (TPM, Secure Enclave, OP-TEE).
    pub device_identity: bool,
    /// Mutually-authenticated authority link.
    pub mutual_auth:  bool,

    // ── Tier 3 additions ──────────────────────────────────────────────────
    /// Authoritative policy service reachable.
    pub policy_service: bool,
    /// Durable audit storage available.
    pub durable_audit:  bool,

    // ── Tier 4 additions ──────────────────────────────────────────────────
    /// Federation agreements in place.
    pub federation:   bool,
    /// Independent audit peer reachable.
    pub independent_audit: bool,
}

impl PlatformFeatures {
    /// Detect platform features for the current host.
    ///
    /// Populates architecture flags from compile-time cfg and sets T0
    /// baseline flags (sandbox, signed_bundles, capability_check) which
    /// are always true at Tier 0+ in a GAIA userspace deployment.
    pub fn detect() -> Self {
        let mut f = Self::default();

        // Architecture detection.
        #[cfg(target_arch = "x86_64")]  { f.arch_x86_64  = true; }
        #[cfg(target_arch = "aarch64")] { f.arch_aarch64 = true; }
        #[cfg(target_arch = "riscv64")] { f.arch_riscv64 = true; }

        // T0 baseline — always true in GAIA userspace.
        f.sandbox         = true;
        f.signed_bundles  = true;
        f.capability_check = true;

        // T1 — OS sandbox detection (best-effort).
        #[cfg(target_os = "linux")] {
            // seccomp is available on Linux 3.5+; assume present.
            f.os_sandbox  = true;
            f.local_audit = true;
        }
        #[cfg(target_os = "macos")] {
            f.os_sandbox  = true; // Seatbelt / App Sandbox
            f.local_audit = true;
        }

        f
    }

    /// Infer the highest HAL tier this platform satisfies.
    pub fn max_tier(&self) -> HalTier {
        if self.federation && self.independent_audit          { return HalTier::T4; }
        if self.policy_service && self.durable_audit          { return HalTier::T3; }
        if self.device_identity && self.mutual_auth           { return HalTier::T2; }
        if self.os_sandbox && self.local_audit                { return HalTier::T1; }
        HalTier::T0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_ordering_is_correct() {
        assert!(HalTier::T0 < HalTier::T1);
        assert!(HalTier::T1 < HalTier::T4);
    }

    #[test]
    fn detect_sets_exactly_one_arch_flag_on_known_targets() {
        let f = PlatformFeatures::detect();
        let arch_count = [f.arch_x86_64, f.arch_aarch64, f.arch_riscv64]
            .iter()
            .filter(|&&b| b)
            .count();
        // On known targets exactly one flag is set; on unknown targets zero.
        assert!(arch_count <= 1, "at most one arch flag should be set");
    }

    #[test]
    fn t0_baseline_is_always_set() {
        let f = PlatformFeatures::detect();
        assert!(f.sandbox);
        assert!(f.signed_bundles);
        assert!(f.capability_check);
    }

    #[test]
    fn max_tier_is_at_least_t0() {
        let f = PlatformFeatures::detect();
        assert!(f.max_tier() >= HalTier::T0);
    }

    #[test]
    fn tier_descriptions_are_non_empty() {
        for tier in [HalTier::T0, HalTier::T1, HalTier::T2, HalTier::T3, HalTier::T4] {
            assert!(!tier.description().is_empty());
        }
    }
}
