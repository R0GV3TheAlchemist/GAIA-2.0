//! Integration tests for #628 — Hallucination Detection & Knowledge Quality Tier System.
//!
//! These tests exercise the public API surface of `gaia_aikd::quality` from
//! outside the crate, mirroring how downstream crates and the unified API
//! (#627) will consume the quality layer.

use gaia_aikd::quality::{
    check_contradiction, check_entity_hallucination, hallucination_risk_score,
    tier_for_claim, temporal_validation, KnowledgeClaim, QualityTier, SourceType,
};
use gaia_aikd::AikdError;

// ── QualityTier public surface ────────────────────────────────────────────────

#[test]
fn all_tier_labels_are_non_empty() {
    let tiers = [
        QualityTier::Verified,
        QualityTier::HighConfidence,
        QualityTier::Moderate,
        QualityTier::Low,
        QualityTier::VerifyExternally,
    ];
    for t in tiers {
        assert!(!t.label().is_empty(), "tier {:?} has empty label", t);
    }
}

#[test]
fn tier_numeric_values_are_ordered_1_to_5() {
    assert_eq!(QualityTier::Verified.as_u8(), 1);
    assert_eq!(QualityTier::HighConfidence.as_u8(), 2);
    assert_eq!(QualityTier::Moderate.as_u8(), 3);
    assert_eq!(QualityTier::Low.as_u8(), 4);
    assert_eq!(QualityTier::VerifyExternally.as_u8(), 5);
}

#[test]
fn only_tier5_requires_external_verification() {
    assert!(QualityTier::VerifyExternally.requires_verification());
    assert!(!QualityTier::Low.requires_verification());
    assert!(!QualityTier::Verified.requires_verification());
}

#[test]
fn tier4_and_tier5_flag_human_review() {
    assert!(QualityTier::Low.flag_for_human_review());
    assert!(QualityTier::VerifyExternally.flag_for_human_review());
    assert!(!QualityTier::HighConfidence.flag_for_human_review());
    assert!(!QualityTier::Verified.flag_for_human_review());
}

// ── tier_for_claim ────────────────────────────────────────────────────────────

#[test]
fn executed_code_always_tier1() {
    assert_eq!(tier_for_claim("code", SourceType::Executed), QualityTier::Verified);
    assert_eq!(tier_for_claim("mathematics", SourceType::Executed), QualityTier::Verified);
}

#[test]
fn peer_reviewed_is_tier2() {
    assert_eq!(tier_for_claim("biology", SourceType::PeerReviewed), QualityTier::HighConfidence);
}

#[test]
fn biographical_is_tier4() {
    assert_eq!(tier_for_claim("history", SourceType::Biographical), QualityTier::Low);
}

#[test]
fn realtime_and_personal_are_tier5() {
    assert_eq!(tier_for_claim("news", SourceType::RealTime), QualityTier::VerifyExternally);
    assert_eq!(tier_for_claim("personal", SourceType::Personal), QualityTier::VerifyExternally);
}

// ── KnowledgeClaim ────────────────────────────────────────────────────────────

#[test]
fn verified_claim_is_usable_and_not_flagged() {
    let c = KnowledgeClaim::new(
        "The integral of x^2 dx is x^3/3 + C.",
        QualityTier::Verified,
        SourceType::Executed,
    );
    assert!(c.assert_usable().is_ok());
    assert!(!c.flag_for_review);
    assert!(c.risk_score < 15);
    assert!(!c.verification_pathway.is_empty());
}

#[test]
fn tier5_claim_blocked_and_flagged() {
    let c = KnowledgeClaim::new(
        "The prime minister announced new policy today.",
        QualityTier::VerifyExternally,
        SourceType::RealTime,
    );
    assert_eq!(c.assert_usable(), Err(AikdError::NeedVerify));
    assert!(c.flag_for_review);
    assert!(c.risk_score > 80);
}

#[test]
fn tier4_claim_flagged_for_review_but_usable() {
    let c = KnowledgeClaim::new(
        "He was born on March 3rd, 1978 in Cincinnati.",
        QualityTier::Low,
        SourceType::Biographical,
    );
    // T4 does not require external verification — assert_usable passes
    assert!(c.assert_usable().is_ok());
    // But human review flag is raised
    assert!(c.flag_for_review);
}

// ── hallucination_risk_score ──────────────────────────────────────────────────

#[test]
fn risk_score_in_valid_range() {
    for tier in [
        QualityTier::Verified,
        QualityTier::HighConfidence,
        QualityTier::Moderate,
        QualityTier::Low,
        QualityTier::VerifyExternally,
    ] {
        for source in [
            SourceType::Executed,
            SourceType::Unknown,
            SourceType::RealTime,
        ] {
            let score = hallucination_risk_score(tier, source, "some claim text");
            assert!(
                score <= 100,
                "score out of range for {:?}/{:?}: {}", tier, source, score
            );
        }
    }
}

#[test]
fn executed_tier1_has_lowest_risk() {
    let low  = hallucination_risk_score(QualityTier::Verified, SourceType::Executed, "2+2=4");
    let high = hallucination_risk_score(QualityTier::VerifyExternally, SourceType::Unknown, "According to 2026 figures...");
    assert!(low < high);
}

// ── temporal_validation ───────────────────────────────────────────────────────

#[test]
fn near_cutoff_year_flagged() {
    let v = temporal_validation("The 2025 census data shows population growth.", 2025);
    assert!(v.flagged);
    assert_eq!(v.suggested_tier, QualityTier::VerifyExternally);
}

#[test]
fn historical_year_not_flagged() {
    let v = temporal_validation("The French Revolution began in 1789.", 2025);
    assert!(!v.flagged);
}

#[test]
fn no_year_in_claim_not_flagged() {
    let v = temporal_validation("Water boils at 100 degrees Celsius at sea level.", 2025);
    // "100" is not a valid year in range
    assert!(!v.flagged);
}

// ── check_contradiction ───────────────────────────────────────────────────────

#[test]
fn matching_claims_produce_no_flag() {
    assert!(check_contradiction(
        "The speed of light is approximately 299,792 km/s.",
        "The speed of light is approximately 299,792 km/s.",
    ).is_none());
}

#[test]
fn negation_contradiction_detected() {
    let f = check_contradiction(
        "The experiment was not replicated.",
        "The experiment was replicated.",
    );
    assert!(f.is_some());
}

#[test]
fn numeric_contradiction_detected() {
    let f = check_contradiction(
        "There were 12 participants in the study.",
        "There were 120 participants in the study.",
    );
    assert!(f.is_some());
}

// ── check_entity_hallucination ────────────────────────────────────────────────

#[test]
fn well_formed_arxiv_id_passes() {
    assert!(check_entity_hallucination("arXiv:2510.06265").is_none());
    assert!(check_entity_hallucination("arXiv:2310.01234v2").is_none());
}

#[test]
fn malformed_arxiv_id_flagged() {
    assert!(check_entity_hallucination("arXiv:999.abc").is_some());
    assert!(check_entity_hallucination("arXiv:25.1234").is_some());
}

#[test]
fn valid_doi_passes() {
    assert!(check_entity_hallucination("doi:10.1038/nature12345").is_none());
}

#[test]
fn invalid_doi_prefix_flagged() {
    assert!(check_entity_hallucination("doi:99.1234/fake").is_some());
}

#[test]
fn et_al_without_year_flagged() {
    assert!(check_entity_hallucination("Johnson et al.").is_some());
}

#[test]
fn et_al_with_year_passes() {
    assert!(check_entity_hallucination("Johnson et al. 2023").is_none());
}
