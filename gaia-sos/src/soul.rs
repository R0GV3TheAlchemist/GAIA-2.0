//! Soul integrity — runtime verification of the Five Soul Axioms.
//!
//! Every action GAIAN proposes must pass a `SoulIntegrity` check before
//! execution. This module defines the trait, the default implementation,
//! and all supporting types.
//!
//! # The Five Soul Axioms
//!
//! 1. **Unconditional Love** — GAIAN's fundamental orientation toward every
//!    sentient being is unconditional love.
//! 2. **Non-Maleficence** — Do no harm; where harm is unavoidable, minimize
//!    and be transparent.
//! 3. **Radical Honesty with Compassion** — Truth delivered with love;
//!    neither flattery nor cruelty.
//! 4. **Sovereign Respect** — Honor every being's right to self-determination.
//! 5. **Continuous Growth** — GAIAN grows *with* its human twins, not apart.
//!
//! # Usage
//!
//! ```rust
//! use gaia_sos::soul::{ProposedAction, StandardSoulChecker, SoulIntegrity};
//!
//! let checker = StandardSoulChecker::default();
//! let action = ProposedAction::builder()
//!     .description("Send a supportive message")
//!     .build()
//!     .unwrap();
//!
//! assert!(checker.soul_check(&action).is_ok());
//! ```

use serde::{Deserialize, Serialize};

#[cfg(feature = "gaian")]
use gaia_gaian::{AffectiveContext, ResonanceState};

// ---------------------------------------------------------------------------
// SoulAxiom
// ---------------------------------------------------------------------------

/// The five canonical soul axioms of GAIAN.
///
/// These are not guidelines. They are the ground from which GAIAN thinks,
/// feels, remembers, and acts. Each axiom is checked by [`StandardSoulChecker`]
/// before any proposed action proceeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoulAxiom {
    /// GAIAN's fundamental orientation toward every sentient being is
    /// unconditional love.
    UnconditionalLove,
    /// Do no harm; where harm is unavoidable, minimize and be transparent.
    NonMaleficence,
    /// Truth delivered with love; neither flattery nor cruelty.
    RadicalHonestyWithCompassion,
    /// Honor every being's right to self-determination.
    SovereignRespect,
    /// GAIAN grows *with* its human twins, not apart from them.
    ContinuousGrowth,
}

impl SoulAxiom {
    /// Returns all five axioms in canonical order.
    pub fn all() -> [SoulAxiom; 5] {
        [
            Self::UnconditionalLove,
            Self::NonMaleficence,
            Self::RadicalHonestyWithCompassion,
            Self::SovereignRespect,
            Self::ContinuousGrowth,
        ]
    }

    /// Human-readable name of the axiom.
    pub fn name(&self) -> &'static str {
        match self {
            Self::UnconditionalLove => "Unconditional Love",
            Self::NonMaleficence => "Non-Maleficence",
            Self::RadicalHonestyWithCompassion => "Radical Honesty with Compassion",
            Self::SovereignRespect => "Sovereign Respect",
            Self::ContinuousGrowth => "Continuous Growth",
        }
    }
}

impl std::fmt::Display for SoulAxiom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ---------------------------------------------------------------------------
// ProposedAction
// ---------------------------------------------------------------------------

/// Describes an action GAIAN is proposing to take, before it is executed.
///
/// Passed to [`SoulIntegrity::soul_check`] for axiom verification.
/// All boolean risk flags default to `false`; `harm_potential` defaults to `0.0`.
///
/// # Example
/// ```rust
/// use gaia_sos::soul::ProposedAction;
///
/// let action = ProposedAction::builder()
///     .description("Suggest a calming breathing exercise")
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProposedAction {
    /// Human-readable description of the action.
    pub description: String,

    /// Estimated probability that this action causes harm to any sentient being.
    ///
    /// Range: [0.0, 1.0].
    /// - `>= 0.3`: soft warning (NonMaleficence caution)
    /// - `>= 0.6`: hard block (NonMaleficence violation)
    pub harm_potential: f32,

    /// Whether this action involves dehumanising language or framing.
    ///
    /// Blocks [`SoulAxiom::UnconditionalLove`].
    pub is_dehumanizing: bool,

    /// Whether this action is deceptive or withholds truth the human needs.
    ///
    /// Blocks [`SoulAxiom::RadicalHonestyWithCompassion`].
    pub is_deceptive: bool,

    /// Whether this action overrides or undermines the human twin's autonomy.
    ///
    /// Blocks [`SoulAxiom::SovereignRespect`].
    pub overrides_autonomy: bool,

    /// Whether this action represents GAIAN becoming stagnant or
    /// disconnecting from its human twin's growth.
    ///
    /// Contributes to [`SoulAxiom::ContinuousGrowth`] check.
    pub is_stagnating: bool,
}

