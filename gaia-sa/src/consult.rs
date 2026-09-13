//! #202/#207 TEK protocol. Consultation can return no.

use crate::SaError;

pub fn consult(granted: bool) -> Result<(), SaError> {
    if !granted {
        return Err(SaError::TekVeto);
    }
    Ok(())
}

pub fn ingest_site(tek_state: Option<&str>) -> Result<(), SaError> {
    if tek_state.is_none() {
        return Err(SaError::NoConsultation);
    }
    Ok(())
}

pub fn neuro_claim(cited: bool) -> Result<&'static str, SaError> {
    if cited {
        Ok("graded")
    } else {
        Ok("ungraded")
    }
}
