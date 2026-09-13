//! #178 purpose-tagged streams. PII empty.

#[derive(Debug, Clone)]
pub struct Stream {
    pub purpose: String,
    pub retention: String,
    pub pii: Vec<String>,
    pub cameras: bool,
}

impl Stream {
    pub fn plant(purpose: &str) -> Self {
        Self {
            purpose: purpose.into(),
            retention: "session".into(),
            pii: vec![],
            cameras: false,
        }
    }
}
