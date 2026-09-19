//! Honest phase and HAL tier targets. These are deployment targets, not kernel claims.

/// Existing Phase 0 integrations are modeled as unavailable until a verified
/// implementation is wired in. They remain exported for the phase contract.
pub fn live_containerd() -> bool {
    false
}
pub fn live_mcp() -> bool {
    false
}
pub fn live_slurm() -> bool {
    false
}
pub fn live_whisper() -> bool {
    false
}

/// #196 HAL tiers: userspace-first deployment targets, not a GAIA kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalTier {
    T0,
    T1,
    T2,
    T3,
    T4,
}

pub fn hal_tiers() -> [HalTier; 5] {
    [
        HalTier::T0,
        HalTier::T1,
        HalTier::T2,
        HalTier::T3,
        HalTier::T4,
    ]
}
