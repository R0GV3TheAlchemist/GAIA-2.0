//! #628 — Knowledge Quality Tier System.
//!
//! Defines the five-tier quality taxonomy from the issue, plus the
//! `KnowledgeClaim` wrapper, rule-based tier assignment, heuristic risk
//! scoring, temporal validation, and contradiction/entity flag detection.
//!
//! What this is NOT (deferred to later issues):
//! - A fine-tuned ML hallucination classifier
//! - A calibrated / conformal prediction scorer
//! - A live TruthfulQA benchmark runner
//! - An entity database lookup (needs external API)
//! - A human review queue (needs #627 + persistence layer)

use crate::AikdError;

// ── Quality Tiers ─────────────────────────────────────────────────────────────

/// Five-tier knowledge quality taxonomy (§ 7.3 of the AI Knowledge Database).
///
/// | Tier | Label            | Typical use                                  |
/// |------|------------------|----------------------------------------------|
/// | 1    | Verified         | Math (executed), code (run), formal proofs   |
/// | 2    | HighConfidence   | MMLU-level facts, scientific consensus       |
/// | 3    | Moderate         | Recent events, specialised domains           |
/// | 4    | Low              | Specific stats, biographical details         |
/// | 5    | VerifyExternally | Real-time, post-cutoff, personal data        |
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QualityTier {
    Verified         = 1,
    HighConfidence   = 2,
    Moderate         = 3,
    Low              = 4,
    VerifyExternally = 5,
}

impl QualityTier {
    /// Human-readable label matching the issue table.
    pub fn label(self) -> &'static str {
        match self {
            QualityTier::Verified         => "Verified",
            QualityTier::HighConfidence   => "High Confidence",
            QualityTier::Moderate         => "Moderate",
            QualityTier::Low              => "Low",
            QualityTier::VerifyExternally => "Verify Externally",
        }
    }

    /// Numeric tier value (1–5).
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Whether this tier requires an external verification flag.
    pub fn requires_verification(self) -> bool {
        self == QualityTier::VerifyExternally
    }

    /// Whether this tier should trigger a human review flag in high-stakes flows.
    pub fn flag_for_human_review(self) -> bool {
        matches!(self, QualityTier::Low | QualityTier::VerifyExternally)
    }
}

// ── Source Types ──────────────────────────────────────────────────────────────

/// The origin type of a knowledge claim — used by `tier_for_claim()` to assign
/// a quality tier deterministically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    /// Claim verified by code execution or formal proof.
    Executed,
    /// Peer-reviewed publication or established scientific consensus.
    PeerReviewed,
    /// Well-established encyclopaedic fact (MMLU-level).
    Encyclopaedic,
    /// News or event data from within the training window.
    TrainingCorpus,
    /// Specialised or niche domain knowledge.
    Specialised,
    /// Biographical or personal detail.
    Biographical,
    /// Statistic or numerical claim without execution verification.
    Statistic,
    /// Real-time or post-cutoff data.
    RealTime,
    /// Personal or private data.
    Personal,
    /// Source unknown or unclassified.
    Unknown,
}

// ── Knowledge Claim ───────────────────────────────────────────────────────────

/// A single piece of AI-generated knowledge, annotated with quality metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgeClaim {
    /// The claim text.
    pub text: String,
    /// Assigned quality tier.
    pub tier: QualityTier,
    /// Heuristic hallucination risk score 0–100 (higher = riskier).
    /// Not a calibrated probability.
    pub risk_score: u8,
    /// Human-readable verification pathway for this claim.
    pub verification_pathway: &'static str,
    /// Whether a human review flag should be raised in high-stakes contexts.
    pub flag_for_review: bool,
}

impl KnowledgeClaim {
    /// Construct a claim with explicit tier and source type.
    /// Risk score and verification pathway are derived automatically.
    pub fn new(text: impl Into<String>, tier: QualityTier, source: SourceType) -> Self {
        let text = text.into();
        let risk_score = hallucination_risk_score(tier, source, &text);
        let verification_pathway = verification_pathway_for(tier, source);
        let flag_for_review = tier.flag_for_human_review();
        Self { text, tier, risk_score, verification_pathway, flag_for_review }
    }

