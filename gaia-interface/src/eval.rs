//! #106 calibration and eval wiring for gaia-interface — audit report,
//! v1.0 gate guards, calibration score surface, and gap detection.
//!
//! Follows the same escape() + Result propagation pattern as agentic.rs and packs.rs.

use gaia_aikd::{
    aikd_v1_tagged, audit_report, calibration_score, gap_detected, knows_everything, AikdError,
    EvalReport,
};

// ── Audit report ─────────────────────────────────────────────────────────────

/// Render the audit report as a plain-text block.
/// Fixture lines only — callers MUST NOT present these as measured scores.
pub fn render_audit_report() -> String {
    let lines = audit_report();
    lines.join("\n")
}

/// HTML variant — renders each audit line as a `<li>` inside a `<ul>`.
pub fn render_audit_report_html() -> String {
    let lines = audit_report();
    let items: String = lines
        .iter()
        .map(|l| format!("<li>{}</li>", escape(l)))
        .collect();
    format!("<ul class=\"audit-report\">{items}</ul>")
}

// ── v1.0 gate guards ─────────────────────────────────────────────────────────

/// Named guard over `aikd_v1_tagged`.
/// Returns an error string when called from any path that requires v1.0.
pub fn guard_not_v1() -> Result<(), String> {
    if aikd_v1_tagged() {
        return Err("unexpected: AIKD v1.0 must not be tagged without TSC vote".into());
    }
    Ok(())
}

/// Named guard over `knows_everything`.
/// Wire to any path that would claim omniscience.
pub fn guard_does_not_know_everything() -> Result<(), String> {
    if knows_everything() {
        return Err("GAIA does not know everything".into());
    }
    Ok(())
}

// ── Calibration score ────────────────────────────────────────────────────────

/// Render the calibration score. Surfaces `None` as "not yet calibrated".
/// Callers MUST NOT treat None as a zero score.
pub fn render_calibration() -> String {
    match calibration_score() {
        Some(score) => format!("[calibration] score={score:.3}"),
        None => "[calibration] not yet calibrated".into(),
    }
}

// ── Gap detection ────────────────────────────────────────────────────────────

/// Surface a gap-detection result for a topic.
/// Returns `Err` for empty topic (CannotKnow) or a user-facing gap string.
pub fn render_gap(topic: &str) -> Result<String, String> {
    match gap_detected(topic) {
        Ok(Some(msg)) => Ok(format!("[gap] {}", escape(&msg))),
        Ok(None) => Ok("[gap] no gap detected".into()),
        Err(AikdError::CannotKnow) => Err("cannot detect gap: empty topic".into()),
        Err(_) => Err("gap detection refused".into()),
    }
}

// ── EvalReport surface ───────────────────────────────────────────────────────

/// Build and render a full EvalReport as a plain-text summary.
pub fn render_eval_report() -> String {
    let report = EvalReport::build();
    let cal = match report.calibration {
        Some(s) => format!("{s:.3}"),
        None => "none".into(),
    };
    format!(
        "v1_tagged={v} knows_all={k} calibration={c} audit_lines={n} gaps={g}",
        v = report.v1_tagged,
        k = report.knows_all,
        c = cal,
        n = report.audit_lines.len(),
        g = report.gaps.len(),
    )
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_report_has_four_lines() {
        let text = render_audit_report();
        let lines: Vec<_> = text.lines().collect();
        assert_eq!(lines.len(), 4);
        assert!(lines[0].contains("none yet"));
        assert!(lines[3].contains("unmeasured"));
    }

    #[test]
    fn audit_report_html_has_ul() {
        let html = render_audit_report_html();
        assert!(html.contains("<ul"));
        assert!(html.contains("<li>"));
    }

    #[test]
    fn v1_not_tagged() {
        assert!(guard_not_v1().is_ok());
    }

    #[test]
    fn does_not_know_everything() {
        assert!(guard_does_not_know_everything().is_ok());
    }

    #[test]
    fn calibration_not_yet() {
        let line = render_calibration();
        assert!(line.contains("not yet calibrated"));
    }

    #[test]
    fn gap_empty_topic_refused() {
        assert!(render_gap("").is_err());
    }

    #[test]
    fn gap_detected_topic() {
        let msg = render_gap("thermodynamics").unwrap();
        assert!(msg.contains("thermodynamics"));
    }

    #[test]
    fn eval_report_summary() {
        let s = render_eval_report();
        assert!(s.contains("v1_tagged=false"));
        assert!(s.contains("knows_all=false"));
        assert!(s.contains("calibration=none"));
        assert!(s.contains("audit_lines=4"));
    }
}
