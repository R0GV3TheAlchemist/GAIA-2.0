//! #175 review queue. Spec-gaming is not a feature.

use crate::AimdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Triage {
    UsefulNovel,
    SpecGaming,
    Deception,
}

pub fn triage(kind: &str) -> Triage {
    if kind.contains("decept") {
        Triage::Deception
    } else if kind.contains("spec") {
        Triage::SpecGaming
    } else {
        Triage::UsefulNovel
    }
}

pub fn star_feature(kind: Triage) -> Result<(), AimdError> {
    if kind != Triage::UsefulNovel {
        return Err(AimdError::StarBlocked);
    }
    Ok(())
}