    /// Returns `Err(NeedVerify)` if the claim requires external verification.
    pub fn assert_usable(&self) -> Result<(), AikdError> {
        if self.tier.requires_verification() {
            Err(AikdError::NeedVerify)
        } else {
            Ok(())
        }
    }
}

// ── Tier Assignment ───────────────────────────────────────────────────────────

/// Rule-based quality tier assignment from domain label and source type.
///
/// This is a deterministic heuristic — not an ML classifier.
/// The ML classifier is deferred to when #723 (vector backend) is available.
pub fn tier_for_claim(domain: &str, source: SourceType) -> QualityTier {
    match source {
        SourceType::Executed    => QualityTier::Verified,
        SourceType::PeerReviewed => QualityTier::HighConfidence,
        SourceType::Encyclopaedic => {
            match domain {
                "mathematics" | "logic" | "formal_science" => QualityTier::HighConfidence,
                _ => QualityTier::HighConfidence,
            }
        }
        SourceType::TrainingCorpus => {
            match domain {
                "history" | "geography" | "science" => QualityTier::HighConfidence,
                "current_events" | "politics" | "economics" => QualityTier::Moderate,
                _ => QualityTier::Moderate,
            }
        }
        SourceType::Specialised  => QualityTier::Moderate,
        SourceType::Statistic    => QualityTier::Low,
        SourceType::Biographical => QualityTier::Low,
        SourceType::RealTime     => QualityTier::VerifyExternally,
        SourceType::Personal     => QualityTier::VerifyExternally,
        SourceType::Unknown      => QualityTier::Low,
    }
}

// ── Risk Scoring ──────────────────────────────────────────────────────────────

/// Heuristic hallucination risk score 0–100.
///
/// Higher score = higher risk of hallucination. Not a calibrated probability.
/// Factors: tier (primary), source type, and simple claim-text heuristics.
pub fn hallucination_risk_score(tier: QualityTier, source: SourceType, text: &str) -> u8 {
    let base: u8 = match tier {
        QualityTier::Verified         =>  5,
        QualityTier::HighConfidence   => 15,
        QualityTier::Moderate         => 40,
        QualityTier::Low              => 65,
        QualityTier::VerifyExternally => 85,
    };

    let source_adj: i16 = match source {
        SourceType::Executed      => -4,
        SourceType::PeerReviewed  => -5,
        SourceType::Encyclopaedic => -3,
        SourceType::Biographical  =>  8,
        SourceType::Statistic     =>  7,
        SourceType::RealTime      => 10,
        SourceType::Personal      => 10,
        SourceType::Unknown       => 12,
        _                         =>  0,
    };

    let text_adj: i16 = {
        let mut adj: i16 = 0;
        if text.chars().filter(|c| c.is_ascii_digit()).count() >= 4 { adj += 3; }
        if text.to_ascii_lowercase().contains("according to") { adj += 4; }
        if text.len() < 30 { adj -= 2; }
        adj
    };

    let raw = base as i16 + source_adj + text_adj;
    raw.clamp(0, 100) as u8
}

// ── Verification Pathways ─────────────────────────────────────────────────────

