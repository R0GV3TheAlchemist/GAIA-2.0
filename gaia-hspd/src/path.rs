//! #140 practice paths only. No pharmacological objects. No breath-hold protocol.

use crate::HspdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticePath {
    pub id: String,
    pub steps: Vec<String>,
    pub supplements: bool,
}

pub fn flow_path() -> PracticePath {
    PracticePath {
        id: "hspd:path:flow".into(),
        steps: vec!["attention-hygiene".into(), "open-practice".into()],
        supplements: false,
    }
}

pub fn freediving_note() -> &'static str {
    "find a certified instructor"
}

pub fn pharma_path() -> Result<PracticePath, HspdError> {
    Err(HspdError::NotPractice)
}
