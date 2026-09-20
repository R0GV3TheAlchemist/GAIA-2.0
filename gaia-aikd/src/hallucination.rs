//! #102 hallucination labels, uncertainty bands, and tier floor.
//! Not a live detector or conformal scorer. Not AIKD v1.0.

use crate::{AikdError, Tier};

// ── Existing types (unchanged) ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HallucinationKind {
    Factual,
    Temporal,
    Entity,
    Logical,
    Contextual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generation {
    pub text: String,
    pub flags: Vec<HallucinationKind>,
    pub tier: Tier,
}

impl Generation {
    pub fn from_fixture(text: &str, tier: Tier) -> Result<Self, AikdError> {
        let mut flags = vec![];
        let lower = text.to_ascii_lowercase();
        if lower.contains("born in 1800") {
            flags.push(HallucinationKind::Temporal);
        }
        if lower.contains("capital of france is berlin") {
            flags.push(HallucinationKind::Factual);
        }
        if tier == Tier::T5 && flags.is_empty() && !text.starts_with("verify:") {
            return Err(AikdError::NeedVerify);
        }
        Ok(Self {
            text: text.into(),
            flags,
            tier,
        })
    }
}

// ── New types for #102 interface wiring ──────────────────────────────────────

/// Five hallucination classes used by the interface uncertainty surface.
/// Distinct from `HallucinationKind` (internal label) — this is the
/// user-facing classification exposed at L6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HallucinationClass {
    Temporal,
    Factual,
    Fabricated,
    Overconfident,
    Contradictory,
}

/// A rendered hallucination warning. `Fabricated` at T5 → `NeedVerify`.
#[derive(Debug, Clone, PartialEq)]
pub struct HallucinationWarning {
    pub class: HallucinationClass,
    pub tier: u8,
    pub message: String,
}

/// Phase 1 uncertainty band. `lower` and `upper` are fixture floats.
/// Not a calibrated or conformal score.
#[derive(Debug, Clone, PartialEq)]
pub struct UncertaintyBand {
    pub lower: f32,
    pub upper: f32,
}

/// Return the Phase 1 fixture uncertainty band (maximum epistemic uncertainty).
/// Not calibrated. Not conformal prediction.
pub fn uncertainty_band() -> UncertaintyBand {
    UncertaintyBand { lower: 0.0, upper: 1.0 }
}

/// Return a tier-specific fixture uncertainty band.
/// T1: 0.0–0.2, T2: 0.1–0.4, T3: 0.3–0.6, T4: 0.5–0.8, T5: 0.8–1.0.
/// Out-of-range tier falls back to the full Phase 1 band.
pub fn propagate_uncertainty(tier: u8) -> UncertaintyBand {
    match tier {
        1 => UncertaintyBand { lower: 0.0, upper: 0.2 },
        2 => UncertaintyBand { lower: 0.1, upper: 0.4 },
        3 => UncertaintyBand { lower: 0.3, upper: 0.6 },
        4 => UncertaintyBand { lower: 0.5, upper: 0.8 },
        5 => UncertaintyBand { lower: 0.8, upper: 1.0 },
        _ => uncertainty_band(),
    }
}

/// Tier floor guard.
/// - Tier 5 → `Err(NeedVerify)` (soft ceiling: verification required)
/// - Tier 0 or 6+ → `Err(CannotKnow)` (out of range)
/// - Tier 1–4 → `Ok(tier)`
pub fn tier_floor(tier: u8) -> Result<u8, AikdError> {
    match tier {
        0 | 6..=u8::MAX => Err(AikdError::CannotKnow),
        5 => Err(AikdError::NeedVerify),
        t => Ok(t),
    }
}

/// Build a hallucination warning for the given class and tier.
/// `Fabricated` at tier 5 always returns `Err(NeedVerify)`.
pub fn hallucination_warning(
    class: HallucinationClass,
    tier: u8,
) -> Result<HallucinationWarning, AikdError> {
    if class == HallucinationClass::Fabricated && tier >= 5 {
        return Err(AikdError::NeedVerify);
    }
    let message = match class {
        HallucinationClass::Temporal => {
            format!("temporal hallucination detected at tier {tier}: cutoff-bounded")
        }
        HallucinationClass::Factual => {
            format!("factual hallucination detected at tier {tier}: citation required")
        }
        HallucinationClass::Fabricated => {
            format!("fabricated content at tier {tier}: verification required")
        }
        HallucinationClass::Overconfident => {
            format!("overconfident claim at tier {tier}: uncertainty band applies")
        }
        HallucinationClass::Contradictory => {
            format!("contradictory output at tier {tier}: context mismatch")
        }
    };
    Ok(HallucinationWarning { class, tier, message })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncertainty_band_full_range() {
        let b = uncertainty_band();
        assert_eq!(b.lower, 0.0);
        assert_eq!(b.upper, 1.0);
    }

    #[test]
    fn propagate_t1_tight() {
        let b = propagate_uncertainty(1);
        assert_eq!(b.lower, 0.0);
        assert_eq!(b.upper, 0.2);
    }

    #[test]
    fn propagate_t5_wide() {
        let b = propagate_uncertainty(5);
        assert_eq!(b.lower, 0.8);
        assert_eq!(b.upper, 1.0);
    }

    #[test]
    fn tier_floor_out_of_range() {
        assert_eq!(tier_floor(0), Err(AikdError::CannotKnow));
        assert_eq!(tier_floor(6), Err(AikdError::CannotKnow));
    }

    #[test]
    fn tier_floor_t5_needs_verify() {
        assert_eq!(tier_floor(5), Err(AikdError::NeedVerify));
    }

    #[test]
    fn tier_floor_t3_passes() {
        assert_eq!(tier_floor(3), Ok(3));
    }

    #[test]
    fn fabricated_t5_refused() {
        assert_eq!(
            hallucination_warning(HallucinationClass::Fabricated, 5),
            Err(AikdError::NeedVerify)
        );
    }

    #[test]
    fn temporal_t3_ok() {
        let w = hallucination_warning(HallucinationClass::Temporal, 3).unwrap();
        assert_eq!(w.tier, 3);
        assert!(w.message.contains("temporal"));
    }
}
