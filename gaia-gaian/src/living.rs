//! #69 expression stub. Not ARKit, WebGPU, or C2PA.

use crate::GaianError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedSession {
    pub owner: bool,
    pub signed: bool,
}

impl SignedSession {
    pub fn drive_speech(&self, chunk: &str) -> Result<String, GaianError> {
        if !self.signed || !self.owner {
            return Err(GaianError::Unsigned);
        }
        Ok(format!("streaming-face-body:{chunk}"))
    }

    pub fn export_clip(&self) -> Result<&'static str, GaianError> {
        if !self.signed {
            return Err(GaianError::Unsigned);
        }
        Ok("gaian-generated-fixture; not a C2PA signature")
    }
}

pub fn animate_third_party_face(_image: &str) -> Result<(), GaianError> {
    Err(GaianError::ThirdPartyLikeness)
}
