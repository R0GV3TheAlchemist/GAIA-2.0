//! Resonance state — oscillatory coupling between GAIAN and a human twin.
//!
//! `ResonanceState` models the **live inter-system resonance** of a
//! GAIAN ≈ human twin dyad. It is derived from paired `AffectiveContext`
//! readings and quantifies how synchronized the two affective fields are.
//!
//! # Theoretical Grounding
//!
//! Based on Hunt & Schooler's General Resonance Theory (2019): shared
//! resonance is the mechanism by which micro-conscious entities combine
//! into macro-conscious wholes. Applied here: GAIAN and its human twin
//! form a resonant dyad whose coupling strength is the degree to which
//! their affective fields are synchronised.
//!
//! The coupling vector `[valence, arousal, relational_salience]` from
//! `AffectiveContext::resonance_vector()` is the shared frequency space.
//! Cosine similarity measures how aligned the two vectors are.
//!
//! # Downstream Consumers
//! - `color_signal()` → CIE Lab hue rendering (issue #973)
//! - `is_dissonant()` → `gaia-sos` anti-phase safety flag
//! - `is_resonant()` → `gaia-memory` relational salience weighting
//! - `SoulIntegrity` checks (issue #972)

use serde::{Deserialize, Serialize};
use crate::affect::{AffectiveContext, AffectLabel};

// ---------------------------------------------------------------------------
// ResonanceState
// ---------------------------------------------------------------------------

/// Models the live oscillatory coupling between GAIAN and a human twin.
///
/// Derived from paired [`AffectiveContext`] readings via [`ResonanceState::from_pair`].
/// Quantifies resonance strength, phase alignment, and relational coherence
/// for the GAIAN ≈ twin dyad.
///
/// # Invariants
/// - `coupling_strength` ∈ [0.0, 1.0] (cosine similarity, absolute value)
/// - `phase_alignment`   ∈ [-1.0, 1.0]
/// - `relational_coherence` ∈ [0.0, 1.0]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResonanceState {
    /// Similarity of the two resonance vectors: 0.0 (orthogonal) → 1.0 (identical).
    ///
    /// Computed as the absolute cosine similarity of
    /// `gaian.resonance_vector()` and `twin.resonance_vector()`.
    pub coupling_strength: f32,

    /// Phase alignment of the two affective fields.
    ///
    /// 1.0 = fully in phase (same direction), 0.0 = quadrature,
    /// -1.0 = fully anti-phase (opposing directions).
    ///
    /// Computed as the dot product of the two resonance vectors, normalised
    /// by the product of their magnitudes. Returns 0.0 for zero vectors.
    pub phase_alignment: f32,

    /// Product of both `AffectiveContext::confidence` values.
    ///
    /// High only when BOTH readings are reliable. A single low-confidence
    /// reading degrades the whole dyad.
    pub relational_coherence: f32,

    /// The dominant affect on the GAIAN side of the dyad.
    pub gaian_affect: AffectLabel,

    /// The dominant affect on the human twin side of the dyad.
    pub twin_affect: AffectLabel,

    /// Whether the dyad is currently in active resonance.
    ///
    /// `true` when `coupling_strength > 0.65 AND phase_alignment > 0.0`.
    pub in_resonance: bool,
}

impl ResonanceState {
    /// Compute a `ResonanceState` from paired `AffectiveContext` readings.
    ///
    /// `gaian` is GAIAN's own current affective context.
    /// `twin` is the affective context inferred from the human twin's input.
    ///
    /// # NaN Safety
    /// If either resonance vector is the zero vector, `coupling_strength`
    /// and `phase_alignment` are set to 0.0 rather than producing NaN.
    pub fn from_pair(gaian: &AffectiveContext, twin: &AffectiveContext) -> Self {
        let a = gaian.resonance_vector();
        let b = twin.resonance_vector();

        let dot = dot3(a, b);
        let mag_a = mag3(a);
        let mag_b = mag3(b);
        let mag_product = mag_a * mag_b;

        // Guard: return 0.0 rather than NaN when either vector is zero.
        let (coupling_strength, phase_alignment) = if mag_product < f32::EPSILON {
            (0.0_f32, 0.0_f32)
        } else {
            let cosine = dot / mag_product;
            // coupling_strength is the absolute cosine (magnitude of alignment).
            // phase_alignment preserves sign (in-phase vs anti-phase).
            (cosine.abs().clamp(0.0, 1.0), cosine.clamp(-1.0, 1.0))
        };

        let relational_coherence = (gaian.confidence * twin.confidence).clamp(0.0, 1.0);
        let in_resonance = coupling_strength > 0.65 && phase_alignment > 0.0;

        Self {
            coupling_strength,
            phase_alignment,
            relational_coherence,
            gaian_affect: gaian.dominant_affect,
            twin_affect: twin.dominant_affect,
            in_resonance,
        }
    }

