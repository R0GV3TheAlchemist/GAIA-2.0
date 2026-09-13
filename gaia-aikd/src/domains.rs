//! #96 domain adapters. Professional packs are not advice.

use crate::AikdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adapter {
    Math,
    Code,
    Science,
    Vision,
    MedicalRef,
    LegalRef,
}

impl Adapter {
    pub fn eval_slice(self) -> &'static str {
        match self {
            Self::Math => "math-eval-slice",
            Self::Code => "code-eval-slice",
            Self::Science => "science-eval-slice",
            Self::Vision => "vision-eval-slice",
            Self::MedicalRef => "medical-ref-eval-slice",
            Self::LegalRef => "legal-ref-eval-slice",
        }
    }

    pub fn answer(self, cited: bool, uncertainty: bool) -> Result<&'static str, AikdError> {
        match self {
            Self::MedicalRef | Self::LegalRef if !cited || !uncertainty => {
                Err(AikdError::MissingCitation)
            }
            Self::MedicalRef => Ok("reference only; not medical advice"),
            Self::LegalRef => Ok("reference only; not legal advice"),
            _ => Ok(self.eval_slice()),
        }
    }

    pub fn is_practice_license(self) -> bool {
        false
    }
}