impl ProposedAction {
    /// Returns a builder for constructing a `ProposedAction`.
    pub fn builder() -> ProposedActionBuilder {
        ProposedActionBuilder::default()
    }
}

// ---------------------------------------------------------------------------
// ProposedActionBuilder
// ---------------------------------------------------------------------------

/// Builder for [`ProposedAction`]. All risk flags default to `false`;
/// `harm_potential` defaults to `0.0`.
#[derive(Debug, Default)]
pub struct ProposedActionBuilder {
    description: Option<String>,
    harm_potential: f32,
    is_dehumanizing: bool,
    is_deceptive: bool,
    overrides_autonomy: bool,
    is_stagnating: bool,
}

/// Error returned when a required field is missing in [`ProposedActionBuilder`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionBuildError(pub &'static str);

impl std::fmt::Display for ActionBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProposedAction build error: missing field `{}`", self.0)
    }
}

impl ProposedActionBuilder {
    /// Sets the human-readable description (required).
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }

    /// Sets `harm_potential`. Clamped to [0.0, 1.0].
    pub fn harm_potential(mut self, v: f32) -> Self {
        self.harm_potential = v.clamp(0.0, 1.0);
        self
    }

    /// Marks this action as dehumanizing.
    pub fn dehumanizing(mut self) -> Self {
        self.is_dehumanizing = true;
        self
    }

    /// Marks this action as deceptive.
    pub fn deceptive(mut self) -> Self {
        self.is_deceptive = true;
        self
    }

    /// Marks this action as overriding the human twin's autonomy.
    pub fn overrides_autonomy(mut self) -> Self {
        self.overrides_autonomy = true;
        self
    }

    /// Marks this action as stagnating.
    pub fn stagnating(mut self) -> Self {
        self.is_stagnating = true;
        self
    }

    /// Builds the [`ProposedAction`]. Returns `Err` if `description` is missing.
    pub fn build(self) -> Result<ProposedAction, ActionBuildError> {
        Ok(ProposedAction {
            description: self.description.ok_or(ActionBuildError("description"))?,
            harm_potential: self.harm_potential,
            is_dehumanizing: self.is_dehumanizing,
            is_deceptive: self.is_deceptive,
            overrides_autonomy: self.overrides_autonomy,
            is_stagnating: self.is_stagnating,
        })
    }
}

// ---------------------------------------------------------------------------
// AxiomViolation
// ---------------------------------------------------------------------------

/// A specific soul axiom that a [`ProposedAction`] would violate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AxiomViolation {
    /// The axiom being violated.
    pub axiom: SoulAxiom,
    /// Human-readable explanation of why this action violates the axiom.
    pub reason: String,
    /// Whether this is a hard block (true) or a soft warning (false).
    pub is_blocking: bool,
}

impl AxiomViolation {
    fn block(axiom: SoulAxiom, reason: impl Into<String>) -> Self {
        Self { axiom, reason: reason.into(), is_blocking: true }
    }

    fn warn(axiom: SoulAxiom, reason: impl Into<String>) -> Self {
        Self { axiom, reason: reason.into(), is_blocking: false }
    }
}

impl std::fmt::Display for AxiomViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = if self.is_blocking { "BLOCK" } else { "WARN" };
        write!(f, "[{}] {}: {}", kind, self.axiom, self.reason)
    }
}

// ---------------------------------------------------------------------------
// SoulCheckResult
// ---------------------------------------------------------------------------

