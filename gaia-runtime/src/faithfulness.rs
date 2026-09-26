//! Faithfulness scorer stub (#932). Not an LLM evaluator.

/// Score in `[0.0, 1.0]`. Stub: lexical overlap of response tokens against chunk text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaithfulnessScore(pub f32);

pub fn score_faithfulness(response: &str, chunk_texts: &[&str]) -> FaithfulnessScore {
    if response.trim().is_empty() || chunk_texts.is_empty() {
        return FaithfulnessScore(0.0);
    }
    let hay = chunk_texts.join(" ").to_ascii_lowercase();
    let tokens: Vec<&str> = response
        .split_whitespace()
        .filter(|t| t.len() > 2)
        .collect();
    if tokens.is_empty() {
        return FaithfulnessScore(0.0);
    }
    let hits = tokens
        .iter()
        .filter(|t| hay.contains(&t.to_ascii_lowercase()))
        .count();
    let raw = hits as f32 / tokens.len() as f32;
    FaithfulnessScore(raw.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_is_unit_interval() {
        let s = score_faithfulness("the river is blue", &["the river is blue today"]);
        assert!((0.0..=1.0).contains(&s.0));
        assert!(s.0 > 0.5);
    }

    #[test]
    fn hallucination_fixture_scores_below_half() {
        let chunks = ["The treaty was signed in 1992 in Rio."];
        let response = "The treaty was signed in 1848 on Mars and banned water.";
        let s = score_faithfulness(response, &chunks);
        assert!(
            s.0 < 0.5,
            "contradictory fixture must score below 0.5, got {}",
            s.0
        );
    }
}
