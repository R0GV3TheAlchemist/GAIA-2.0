//! #185 purpose-tagged streams.

use crate::SiError;

#[derive(Debug, Clone)]
pub struct Stream {
    pub purpose: String,
    pub retention: String,
    pub pii: Vec<String>,
    pub cameras: bool,
    pub autonomy: &'static str,
}

impl Stream {
    pub fn plant(purpose: &str) -> Self {
        Self {
            purpose: purpose.into(),
            retention: "session".into(),
            pii: vec![],
            cameras: false,
            autonomy: "none",
        }
    }
}

pub fn admit(purpose: &str) -> Result<Stream, SiError> {
    if purpose.is_empty() {
        return Err(SiError::NoPurpose);
    }
    Ok(Stream::plant(purpose))
}

pub fn face_field() -> Result<(), SiError> {
    Err(SiError::FaceUnsupported)
}