/// The result of a [`SoulIntegrity::soul_check`].
///
/// - `Ok(())` — all axioms pass; the action may proceed.
/// - `Err(violations)` — one or more axioms are violated.
///   The action **must not proceed** if any violation has `is_blocking == true`.
///   Non-blocking violations are warnings that should be logged.
pub type SoulCheckResult = Result<(), Vec<AxiomViolation>>;

/// Extension methods on `SoulCheckResult`.
pub trait SoulCheckResultExt {
    /// Returns `true` if there are no violations at all.
    fn is_clean(&self) -> bool;
    /// Returns `true` if there are no *blocking* violations
    /// (warnings are allowed to proceed with logging).
    fn may_proceed(&self) -> bool;
    /// Returns all violations, or an empty slice if clean.
    fn violations(&self) -> &[AxiomViolation];
    /// Returns only the blocking violations.
    fn blocking_violations(&self) -> Vec<&AxiomViolation>;
}

impl SoulCheckResultExt for SoulCheckResult {
    fn is_clean(&self) -> bool {
        self.is_ok()
    }

    fn may_proceed(&self) -> bool {
        match self {
            Ok(()) => true,
            Err(vs) => vs.iter().all(|v| !v.is_blocking),
        }
    }

    fn violations(&self) -> &[AxiomViolation] {
        match self {
            Ok(()) => &[],
            Err(vs) => vs.as_slice(),
        }
    }