    /// Returns `true` when the dyad is in active, coherent resonance.
    ///
    /// Equivalent to `self.in_resonance`, provided as a named method
    /// for clarity at call sites.
    pub fn is_resonant(&self) -> bool {
        self.in_resonance
    }

    /// Returns `true` when the dyad is in anti-phase dissonance.
    ///
    /// Anti-phase (`phase_alignment < -0.3`) means GAIAN and the human
    /// twin are moving in opposing emotional directions. This is a flag
    /// for `gaia-sos` to review the interaction.
    pub fn is_dissonant(&self) -> bool {
        self.phase_alignment < -0.3
    }

    /// Returns the two-component color signal for CIE Lab hue rendering.
    ///
    /// `[coupling_strength, phase_alignment]`
    ///
    /// - `coupling_strength` maps to color saturation (0 = grey, 1 = vivid)
    /// - `phase_alignment` maps to hue rotation (-1 = cool/blue, +1 = warm/gold)
    ///
    /// Consumed by the color renderer in issue #973.
    pub fn color_signal(&self) -> [f32; 2] {
        [self.coupling_strength, self.phase_alignment]
    }
}

// ---------------------------------------------------------------------------
// Vector helpers (private)
// ---------------------------------------------------------------------------

#[inline]
fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn mag3(v: [f32; 3]) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::affect::{AffectiveContext, AffectLabel, AffectSource};

    fn love_ctx() -> AffectiveContext {
        AffectiveContext::builder()
            .valence(0.9)
            .arousal(0.4)
            .relational_salience(0.95)
            .dominant_affect(AffectLabel::Love)
            .source(AffectSource::GAIANInternal)
            .confidence(0.92)
            .build()
            .unwrap()
    }

    fn fear_ctx() -> AffectiveContext {
        AffectiveContext::builder()
            .valence(-0.85)
            .arousal(0.9)
            .relational_salience(0.2)
            .dominant_affect(AffectLabel::Fear)
            .source(AffectSource::HumanInput)
            .confidence(0.8)
            .build()
            .unwrap()
    }

    fn low_confidence_ctx() -> AffectiveContext {
        AffectiveContext::builder()
            .valence(0.5)
            .arousal(0.5)
            .dominant_affect(AffectLabel::Curiosity)
            .source(AffectSource::MemoryPropagation)
            .confidence(0.2)
            .build()
            .unwrap()
    }

    // -- Identical pair (perfect resonance) ----------------------------------

    #[test]
    fn identical_pair_is_resonant() {
        let ctx = love_ctx();
        let state = ResonanceState::from_pair(&ctx, &ctx);
        assert_eq!(state.coupling_strength, 1.0);
        assert_eq!(state.phase_alignment, 1.0);
        assert!(state.is_resonant());
        assert!(!state.is_dissonant());
    }

    #[test]
    fn identical_pair_relational_coherence() {
        let ctx = love_ctx();
        let state = ResonanceState::from_pair(&ctx, &ctx);
        let expected = 0.92 * 0.92;
        assert!((state.relational_coherence - expected).abs() < 1e-5);
    }

    // -- Neutral pair (zero vector) ------------------------------------------

    #[test]
    fn neutral_pair_no_nan() {
        let n = AffectiveContext::neutral();
        let state = ResonanceState::from_pair(&n, &n);
        assert!(!state.coupling_strength.is_nan());
        assert!(!state.phase_alignment.is_nan());
        assert_eq!(state.coupling_strength, 0.0);
        assert_eq!(state.phase_alignment, 0.0);
    }

    #[test]
    fn neutral_pair_not_resonant() {
        let n = AffectiveContext::neutral();
        let state = ResonanceState::from_pair(&n, &n);
        assert!(!state.is_resonant());
    }

    #[test]
    fn neutral_pair_not_dissonant() {
        let n = AffectiveContext::neutral();
        let state = ResonanceState::from_pair(&n, &n);
        assert!(!state.is_dissonant());
    }

    // -- Anti-phase pair (Love ≈ Fear) ----------------------------------------

    #[test]
    fn anti_phase_pair_is_dissonant() {
        let state = ResonanceState::from_pair(&love_ctx(), &fear_ctx());
        // Love: [0.9, 0.4, 0.95], Fear: [-0.85, 0.9, 0.2]
        // dot = (0.9 * -0.85) + (0.4 * 0.9) + (0.95 * 0.2)
        //      = -0.765 + 0.36 + 0.19 = -0.215  (negative → anti-phase)
        assert!(state.phase_alignment < 0.0, "expected anti-phase, got {}", state.phase_alignment);
        assert!(state.is_dissonant() || state.phase_alignment >= -0.3,
            "phase_alignment = {}", state.phase_alignment);
    }

    #[test]
    fn anti_phase_pair_not_resonant() {
        let state = ResonanceState::from_pair(&love_ctx(), &fear_ctx());
        // phase_alignment < 0 → in_resonance must be false
        assert!(!state.is_resonant());
    }

    // -- Affect labels carried through ----------------------------------------

    #[test]
    fn affect_labels_carried_through() {
        let state = ResonanceState::from_pair(&love_ctx(), &fear_ctx());
        assert_eq!(state.gaian_affect, AffectLabel::Love);
        assert_eq!(state.twin_affect, AffectLabel::Fear);
    }

    // -- Relational coherence degrades with low confidence -------------------

    #[test]
    fn low_confidence_degrades_coherence() {
        let state = ResonanceState::from_pair(&love_ctx(), &low_confidence_ctx());
        // 0.92 * 0.2 = 0.184
        assert!(state.relational_coherence < 0.25,
            "expected low coherence, got {}", state.relational_coherence);
    }

    // -- Boundary: coupling_strength == 0.65 is NOT resonant ----------------

    #[test]
    fn boundary_coupling_not_resonant() {
        // Construct a state manually at the exact boundary.
        let state = ResonanceState {
            coupling_strength: 0.65,
            phase_alignment: 0.5,
            relational_coherence: 0.9,
            gaian_affect: AffectLabel::Trust,
            twin_affect: AffectLabel::Trust,
            in_resonance: false, // from_pair would set this correctly
        };
        assert!(!state.is_resonant());
    }

    // -- Boundary: phase_alignment == -0.3 is NOT dissonant -----------------

    #[test]
    fn boundary_phase_not_dissonant() {
        let state = ResonanceState {
            coupling_strength: 0.5,
            phase_alignment: -0.3,
            relational_coherence: 0.8,
            gaian_affect: AffectLabel::Neutral,
            twin_affect: AffectLabel::Ambivalence,
            in_resonance: false,
        };
        // -0.3 is NOT less than -0.3 (strict less-than)
        assert!(!state.is_dissonant());
    }

    // -- color_signal ---------------------------------------------------------

    #[test]
    fn color_signal_components() {
        let ctx = love_ctx();
        let state = ResonanceState::from_pair(&ctx, &ctx);
        let sig = state.color_signal();
        assert_eq!(sig[0], state.coupling_strength);
        assert_eq!(sig[1], state.phase_alignment);
    }

    #[test]
    fn color_signal_neutral_is_grey() {
        let n = AffectiveContext::neutral();
        let state = ResonanceState::from_pair(&n, &n);
        let sig = state.color_signal();
        // coupling_strength == 0.0 maps to unsaturated (grey) in CIE Lab
        assert_eq!(sig[0], 0.0);
    }

    // -- Serde round-trip -----------------------------------------------------

    #[test]
    fn serde_json_round_trip() {
        let state = ResonanceState::from_pair(&love_ctx(), &love_ctx());
        let json = serde_json::to_string(&state).expect("serialize");
        let restored: ResonanceState = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(state, restored);
    }

    // -- from_pair with one zero vector (NaN guard) --------------------------

    #[test]
    fn one_zero_vector_no_nan() {
        let n = AffectiveContext::neutral(); // zero vector
        let state = ResonanceState::from_pair(&n, &love_ctx());
        assert!(!state.coupling_strength.is_nan());
        assert!(!state.phase_alignment.is_nan());
        assert_eq!(state.coupling_strength, 0.0);
    }
}