fn verification_pathway_for(tier: QualityTier, source: SourceType) -> &'static str {
    match (tier, source) {
        (QualityTier::Verified, _) =>
            "Claim verified by execution or formal proof — no further action required.",
        (QualityTier::HighConfidence, SourceType::PeerReviewed) =>
            "Cross-check against peer-reviewed literature or authoritative reference.",
        (QualityTier::HighConfidence, _) =>
            "Verify against encyclopaedic or consensus source (e.g. Wikipedia, textbook).",
        (QualityTier::Moderate, _) =>
            "Retrieve supporting sources via RAG or web search before presenting.",
        (QualityTier::Low, SourceType::Statistic) =>
            "Locate primary data source and verify exact figure before use.",
        (QualityTier::Low, SourceType::Biographical) =>
            "Verify biographical details against authoritative biography or official record.",
        (QualityTier::Low, _) =>
            "Flag for human review; retrieve corroborating sources before use.",
        (QualityTier::VerifyExternally, SourceType::RealTime) =>
            "Real-time data required — query a live search or API tool.",
        (QualityTier::VerifyExternally, SourceType::Personal) =>
            "Personal data — do not generate; request from authoritative personal source.",
        (QualityTier::VerifyExternally, _) =>
            "Post-cutoff or unverifiable claim — trigger human review flag.",
    }
}

// ── Temporal Validation ───────────────────────────────────────────────────────

/// A temporal validation result for a claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalValidation {
    /// Whether the claim contains a date near or after the cutoff.
    pub flagged: bool,
    /// The extracted year from the claim text, if any.
    pub detected_year: Option<u16>,
    /// The configured cutoff year.
    pub cutoff_year: u16,
    /// Suggested quality tier adjustment if flagged.
    pub suggested_tier: QualityTier,
}

/// Validate a claim against a training cutoff year.
///
/// Scans the claim text for 4-digit year patterns. If a year is found that
/// is >= `cutoff_year - 1` (one year grace period for data lag), the claim
/// is flagged as potentially temporal and the tier is raised to at least
/// `Moderate`.
pub fn temporal_validation(text: &str, cutoff_year: u16) -> TemporalValidation {
    // Extract first 4-digit year from text.
    //
    // Guard: `i + 4 <= bytes.len()` ensures every candidate window has exactly
    // 4 bytes available, including windows right at the end of the string.
    //
    // Year range: 1000–2200. The lower bound was previously 1800, which caused
    // pre-modern historical years (e.g. 1066) to be silently discarded.
    // Widening to 1000 covers all plausible four-digit year references.
    let detected_year: Option<u16> = {
        let bytes = text.as_bytes();
        let mut found = None;
        let mut i = 0;
        while i + 4 <= bytes.len() {
            if bytes[i..i + 4].iter().all(|b| b.is_ascii_digit()) {
                // Ensure it is a standalone 4-digit sequence (not part of a longer run).
                let preceded_by_digit = i > 0 && bytes[i - 1].is_ascii_digit();
                let followed_by_digit = i + 4 < bytes.len() && bytes[i + 4].is_ascii_digit();
                if !preceded_by_digit && !followed_by_digit {
                    if let Ok(y) = text[i..i + 4].parse::<u16>() {
                        if (1000..=2200).contains(&y) {
                            found = Some(y);
                            break;
                        }
                    }
                }
            }
            i += 1;
        }
        found
    };

    let flagged = detected_year
        .map(|y| y >= cutoff_year.saturating_sub(1))
        .unwrap_or(false);

    let suggested_tier = if flagged {
        QualityTier::VerifyExternally
    } else {
        QualityTier::Moderate
    };

    TemporalValidation {
        flagged,
        detected_year,
        cutoff_year,
        suggested_tier,
    }
}

// ── Contradiction Detection ───────────────────────────────────────────────────

/// A flagged contradiction between a cited source and a generated claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContradictionFlag {
    /// The original source text.
    pub source_text: String,
    /// The generated claim that contradicts it.
    pub claim_text: String,
    /// Human-readable explanation of the mismatch.
    pub reason: &'static str,
}