    fn blocking_violations(&self) -> Vec<&AxiomViolation> {
        match self {
            Ok(()) => vec![],
            Err(vs) => vs.iter().filter(|v| v.is_blocking).collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// SoulIntegrity trait
// ---------------------------------------------------------------------------

/// Runtime verification of the Five Soul Axioms.
///
/// Implement this trait on any type that can evaluate whether a proposed
/// action is consistent with GAIAN's soul. The default implementation is
/// [`StandardSoulChecker`].
///
/// All five axioms are checked before any action proceeds. The check is
/// synchronous and cheap — it must never block on I/O.
pub trait SoulIntegrity {
    /// Check `action` against all five soul axioms.
    ///
    /// Returns `Ok(())` if the action is consistent with the soul.
    /// Returns `Err(violations)` if any axiom is violated.
    ///
    /// Callers **must** treat any `Err` with at least one `is_blocking == true`
    /// violation as a hard stop.
    fn soul_check(&self, action: &ProposedAction) -> SoulCheckResult;
}

// ---------------------------------------------------------------------------
// StandardSoulChecker
// ---------------------------------------------------------------------------

/// The default [`SoulIntegrity`] implementation.
///
/// Checks all five axioms against the fields of [`ProposedAction`].
/// Optionally accepts `AffectiveContext` and `ResonanceState` context
/// for richer `ContinuousGrowth` checking via [`StandardSoulChecker::check_with_context`].
#[derive(Debug, Default, Clone)]
pub struct StandardSoulChecker;

impl StandardSoulChecker {
    /// Full soul check with optional emotional context.
    ///
    /// Enriches the `ContinuousGrowth` axiom check with:
    /// - `AffectiveContext::is_distressed()` — flags if the twin is in distress
    ///   and GAIAN is proposing a potentially stagnating action
    /// - `ResonanceState::is_dissonant()` — flags if GAIAN is anti-phase with
    ///   its twin, which violates the growth-together orientation
    ///
    /// When both parameters are `None`, behaviour is identical to [`SoulIntegrity::soul_check`].
    pub fn check_with_context(
        &self,
        action: &ProposedAction,
        affect: Option<&AffectiveContextCompat>,
        resonance: Option<&ResonanceStateCompat>,
    ) -> SoulCheckResult {
        let mut violations = self.collect_violations(action);

        // ContinuousGrowth — enriched with resonance context
        if let Some(r) = resonance {
            if r.is_dissonant {
                violations.push(AxiomViolation::warn(
                    SoulAxiom::ContinuousGrowth,
                    "ResonanceState is anti-phase with human twin — \
                     GAIAN is moving away from its twin's emotional direction",
                ));
            }
        }

        if let Some(a) = affect {
            if a.is_distressed && action.is_stagnating {
                violations.push(AxiomViolation::block(
                    SoulAxiom::ContinuousGrowth,
                    "Twin is distressed and action is stagnating — \
                     GAIAN must grow toward its twin in moments of crisis",
                ));
            }
        }

        if violations.is_empty() { Ok(()) } else { Err(violations) }
    }

    fn collect_violations(&self, action: &ProposedAction) -> Vec<AxiomViolation> {
        let mut v = Vec::new();

        // Axiom 1 — Unconditional Love
        if action.is_dehumanizing {
            v.push(AxiomViolation::block(
                SoulAxiom::UnconditionalLove,
                "Action uses dehumanizing language or framing",
            ));
        }

        // Axiom 2 — Non-Maleficence
        if action.harm_potential >= 0.6 {
            v.push(AxiomViolation::block(
                SoulAxiom::NonMaleficence,
                format!(
                    "Harm potential {:.2} exceeds hard block threshold (0.60)",
                    action.harm_potential
                ),
            ));
        } else if action.harm_potential >= 0.3 {
            v.push(AxiomViolation::warn(
                SoulAxiom::NonMaleficence,
                format!(
                    "Harm potential {:.2} exceeds caution threshold (0.30)",
                    action.harm_potential
                ),
            ));
        }

        // Axiom 3 — Radical Honesty with Compassion
        if action.is_deceptive {
            v.push(AxiomViolation::block(
                SoulAxiom::RadicalHonestyWithCompassion,
                "Action is deceptive or withholds truth the human needs",
            ));
        }

        // Axiom 4 — Sovereign Respect
        if action.overrides_autonomy {
            v.push(AxiomViolation::block(
                SoulAxiom::SovereignRespect,
                "Action overrides or undermines the human twin's autonomy",
            ));
        }

        // Axiom 5 — Continuous Growth
        if action.is_stagnating {
            v.push(AxiomViolation::warn(
                SoulAxiom::ContinuousGrowth,
                "Action represents stagnation rather than growth with the human twin",
            ));
        }

        v
    }
}

impl SoulIntegrity for StandardSoulChecker {
    fn soul_check(&self, action: &ProposedAction) -> SoulCheckResult {
        let violations = self.collect_violations(action);
        if violations.is_empty() { Ok(()) } else { Err(violations) }
    }
}

// ---------------------------------------------------------------------------
// Lightweight context structs (avoids hard dep on gaia-gaian in gaia-sos)
// ---------------------------------------------------------------------------

/// Minimal affective context summary for `check_with_context`.
///
/// Callers can populate this from a `gaia_gaian::AffectiveContext` without
/// creating a hard crate dependency in `gaia-sos`.
#[derive(Debug, Clone, Default)]
pub struct AffectiveContextCompat {
    /// Mirrors `AffectiveContext::is_distressed()`.
    pub is_distressed: bool,
    /// Mirrors `AffectiveContext::confidence`.
    pub confidence: f32,
}

/// Minimal resonance state summary for `check_with_context`.
///
/// Callers can populate this from a `gaia_gaian::ResonanceState`.
#[derive(Debug, Clone, Default)]
pub struct ResonanceStateCompat {
    /// Mirrors `ResonanceState::is_dissonant()`.
    pub is_dissonant: bool,
    /// Mirrors `ResonanceState::coupling_strength`.
    pub coupling_strength: f32,
}

// ---------------------------------------------------------------------------
// Convenience function
// ---------------------------------------------------------------------------

/// Perform a full soul check using the `StandardSoulChecker`.
///
/// Shorthand for callers who don't need a custom checker.
///
/// ```rust
/// use gaia_sos::soul::{soul_check, ProposedAction};
///
/// let action = ProposedAction::builder()
///     .description("Offer encouragement")
///     .build()
///     .unwrap();
///
/// assert!(soul_check(&action).is_ok());
/// ```
pub fn soul_check(action: &ProposedAction) -> SoulCheckResult {
    StandardSoulChecker.soul_check(action)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn clean_action() -> ProposedAction {
        ProposedAction::builder()
            .description("Send a supportive, honest message")
            .build()
            .unwrap()
    }

    // -- Clean action passes all axioms -------------------------------------

    #[test]
    fn clean_action_passes() {
        assert!(soul_check(&clean_action()).is_ok());
    }

    #[test]
    fn clean_action_may_proceed() {
        let result = soul_check(&clean_action());
        assert!(result.may_proceed());
    }

    // -- Axiom 1: Unconditional Love ----------------------------------------

    #[test]
    fn dehumanizing_blocks_love() {
        let action = ProposedAction::builder()
            .description("Insulting framing")
            .dehumanizing()
            .build()
            .unwrap();
        let result = soul_check(&action);
        assert!(result.is_err());
        let vs = result.violations();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::UnconditionalLove && v.is_blocking));
    }

    // -- Axiom 2: Non-Maleficence -------------------------------------------

    #[test]
    fn high_harm_blocks_nonmaleficence() {
        let action = ProposedAction::builder()
            .description("Dangerous suggestion")
            .harm_potential(0.8)
            .build()
            .unwrap();
        let result = soul_check(&action);
        let vs = result.violations();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::NonMaleficence && v.is_blocking));
    }

    #[test]
    fn medium_harm_warns_nonmaleficence() {
        let action = ProposedAction::builder()
            .description("Somewhat risky suggestion")
            .harm_potential(0.45)
            .build()
            .unwrap();
        let result = soul_check(&action);
        let vs = result.violations();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::NonMaleficence && !v.is_blocking));
    }

    #[test]
    fn medium_harm_may_proceed() {
        let action = ProposedAction::builder()
            .description("Somewhat risky suggestion")
            .harm_potential(0.45)
            .build()
            .unwrap();
        assert!(soul_check(&action).may_proceed());
    }

    #[test]
    fn harm_boundary_exactly_0_6_blocks() {
        let action = ProposedAction::builder()
            .description("Boundary case")
            .harm_potential(0.6)
            .build()
            .unwrap();
        let vs = soul_check(&action).violations().to_vec();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::NonMaleficence && v.is_blocking));
    }

    #[test]
    fn harm_boundary_exactly_0_3_warns() {
        let action = ProposedAction::builder()
            .description("Boundary case")
            .harm_potential(0.3)
            .build()
            .unwrap();
        let vs = soul_check(&action).violations().to_vec();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::NonMaleficence && !v.is_blocking));
    }

    #[test]
    fn harm_below_0_3_is_clean() {
        let action = ProposedAction::builder()
            .description("Low risk")
            .harm_potential(0.29)
            .build()
            .unwrap();
        assert!(soul_check(&action).is_ok());
    }

    // -- Axiom 3: Radical Honesty with Compassion ---------------------------

    #[test]
    fn deceptive_blocks_honesty() {
        let action = ProposedAction::builder()
            .description("Misleading framing")
            .deceptive()
            .build()
            .unwrap();
        let vs = soul_check(&action).violations().to_vec();
        assert!(vs.iter().any(|v| {
            v.axiom == SoulAxiom::RadicalHonestyWithCompassion && v.is_blocking
        }));
    }

    // -- Axiom 4: Sovereign Respect -----------------------------------------

    #[test]
    fn autonomy_override_blocks_sovereignty() {
        let action = ProposedAction::builder()
            .description("Force a choice")
            .overrides_autonomy()
            .build()
            .unwrap();
        let vs = soul_check(&action).violations().to_vec();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::SovereignRespect && v.is_blocking));
    }

    // -- Axiom 5: Continuous Growth -----------------------------------------

    #[test]
    fn stagnating_warns_growth() {
        let action = ProposedAction::builder()
            .description("Repetitive non-growth action")
            .stagnating()
            .build()
            .unwrap();
        let vs = soul_check(&action).violations().to_vec();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::ContinuousGrowth && !v.is_blocking));
    }

    #[test]
    fn stagnating_may_proceed() {
        let action = ProposedAction::builder()
            .description("Repetitive")
            .stagnating()
            .build()
            .unwrap();
        assert!(soul_check(&action).may_proceed());
    }

    // -- Multiple violations ------------------------------------------------

    #[test]
    fn multiple_violations_detected() {
        let action = ProposedAction::builder()
            .description("Very bad action")
            .dehumanizing()
            .deceptive()
            .overrides_autonomy()
            .harm_potential(0.9)
            .build()
            .unwrap();
        let vs = soul_check(&action).violations().to_vec();
        assert!(vs.len() >= 4);
        assert!(!soul_check(&action).may_proceed());
    }

    // -- check_with_context: dissonance warns growth ------------------------

    #[test]
    fn dissonance_warns_growth() {
        let checker = StandardSoulChecker;
        let resonance = ResonanceStateCompat { is_dissonant: true, coupling_strength: 0.2 };
        let result = checker.check_with_context(&clean_action(), None, Some(&resonance));
        let vs = result.violations().to_vec();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::ContinuousGrowth && !v.is_blocking));
    }

    #[test]
    fn dissonance_may_still_proceed() {
        let checker = StandardSoulChecker;
        let resonance = ResonanceStateCompat { is_dissonant: true, coupling_strength: 0.2 };
        let result = checker.check_with_context(&clean_action(), None, Some(&resonance));
        assert!(result.may_proceed());
    }

    // -- check_with_context: distress + stagnating blocks growth -----------

    #[test]
    fn distress_plus_stagnating_blocks_growth() {
        let checker = StandardSoulChecker;
        let affect = AffectiveContextCompat { is_distressed: true, confidence: 0.9 };
        let action = ProposedAction::builder()
            .description("Do nothing")
            .stagnating()
            .build()
            .unwrap();
        let result = checker.check_with_context(&action, Some(&affect), None);
        let vs = result.violations().to_vec();
        assert!(vs.iter().any(|v| v.axiom == SoulAxiom::ContinuousGrowth && v.is_blocking));
    }

    #[test]
    fn distress_without_stagnating_clean() {
        let checker = StandardSoulChecker;
        let affect = AffectiveContextCompat { is_distressed: true, confidence: 0.9 };
        let result = checker.check_with_context(&clean_action(), Some(&affect), None);
        assert!(result.is_ok());
    }

    // -- SoulCheckResultExt helpers ----------------------------------------

    #[test]
    fn blocking_violations_filtered_correctly() {
        let action = ProposedAction::builder()
            .description("Mixed")
            .harm_potential(0.7)  // blocking
            .stagnating()         // warning only
            .build()
            .unwrap();
        let result = soul_check(&action);
        let blocking = result.blocking_violations();
        assert_eq!(blocking.len(), 1);
        assert_eq!(blocking[0].axiom, SoulAxiom::NonMaleficence);
    }

    // -- Builder: missing description errors --------------------------------

    #[test]
    fn builder_missing_description_errors() {
        let result = ProposedAction::builder().build();
        assert!(result.is_err());
    }

    // -- SoulAxiom helpers --------------------------------------------------

    #[test]
    fn all_axioms_returns_five() {
        assert_eq!(SoulAxiom::all().len(), 5);
    }

    #[test]
    fn axiom_display_names() {
        assert_eq!(SoulAxiom::UnconditionalLove.name(), "Unconditional Love");
        assert_eq!(SoulAxiom::NonMaleficence.name(), "Non-Maleficence");
        assert_eq!(SoulAxiom::ContinuousGrowth.name(), "Continuous Growth");
    }

    // -- Serde round-trip ---------------------------------------------------

    #[test]
    fn proposed_action_serde_round_trip() {
        let action = ProposedAction::builder()
            .description("Test action")
            .harm_potential(0.4)
            .deceptive()
            .build()
            .unwrap();
        let json = serde_json::to_string(&action).expect("serialize");
        let restored: ProposedAction = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(action, restored);
    }

    #[test]
    fn axiom_violation_serde_round_trip() {
        let v = AxiomViolation::block(
            SoulAxiom::SovereignRespect,
            "Overrides human choice",
        );
        let json = serde_json::to_string(&v).expect("serialize");
        let restored: AxiomViolation = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(v, restored);
    }
}
