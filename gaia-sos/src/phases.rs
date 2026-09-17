//! Honest HAL tier targets for #196. Deployment targets, not kernel claims.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalTier { T0, T1, T2, T3, T4 }

pub fn hal_tiers() -> [HalTier; 5] { [HalTier::T0, HalTier::T1, HalTier::T2, HalTier::T3, HalTier::T4] }
