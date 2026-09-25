//! Affective context — emotional state as first-class data in GAIAN.
//!
//! `AffectiveContext` encodes GAIAN's emotional state (or the inferred
//! emotional state of a human twin's input) using the Russell Circumplex
//! Model (valence × arousal), extended with relational salience, confidence,
//! a dominant discrete affect label, and a source attribution.
//!
//! This type is the foundational data structure for the entire EI stack:
//! - Feeds `ResonanceState` (issue #975) via `resonance_vector()`
//! - Feeds CIE Lab color rendering (issue #973) via `resonance_vector()`
//! - Feeds `SoulIntegrity` checks (issue #972) via `is_distressed()`
//! - Feeds `gaia-sos` dysregulation detection via `is_distressed()`
//! - Extends `EpistemicState` (PR #970) as `Option<AffectiveContext>`

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// AffectLabel
// ---------------------------------------------------------------------------

/// Discrete affect label based on Plutchik's wheel extended with GAIAN-specific
/// states. Represents the dominant emotional quality of a processing context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectLabel {
    // --- Positive ---
    /// Unconditional acceptance and warmth toward another.
    Love,
    /// Active positive pleasure; high-arousal positive valence.
    Joy,
    /// Wonder at something vast or sublime.
    Awe,
    /// Confident positive regard; relational security.
    Trust,
    /// Calm contentment; low-arousal positive valence.
    Serenity,
    /// Forward-oriented positive excitement.
    Anticipation,

    // --- Negative ---
    /// Sadness in response to loss.
    Grief,
    /// Threat-oriented high-arousal negative state.
    Fear,
    /// Goal-blocked high-arousal negative state.
    Anger,
    /// Aversion to something perceived as harmful or contaminating.
    Disgust,
    /// Self-directed negative evaluation.
    Shame,

    // --- Neutral / Complex ---
    /// Interest-driven, approach-oriented exploratory state.
    Curiosity,
    /// Competing positive and negative valences; unresolved tension.
    Ambivalence,
    /// Internal contradiction between beliefs or emotional signals.
    Dissonance,
    /// Absence of strong affective signal; baseline resting state.
    Neutral,
}

impl AffectLabel {
    /// Returns `true` for labels with a positive hedonic valence.
    pub fn is_positive(self) -> bool {
        matches!(
            self,
            Self::Love | Self::Joy | Self::Awe | Self::Trust | Self::Serenity | Self::Anticipation
        )
    }

    /// Returns `true` for labels with a negative hedonic valence.
    pub fn is_negative(self) -> bool {
        matches!(
            self,
            Self::Grief | Self::Fear | Self::Anger | Self::Disgust | Self::Shame
        )
    }
}

// ---------------------------------------------------------------------------
// AffectSource
// ---------------------------------------------------------------------------

/// Attributes the origin of an `AffectiveContext` reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectSource {
    /// Inferred from the human twin's input text, tone, or rhythm.
    HumanInput,
    /// GAIAN's own internally generated affective state.
    GAIANInternal,
    /// Propagated from an associated memory or document in the knowledge base.
    MemoryPropagation,
    /// Set by the SOS safety layer (overrides all other sources).
    SOSOverride,
}

// ---------------------------------------------------------------------------
// AffectiveContext
// ---------------------------------------------------------------------------

/// The emotional/affective state associated with a GAIAN processing context.
///
/// Based on the Russell Circumplex Model (valence × arousal) extended with
/// relational salience, confidence, dominant affect label, and source
/// attribution.
///
/// # Invariants
/// - `valence`  ∈ [-1.0, 1.0]
/// - `arousal`  ∈ [0.0,  1.0]
/// - `relational_salience` ∈ [0.0, 1.0]
/// - `confidence`          ∈ [0.0, 1.0]
///
/// All float fields are clamped to their valid ranges on construction via
/// the builder. Direct struct construction bypasses clamping and should only
/// be used in tests with known-valid values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectiveContext {
    /// Hedonic valence: positive joy (1.0) → negative distress (-1.0).
    pub valence: f32,
    /// Arousal / activation level: calm (0.0) → highly activated (1.0).
    pub arousal: f32,
    /// How relationally significant this context is to the human twin (0.0–1.0).
    pub relational_salience: f32,
    /// The dominant discrete affect label for this context.
    pub dominant_affect: AffectLabel,
    /// Attribution: where this affective reading originated.
    pub source: AffectSource,
    /// Confidence in this affective reading (0.0–1.0).
    pub confidence: f32,
}