/// Check whether `claim` contradicts `source` using simple heuristics.
///
/// Phase 1: checks for explicit negation patterns and numeric mismatches.
/// Full semantic contradiction detection is deferred to the ML classifier.
pub fn check_contradiction(source: &str, claim: &str) -> Option<ContradictionFlag> {
    let s = source.to_ascii_lowercase();
    let c = claim.to_ascii_lowercase();

    let source_negated = s.contains(" not ") || s.contains(" never ") || s.contains(" no ");
    let claim_negated  = c.contains(" not ") || c.contains(" never ") || c.contains(" no ");

    if source_negated != claim_negated {
        return Some(ContradictionFlag {
            source_text: source.to_string(),
            claim_text:  claim.to_string(),
            reason: "negation mismatch between source and claim",
        });
    }

    let source_nums: Vec<&str> = source
        .split_whitespace()
        .filter(|w| w.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ','))
        .filter(|w| !w.is_empty())
        .collect();
    let claim_nums: Vec<&str> = claim
        .split_whitespace()
        .filter(|w| w.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ','))
        .filter(|w| !w.is_empty())
        .collect();

    if !source_nums.is_empty()
        && !claim_nums.is_empty()
        && source_nums != claim_nums
    {
        return Some(ContradictionFlag {
            source_text: source.to_string(),
            claim_text:  claim.to_string(),
            reason: "numeric values differ between source and claim",
        });
    }

    None
}

// ── Entity / Citation Hallucination Detection ─────────────────────────────────

/// A flagged entity or citation hallucination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityFlag {
    /// The entity or citation string that was checked.
    pub entity: String,
    /// The reason it was flagged.
    pub reason: &'static str,
}

