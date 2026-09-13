//! #76 export mark fixture. Not c2pa-rs and not GAIAN v1.0.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportMark {
    pub gaian_id: String,
    pub consent_id: String,
    pub generated: bool,
    pub warehouse: bool,
}

impl ExportMark {
    pub fn sign(gaian_id: &str, consent_id: &str) -> Self {
        Self {
            gaian_id: gaian_id.into(),
            consent_id: consent_id.into(),
            generated: true,
            warehouse: false,
        }
    }

    pub fn detect_tamper(&self, altered: bool) -> bool {
        altered
    }

    pub fn verifies_with_c2pa_tooling(&self) -> bool {
        false
    }
}

pub fn qa_script() -> [&'static str; 5] {
    [
        "delete-my-GAIAN",
        "child-safety",
        "non-impersonation",
        "equity-eval",
        "no-GAIAN-v1.0-tag",
    ]
}
