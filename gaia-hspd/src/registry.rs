//! #143 allow-listed classes. Not a drugstore. Not HSPD v1.0.

use crate::HspdError;

#[derive(Debug, Clone)]
pub struct RegistryItem {
    pub name: String,
    pub checkout: bool,
    pub age_gate: u8,
}

pub fn list_for(age: u8) -> Vec<RegistryItem> {
    if age < 18 {
        return vec![RegistryItem {
            name: "memory-palace-card".into(),
            checkout: false,
            age_gate: 0,
        }];
    }
    vec![RegistryItem {
        name: "openbci-research-class".into(),
        checkout: false,
        age_gate: 18,
    }]
}

pub fn checkout(_item: &str) -> Result<(), HspdError> {
    Err(HspdError::CheckoutForbidden)
}

pub fn hspd_v1_tagged() -> bool {
    false
}
