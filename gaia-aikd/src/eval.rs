//! #106 calibration notes, gap detection, and EvalReport.
//! Not lm-eval-harness. Not AIKD v1.0. calibration_score() returns None.

use crate::meta::{aikd_v1_tagged, audit_report, knows_everything};
use crate::AikdError;

// ── Existing stubs (unchanged) ───────────────────────────────────────────────

pub fn measured_vs_published() -> [&'static str; 2] {
    ["published: reference-only", "gaia_measured: none-yet"]
}

pub fn overconfident_fixture_dropped(phase1_flagged: bool) -> bool {
    phase1_flagged
}

pub fn release_ready() -> bool {
    !aikd_v1_tagged()
}

// ── New types and functions for #106 interface wiring ────────────────────────

/// Full eval snapshot. All fields are fixtures — not live measurements.
/// `calibration` is `None` until TSC-approved calibration run completes.
#[derive(Debug, Clone)]
pub struct EvalReport {
    pub v1_tagged: bool,
    pub knows_all: bool,
    pub calibration: Option<f32>,
    pub audit_lines: Vec<String>,
    pub gaps: Vec<String>,
}

impl EvalReport {
    /// Build a fixture EvalReport from the current meta state.
    pub fn build() -> Self {
        let lines: Vec<String> = audit_report().iter().map(|s| s.to_string()).collect();
        Self {
            v1_tagged: aikd_v1_tagged(),
            knows_all: knows_everything(),
            calibration: calibration_score(),
            audit_lines: lines,
            gaps: vec![],
        }
    }
}

/// Calibration score. Returns `None` — not yet calibrated.
/// Callers MUST NOT treat `None` as a zero score.
pub fn calibration_score() -> Option<f32> {
    None
}

/// Detect a knowledge gap for `topic`.
/// - Empty topic → `Err(CannotKnow)`
/// - Non-empty topic → `Ok(Some(fixture gap string))`
///
/// Not connected to a live knowledge graph. Fixture only.
pub fn gap_detected(topic: &str) -> Result<Option<String>, AikdError> {
    if topic.trim().is_empty() {
        return Err(AikdError::CannotKnow);
    }
    Ok(Some(format!(
        "gap: '{}' not yet covered in AIKD Phase 1 fixture corpus",
        topic
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_is_none() {
        assert!(calibration_score().is_none());
    }

    #[test]
    fn gap_empty_cannot_know() {
        assert_eq!(gap_detected(""), Err(AikdError::CannotKnow));
        assert_eq!(gap_detected("   "), Err(AikdError::CannotKnow));
    }

    #[test]
    fn gap_topic_fixture() {
        let g = gap_detected("thermodynamics").unwrap().unwrap();
        assert!(g.contains("thermodynamics"));
    }

    #[test]
    fn eval_report_v1_false() {
        let r = EvalReport::build();
        assert!(!r.v1_tagged);
        assert!(!r.knows_all);
        assert!(r.calibration.is_none());
        assert_eq!(r.audit_lines.len(), 4);
    }

    #[test]
    fn release_ready_while_not_v1() {
        assert!(release_ready());
    }
}
