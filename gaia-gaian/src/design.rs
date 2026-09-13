//! #213–#220 identity, migrate, autonomy. Default Level 1.

use crate::GaianError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub gaian_id: String,
}

impl Identity {
    pub fn new(id: &str) -> Self {
        Self {
            gaian_id: id.into(),
        }
    }
}

pub fn model_swap(id: &Identity) -> String {
    id.gaian_id.clone()
}

pub fn infer_from_photo() -> Result<(), GaianError> {
    Err(GaianError::PhotoInference)
}

pub fn migrate(id: &Identity, _from: &str, _to: &str) -> String {
    id.gaian_id.clone()
}

pub fn forget(item: &str) -> Option<&'static str> {
    let _ = item;
    None
}

pub fn default_level() -> u8 {
    1
}

pub fn pay_at_level(level: u8) -> Result<(), GaianError> {
    if level <= 2 {
        return Err(GaianError::ConfirmRequired);
    }
    Ok(())
}

pub fn dump_vault() -> Result<(), GaianError> {
    Err(GaianError::VaultDump)
}
