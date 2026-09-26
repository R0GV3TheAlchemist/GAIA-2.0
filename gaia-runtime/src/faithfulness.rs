//! Lexical faithfulness metrics (#932 / #1007). Not NLI. Not an LLM judge.
//!
//! | Metric | Meaning |
//! | --- | --- |
//! | `precision` | Share of response tokens that appear in the sources. |
//! | `recall` | Share of source tokens that appear in the response. |
//! | `jaccard` | Token-set intersection over union. |
//! | `composite` | Mean of the three. Used as the gate value. |

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaithfulnessScore {
    pub precision: f32,
    pub recall: f32,
    pub jaccard: f32,
    pub composite: f32,
}

impl FaithfulnessScore {
    pub fn value(self) -> f32 {
        self.composite
    }
}

fn tokens(text: &str) -> HashSet<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| t.len() > 2)
        .map(|t| t.to_ascii_lowercase())
        .collect()
}

fn ratio(num: usize, den: usize) -> f32 {
    if den == 0 {
        0.0
    } else {
        num as f32 / den as f32
    }
}

pub fn score_faithfulness(response: &str, chunk_texts: &[&str]) -> FaithfulnessScore {
    if response.trim().is_empty() || chunk_texts.is_empty() {
        return FaithfulnessScore {
            precision: 0.0,
            recall: 0.0,
            jaccard: 0.0,
            composite: 0.0,
        };
    }
    let resp = tokens(response);
    let src = tokens(&chunk_texts.join(" "));
    if resp.is_empty() && src.is_empty() {
        return FaithfulnessScore {
            precision: 0.0,
            recall: 0.0,
            jaccard: 0.0,
            composite: 0.0,
        };
    }
    let inter = resp.intersection(&src).count();
    let union = resp.union(&src).count();
    let precision = ratio(inter, resp.len());
    let recall = ratio(inter, src.len());
    let jaccard = ratio(inter, union);
    let composite = (precision + recall + jaccard) / 3.0;
    FaithfulnessScore {
        precision,
        recall,
        jaccard,
        composite: composite.clamp(0.0, 1.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copied_answer_scores_high() {
        let s = score_faithfulness("the river is blue", &["the river is blue today"]);
        assert!((0.0..=1.0).contains(&s.composite));
        assert!(s.composite > 0.5);
        assert!(s.precision > 0.9);
    }

    #[test]
    fn hallucination_fixture_scores_below_half() {
        let chunks = ["The treaty was signed in 1992 in Rio."];
        let response = "The treaty was signed in 1848 on Mars and banned water.";
        let s = score_faithfulness(response, &chunks);
        assert!(
            s.composite < 0.5,
            "contradictory fixture must score below 0.5, got {}",
            s.composite
        );
        assert!(s.precision < s.recall || s.precision < 0.5);
    }

    #[test]
    fn empty_inputs_are_zero() {
        let s = score_faithfulness("", &["chunk"]);
        assert_eq!(s.composite, 0.0);
    }
}