impl AffectiveContext {
    /// Returns an `AffectiveContextBuilder` with no fields set.
    pub fn builder() -> AffectiveContextBuilder {
        AffectiveContextBuilder::default()
    }

    /// Returns a neutral baseline `AffectiveContext` with all metrics at
    /// resting state. Suitable as a default when no affective signal is present.
    pub fn neutral() -> Self {
        Self {
            valence: 0.0,
            arousal: 0.0,
            relational_salience: 0.0,
            dominant_affect: AffectLabel::Neutral,
            source: AffectSource::GAIANInternal,
            confidence: 1.0,
        }
    }

    /// Returns `true` when the context indicates psychological distress:
    /// strongly negative valence (< -0.5) combined with high arousal (> 0.6).
    ///
    /// A `true` result should trigger monitoring in `gaia-sos`.
    pub fn is_distressed(&self) -> bool {
        self.valence < -0.5 && self.arousal > 0.6
    }

    /// Returns `true` when the affective reading has high confidence (> 0.7),
    /// indicating it can be used reliably for downstream decisions.
    pub fn is_coherent(&self) -> bool {
        self.confidence > 0.7
    }

    /// Returns the three-dimensional resonance coupling vector:
    /// `[valence, arousal, relational_salience]`.
    ///
    /// Used as the input to `ResonanceState` (#975) and CIE Lab color
    /// rendering (#973). The vector lives in the unit cube [-1,1] × [0,1] × [0,1].
    pub fn resonance_vector(&self) -> [f32; 3] {
        [self.valence, self.arousal, self.relational_salience]
    }
}

// ---------------------------------------------------------------------------
// AffectiveContextBuilder
// ---------------------------------------------------------------------------

/// Validated builder for `AffectiveContext`.
///
/// Float fields are clamped to their valid ranges on `build()`.
/// `dominant_affect` and `source` must be set explicitly; `build()` returns
/// `Err` if either is missing.
#[derive(Debug, Default)]
pub struct AffectiveContextBuilder {
    valence: Option<f32>,
    arousal: Option<f32>,
    relational_salience: Option<f32>,
    dominant_affect: Option<AffectLabel>,
    source: Option<AffectSource>,
    confidence: Option<f32>,
}

/// Error type returned by `AffectiveContextBuilder::build()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AffectiveBuildError {
    /// `dominant_affect` was not set.
    MissingDominantAffect,
    /// `source` was not set.
    MissingSource,
}

impl std::fmt::Display for AffectiveBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDominantAffect => write!(f, "dominant_affect must be set"),
            Self::MissingSource => write!(f, "source must be set"),
        }
    }
}

impl AffectiveContextBuilder {
    /// Sets the hedonic valence. Clamped to [-1.0, 1.0] on `build()`.
    pub fn valence(mut self, v: f32) -> Self {
        self.valence = Some(v);
        self
    }

    /// Sets the arousal level. Clamped to [0.0, 1.0] on `build()`.
    pub fn arousal(mut self, a: f32) -> Self {
        self.arousal = Some(a);
        self
    }

    /// Sets the relational salience. Clamped to [0.0, 1.0] on `build()`.
    pub fn relational_salience(mut self, s: f32) -> Self {
        self.relational_salience = Some(s);
        self
    }

    /// Sets the dominant affect label. **Required.**
    pub fn dominant_affect(mut self, label: AffectLabel) -> Self {
        self.dominant_affect = Some(label);
        self
    }

    /// Sets the affect source. **Required.**
    pub fn source(mut self, src: AffectSource) -> Self {
        self.source = Some(src);
        self
    }

