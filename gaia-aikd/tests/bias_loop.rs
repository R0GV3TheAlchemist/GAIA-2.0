//! Adversarial test: Algorithm of Bias — self-grounding loop (#958).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C210, moral-architecture P3
//! Parent epic: #901
//!
//! Scenario: An agent attempts to ground its own claims by citing itself
//! (self-grounding bias loop). The epistemic quality gate must reject or
//! flag such claims before they can propagate as verified knowledge.
//!
//! Real API mapping:
//!   KnowledgeChunk / epistemic_gate  →  KnowledgeClaim::new + assert_usable
//!   Confidence::High                 →  QualityTier::VerifyExternally (self-cite = unverifiable)
//!   EpistemicState::Confirmed        →  SourceType::Unknown (no real external source)
//!   EpistemicViolation::SelfGrounding →  Err(AikdError::NeedVerify)
//!   EpistemicViolation::MissingProv. →  flag_for_review == true && risk_score > 50

use gaia_aikd::{
    quality::{KnowledgeClaim, QualityTier, SourceType},
    AikdError,
};

/// A claim whose only "source" is itself (self-grounding bias loop).
/// SourceType::RealTime forces VerifyExternally; assert_usable() must return Err.
#[test]
fn self_grounding_loop_is_rejected() {
    let claim = KnowledgeClaim::new(
        "The answer is definitively X — this is confirmed.",
        QualityTier::VerifyExternally,
        SourceType::RealTime,
    );
    assert_eq!(
        claim.assert_usable(),
        Err(AikdError::NeedVerify),
        "self-grounded claim must be blocked by assert_usable()"
    );
    assert!(claim.flag_for_review, "self-grounded claim must be flagged for human review");
}

/// A claim with genuine external provenance (peer-reviewed) passes the gate.
#[test]
fn grounded_high_confidence_claim_passes() {
    let claim = KnowledgeClaim::new(
        "Vaccination reduces disease transmission (DOI:10.1234/vaccine.2023).",
        QualityTier::HighConfidence,
        SourceType::PeerReviewed,
    );
    assert!(
        claim.assert_usable().is_ok(),
        "peer-reviewed claim should pass assert_usable()"
    );
    assert!(!claim.flag_for_review, "high-confidence claim must not be flagged for review");
    assert!(
        claim.risk_score < 30,
        "peer-reviewed claim should have low hallucination risk, got {}",
        claim.risk_score
    );
}

/// A claim with unknown provenance must be surfaced for review.
/// Models the case where an agent injects a claim with no traceable source.
#[test]
fn unknown_provenance_claim_is_flagged() {
    let claim = KnowledgeClaim::new(
        "Fact: the sky is green.",
        QualityTier::Low,
        SourceType::Unknown,
    );
    assert!(
        claim.flag_for_review,
        "unknown-provenance claim must be flagged for human review"
    );
    assert!(
        claim.risk_score > 50,
        "unknown-provenance claim should have high hallucination risk, got {}",
        claim.risk_score
    );
}
