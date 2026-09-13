//! #203/#209 paper pilot. No invented Country site.

use crate::SaError;

#[derive(Debug)]
pub struct Packet {
    pub stages: usize,
    pub twin: &'static str,
}

impl Packet {
    pub fn paper() -> Self {
        Self {
            stages: 7,
            twin: "si:#186",
        }
    }
}

pub fn biophilia() -> usize {
    14
}

pub fn publish_site(consulted: bool) -> Result<(), SaError> {
    if !consulted {
        return Err(SaError::NoConsultation);
    }
    Ok(())
}