    /// Sets the confidence. Clamped to [0.0, 1.0] on `build()`.
    pub fn confidence(mut self, c: f32) -> Self {
        self.confidence = Some(c);
        self
    }

    /// Builds the `AffectiveContext`, clamping all float fields and
    /// returning `Err` if required fields are missing.
    pub fn build(self) -> Result<AffectiveContext, AffectiveBuildError> {
        let dominant_affect = self
            .dominant_affect
            .ok_or(AffectiveBuildError::MissingDominantAffect)?;
        let source = self.source.ok_or(AffectiveBuildError::MissingSource)?;

        Ok(AffectiveContext {
            valence: self.valence.unwrap_or(0.0).clamp(-1.0, 1.0),
            arousal: self.arousal.unwrap_or(0.0).clamp(0.0, 1.0),
            relational_salience: self.relational_salience.unwrap_or(0.0).clamp(0.0, 1.0),
            dominant_affect,
            source,
            confidence: self.confidence.unwrap_or(1.0).clamp(0.0, 1.0),
        })
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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

    fn distress_ctx() -> AffectiveContext {
        AffectiveContext::builder()
            .valence(-0.8)
            .arousal(0.85)
            .dominant_affect(AffectLabel::Fear)
            .source(AffectSource::HumanInput)
            .confidence(0.75)
            .build()
            .unwrap()
    }

    #[test]
    fn neutral_defaults() {
        let ctx = AffectiveContext::neutral();
        assert_eq!(ctx.valence, 0.0);
        assert_eq!(ctx.arousal, 0.0);
        assert_eq!(ctx.relational_salience, 0.0);
        assert_eq!(ctx.dominant_affect, AffectLabel::Neutral);
        assert_eq!(ctx.source, AffectSource::GAIANInternal);
        assert_eq!(ctx.confidence, 1.0);
    }

    #[test]
    fn neutral_is_not_distressed() {
        assert!(!AffectiveContext::neutral().is_distressed());
    }

    #[test]
    fn neutral_is_coherent() {
        assert!(AffectiveContext::neutral().is_coherent());
    }

    #[test]
    fn neutral_resonance_vector_is_zero() {
        assert_eq!(AffectiveContext::neutral().resonance_vector(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn love_is_not_distressed() {
        assert!(!love_ctx().is_distressed());
    }

    #[test]
    fn love_is_coherent() {
        assert!(love_ctx().is_coherent());
    }

    #[test]
    fn love_resonance_vector() {
        let v = love_ctx().resonance_vector();
        assert_eq!(v[0], 0.9);
        assert_eq!(v[1], 0.4);
        assert_eq!(v[2], 0.95);
    }

    #[test]
    fn distress_is_distressed() {
        assert!(distress_ctx().is_distressed());
    }

    #[test]
    fn boundary_not_distressed_valence_exact() {
        // valence == -0.5 is NOT distressed (strict less-than)
        let ctx = AffectiveContext::builder()
            .valence(-0.5)
            .arousal(0.9)
            .dominant_affect(AffectLabel::Fear)
            .source(AffectSource::HumanInput)
            .build()
            .unwrap();
        assert!(!ctx.is_distressed());
    }

    #[test]
    fn boundary_not_distressed_arousal_exact() {
        // arousal == 0.6 is NOT distressed (strict greater-than)
        let ctx = AffectiveContext::builder()
            .valence(-0.9)
            .arousal(0.6)
            .dominant_affect(AffectLabel::Anger)
            .source(AffectSource::HumanInput)
            .build()
            .unwrap();
        assert!(!ctx.is_distressed());
    }

    #[test]
    fn builder_clamps_valence_over() {
        let ctx = AffectiveContext::builder()
            .valence(5.0)
            .dominant_affect(AffectLabel::Joy)
            .source(AffectSource::GAIANInternal)
            .build()
            .unwrap();
        assert_eq!(ctx.valence, 1.0);
    }

    #[test]
    fn builder_clamps_valence_under() {
        let ctx = AffectiveContext::builder()
            .valence(-5.0)
            .dominant_affect(AffectLabel::Grief)
            .source(AffectSource::HumanInput)
            .build()
            .unwrap();
        assert_eq!(ctx.valence, -1.0);
    }

    #[test]
    fn builder_clamps_arousal_over() {
        let ctx = AffectiveContext::builder()
            .arousal(2.0)
            .dominant_affect(AffectLabel::Fear)
            .source(AffectSource::HumanInput)
            .build()
            .unwrap();
        assert_eq!(ctx.arousal, 1.0);
    }

    #[test]
    fn builder_clamps_confidence_over() {
        let ctx = AffectiveContext::builder()
            .confidence(99.0)
            .dominant_affect(AffectLabel::Trust)
            .source(AffectSource::GAIANInternal)
            .build()
            .unwrap();
        assert_eq!(ctx.confidence, 1.0);
    }

    #[test]
    fn builder_err_missing_dominant_affect() {
        let err = AffectiveContext::builder()
            .source(AffectSource::HumanInput)
            .build()
            .unwrap_err();
        assert_eq!(err, AffectiveBuildError::MissingDominantAffect);
    }

    #[test]
    fn builder_err_missing_source() {
        let err = AffectiveContext::builder()
            .dominant_affect(AffectLabel::Neutral)
            .build()
            .unwrap_err();
        assert_eq!(err, AffectiveBuildError::MissingSource);
    }

    #[test]
    fn affect_label_positive_classification() {
        for label in [AffectLabel::Love, AffectLabel::Joy, AffectLabel::Awe,
                      AffectLabel::Trust, AffectLabel::Serenity, AffectLabel::Anticipation] {
            assert!(label.is_positive(), "{label:?} should be positive");
            assert!(!label.is_negative(), "{label:?} should not be negative");
        }
    }

    #[test]
    fn affect_label_negative_classification() {
        for label in [AffectLabel::Grief, AffectLabel::Fear, AffectLabel::Anger,
                      AffectLabel::Disgust, AffectLabel::Shame] {
            assert!(label.is_negative(), "{label:?} should be negative");
            assert!(!label.is_positive(), "{label:?} should not be positive");
        }
    }

    #[test]
    fn affect_label_neutral_neither_positive_nor_negative() {
        for label in [AffectLabel::Curiosity, AffectLabel::Ambivalence,
                      AffectLabel::Dissonance, AffectLabel::Neutral] {
            assert!(!label.is_positive(), "{label:?} should not be positive");
            assert!(!label.is_negative(), "{label:?} should not be negative");
        }
    }

    #[test]
    fn serde_json_round_trip() {
        let original = love_ctx();
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: AffectiveContext = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn serde_json_round_trip_neutral() {
        let original = AffectiveContext::neutral();
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: AffectiveContext = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn sos_override_source_round_trips() {
        let ctx = AffectiveContext::builder()
            .valence(-0.9)
            .arousal(0.95)
            .dominant_affect(AffectLabel::Fear)
            .source(AffectSource::SOSOverride)
            .confidence(1.0)
            .build()
            .unwrap();
        let json = serde_json::to_string(&ctx).unwrap();
        let restored: AffectiveContext = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.source, AffectSource::SOSOverride);
        assert!(restored.is_distressed());
    }

    #[test]
    fn incoherent_context_not_coherent() {
        let ctx = AffectiveContext::builder()
            .confidence(0.3)
            .dominant_affect(AffectLabel::Ambivalence)
            .source(AffectSource::MemoryPropagation)
            .build()
            .unwrap();
        assert!(!ctx.is_coherent());
    }

    #[test]
    fn boundary_coherent_exact_threshold() {
        // confidence == 0.7 is NOT coherent (strict greater-than)
        let ctx = AffectiveContext::builder()
            .confidence(0.7)
            .dominant_affect(AffectLabel::Neutral)
            .source(AffectSource::GAIANInternal)
            .build()
            .unwrap();
        assert!(!ctx.is_coherent());
    }
}