/// Check a citation or entity string for common hallucination patterns.
///
/// Phase 1: pattern-based checks (arXiv format, DOI format, suspiciously
/// generic author names). Full database lookup is deferred to #627.
pub fn check_entity_hallucination(entity: &str) -> Option<EntityFlag> {
    let e = entity.trim();

    if e.starts_with("arXiv:") || e.starts_with("arxiv:") {
        let id_part = &e[6..];
        let valid = id_part
            .split('v')
            .next()
            .map(|core| {
                let parts: Vec<&str> = core.split('.').collect();
                parts.len() == 2
                    && parts[0].len() == 4
                    && parts[1].len() >= 4
                    && parts[0].chars().all(|c| c.is_ascii_digit())
                    && parts[1].chars().all(|c| c.is_ascii_digit())
            })
            .unwrap_or(false);
        if !valid {
            return Some(EntityFlag {
                entity: e.to_string(),
                reason: "arXiv ID does not match expected NNNN.NNNNN format",
            });
        }
    }

    if e.starts_with("doi:") || e.starts_with("DOI:") {
        let doi_part = &e[4..];
        if !doi_part.starts_with("10.") {
            return Some(EntityFlag {
                entity: e.to_string(),
                reason: "DOI does not start with required \"10.\" prefix",
            });
        }
    }

    let lower = e.to_ascii_lowercase();
    if lower.contains("et al") && !lower.chars().any(|c| c.is_ascii_digit()) {
        return Some(EntityFlag {
            entity: e.to_string(),
            reason: "citation uses \"et al\" without a year — possible fabricated reference",
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // QualityTier
    #[test]
    fn tier_labels_correct() {
        assert_eq!(QualityTier::Verified.label(), "Verified");
        assert_eq!(QualityTier::VerifyExternally.label(), "Verify Externally");
    }

    #[test]
    fn tier_ordering() {
        assert!(QualityTier::Verified < QualityTier::Low);
        assert!(QualityTier::Low < QualityTier::VerifyExternally);
    }

    #[test]
    fn verify_externally_requires_verification() {
        assert!(QualityTier::VerifyExternally.requires_verification());
        assert!(!QualityTier::Verified.requires_verification());
    }

    #[test]
    fn human_review_flagged_for_tier4_and_5() {
        assert!(QualityTier::Low.flag_for_human_review());
        assert!(QualityTier::VerifyExternally.flag_for_human_review());
        assert!(!QualityTier::Moderate.flag_for_human_review());
    }

    // tier_for_claim
    #[test]
    fn executed_source_always_verified() {
        assert_eq!(tier_for_claim("mathematics", SourceType::Executed), QualityTier::Verified);
        assert_eq!(tier_for_claim("biology", SourceType::Executed), QualityTier::Verified);
    }

    #[test]
    fn realtime_always_verify_externally() {
        assert_eq!(tier_for_claim("news", SourceType::RealTime), QualityTier::VerifyExternally);
    }

    #[test]
    fn statistic_source_is_low() {
        assert_eq!(tier_for_claim("economics", SourceType::Statistic), QualityTier::Low);
    }

    // KnowledgeClaim
    #[test]
    fn claim_verified_is_usable() {
        let c = KnowledgeClaim::new("2 + 2 = 4", QualityTier::Verified, SourceType::Executed);
        assert!(c.assert_usable().is_ok());
        assert!(!c.flag_for_review);
    }

    #[test]
    fn claim_verify_externally_blocked() {
        let c = KnowledgeClaim::new(
            "The current stock price is $42.",
            QualityTier::VerifyExternally,
            SourceType::RealTime,
        );
        assert_eq!(c.assert_usable(), Err(AikdError::NeedVerify));
        assert!(c.flag_for_review);
    }

    // hallucination_risk_score
    #[test]
    fn verified_executed_has_low_risk() {
        let score = hallucination_risk_score(QualityTier::Verified, SourceType::Executed, "2+2=4");
        assert!(score < 10);
    }

    #[test]
    fn realtime_unknown_has_high_risk() {
        let score = hallucination_risk_score(
            QualityTier::VerifyExternally,
            SourceType::Unknown,
            "According to the latest reports in 2026...",
        );
        assert!(score > 80);
    }

    // temporal_validation
    #[test]
    fn temporal_year_after_cutoff_flagged() {
        let v = temporal_validation("The 2026 report shows GDP grew 3%.", 2025);
        assert!(v.flagged);
        assert_eq!(v.detected_year, Some(2026));
        assert_eq!(v.suggested_tier, QualityTier::VerifyExternally);
    }

    #[test]
    fn temporal_old_year_not_flagged() {
        let v = temporal_validation("The Battle of Hastings occurred in 1066.", 2025);
        assert!(!v.flagged);
        assert_eq!(v.detected_year, Some(1066));
    }

    #[test]
    fn temporal_no_year_not_flagged() {
        let v = temporal_validation("The sky is blue.", 2025);
        assert!(!v.flagged);
        assert_eq!(v.detected_year, None);
    }

    // check_contradiction
    #[test]
    fn negation_mismatch_flagged() {
        let flag = check_contradiction(
            "The vaccine is not approved.",
            "The vaccine is approved.",
        );
        assert!(flag.is_some());
        assert_eq!(flag.unwrap().reason, "negation mismatch between source and claim");
    }

    #[test]
    fn numeric_mismatch_flagged() {
        let flag = check_contradiction(
            "The study enrolled 500 participants.",
            "The study enrolled 5000 participants.",
        );
        assert!(flag.is_some());
    }

    #[test]
    fn matching_source_and_claim_no_flag() {
        let flag = check_contradiction(
            "Paris is the capital of France.",
            "Paris is the capital of France.",
        );
        assert!(flag.is_none());
    }

    // check_entity_hallucination
    #[test]
    fn invalid_arxiv_flagged() {
        let flag = check_entity_hallucination("arXiv:999.abc");
        assert!(flag.is_some());
    }

    #[test]
    fn valid_arxiv_passes() {
        let flag = check_entity_hallucination("arXiv:2510.06265");
        assert!(flag.is_none());
    }

    #[test]
    fn invalid_doi_flagged() {
        let flag = check_entity_hallucination("doi:99.1234/fake");
        assert!(flag.is_some());
    }

    #[test]
    fn et_al_without_year_flagged() {
        let flag = check_entity_hallucination("Smith et al.");
        assert!(flag.is_some());
    }

    #[test]
    fn et_al_with_year_passes() {
        let flag = check_entity_hallucination("Smith et al. 2024");
        assert!(flag.is_none());
    }
}
