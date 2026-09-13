//! #55 course-correction drill. Not Twin v1.0 and not a live fire-season model.

use crate::{allow_purpose, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrillStep {
    Alert,
    Diagnose,
    Simulate,
    Recommend,
    Track,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HistoricalDrill {
    pub event: String,
    pub cube_id: String,
    pub moved_indicator: bool,
    pub steps: [DrillStep; 5],
}

impl HistoricalDrill {
    pub fn bleaching(purpose: &str) -> Result<Self, TwinError> {
        allow_purpose(purpose)?;
        Ok(Self {
            event: "documented bleaching fixture".into(),
            cube_id: "memcube-bleaching-fixture".into(),
            moved_indicator: false,
            steps: [
                DrillStep::Alert,
                DrillStep::Diagnose,
                DrillStep::Simulate,
                DrillStep::Recommend,
                DrillStep::Track,
            ],
        })
    }
}

pub fn release_checklist() -> [&'static str; 4] {
    [
        "non-weaponization check",
        "sovereignty check",
        "no Twin v1.0 tag",
        "known gaps listed",
    ]
}
