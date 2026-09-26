//! First cuts for RAG Tier 2 / 3 leftovers (#1013–#1017).
//! Not a cross-encoder, not HyDE, not calibrated confidence.

use std::collections::HashSet;

use crate::quality::{check_contradiction, ContradictionFlag};
use crate::rank::RankedHit;

fn tokens(text: &str) -> HashSet<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| t.len() > 2)
        .map(|t| t.to_ascii_lowercase())
        .collect()
}

fn jaccard(a: &str, b: &str) -> f32 {
    let ta = tokens(a);
    let tb = tokens(b);
    if ta.is_empty() && tb.is_empty() {
        return 0.0;
    }
    let inter = ta.intersection(&tb).count() as f32;
    let union = ta.union(&tb).count() as f32;
    if union == 0.0 {
        0.0
    } else {
        inter / union
    }
}

/// FM-3: pick `k` hits maximizing λ·relevance − (1−λ)·max Jaccard to selected.
pub fn mmr_select(hits: &[RankedHit], k: usize, lambda: f32) -> Vec<RankedHit> {
    if k == 0 || hits.is_empty() {
        return Vec::new();
    }
    let lambda = lambda.clamp(0.0, 1.0);
    let mut remaining: Vec<RankedHit> = hits.to_vec();
    let mut selected: Vec<RankedHit> = Vec::new();
    while selected.len() < k && !remaining.is_empty() {
        let mut best_i = 0;
        let mut best = f32::NEG_INFINITY;
        for (i, cand) in remaining.iter().enumerate() {
            let sim = selected
                .iter()
                .map(|s: &RankedHit| jaccard(&cand.text, &s.text))
                .fold(0.0_f32, f32::max);
            let mmr = lambda * cand.score - (1.0 - lambda) * sim;
            if mmr > best {
                best = mmr;
                best_i = i;
            }
        }
        selected.push(remaining.remove(best_i));
    }
    selected
}

/// FM-6: tiny synonym table. Not HyDE.
pub fn expand_query(query: &str) -> Vec<String> {
    let q = query.trim().to_ascii_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    let mut out = vec![query.trim().to_string()];
    const PAIRS: &[(&str, &str)] = &[
        ("climate", "earth twin climate observation"),
        ("lisinopril", "lisinopril milligrams prescribed"),
        ("treaty", "treaty signed 1992 rio"),
    ];
    for (key, extra) in PAIRS {
        if q.split(|c: char| !c.is_ascii_alphanumeric()).any(|t| t == *key) {
            out.push((*extra).to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

/// FM-7: pairwise contradiction flags among hit texts.
pub fn flag_hit_conflicts(hits: &[RankedHit]) -> Vec<ContradictionFlag> {
    let mut flags = Vec::new();
    for i in 0..hits.len() {
        for j in (i + 1)..hits.len() {
            if let Some(flag) = check_contradiction(&hits[i].text, &hits[j].text) {
                flags.push(flag);
            }
        }
    }
    flags
}

/// FM-8: keep highest-ranked hits whose texts fit `max_chars`.
pub fn budget_hits(hits: &[RankedHit], max_chars: usize) -> Vec<RankedHit> {
    let mut used: usize = 0;
    let mut out = Vec::new();
    for hit in hits {
        let len = hit.text.len();
        if out.is_empty() && len > max_chars {
            continue;
        }
        if used.saturating_add(len) > max_chars {
            break;
        }
        used += len;
        out.push(hit.clone());
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetrievalConfidence {
    High,
    Medium,
    Low,
    Insufficient,
}

impl RetrievalConfidence {
    pub fn label(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Insufficient => "insufficient",
        }
    }

    pub fn abstain(self) -> bool {
        matches!(self, Self::Insufficient)
    }
}

/// FM-11: map a fused score to a coarse tier. Not calibrated.
pub fn retrieval_confidence(score: f32) -> RetrievalConfidence {
    if score >= 0.35 {
        RetrievalConfidence::High
    } else if score >= 0.15 {
        RetrievalConfidence::Medium
    } else if score >= 0.02 {
        RetrievalConfidence::Low
    } else {
        RetrievalConfidence::Insufficient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(text: &str, score: f32) -> RankedHit {
        RankedHit {
            hex: format!("{:064}", text.len()),
            text: text.into(),
            score,
        }
    }

    #[test]
    fn mmr_drops_near_duplicate() {
        let hits = vec![
            hit("the earth twin observes climate", 0.9),
            hit("the earth twin observes climate patterns", 0.88),
            hit("purple piano recipes", 0.2),
        ];
        let picked = mmr_select(&hits, 2, 0.5);
        assert_eq!(picked.len(), 2);
        assert_eq!(picked[0].text, hits[0].text);
        assert_eq!(picked[1].text, hits[2].text);
    }

    #[test]
    fn expand_climate_adds_phrase() {
        let q = expand_query("climate models");
        assert!(q.iter().any(|s| s.contains("earth twin")));
        assert!(q.iter().any(|s| s.contains("climate models")));
    }

    #[test]
    fn conflict_on_numeric_mismatch() {
        let hits = vec![
            hit("The treaty was signed in 1992.", 0.8),
            hit("The treaty was signed in 1848.", 0.7),
        ];
        let flags = flag_hit_conflicts(&hits);
        assert_eq!(flags.len(), 1);
    }

    #[test]
    fn budget_stops_before_overflow() {
        let hits = vec![
            hit("short", 0.9),
            hit("a much longer climate sentence", 0.8),
        ];
        let kept = budget_hits(&hits, 10);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].text, "short");
    }

    #[test]
    fn confidence_tiers() {
        assert_eq!(retrieval_confidence(0.4), RetrievalConfidence::High);
        assert_eq!(retrieval_confidence(0.2), RetrievalConfidence::Medium);
        assert_eq!(retrieval_confidence(0.05), RetrievalConfidence::Low);
        assert!(retrieval_confidence(0.0).abstain());
    }
}
