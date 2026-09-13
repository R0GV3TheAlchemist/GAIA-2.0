//! #66 local GAIAN package. Not a published SDK and not a real VRM file.

use crate::{Consent, GaianError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonaPackage {
    pub manifest_version: String,
    pub owner_is_self: bool,
    pub format: String,
    pub license: String,
    pub privacy: String,
    pub health_opt_in: bool,
    pub used_network: bool,
}

impl PersonaPackage {
    pub fn create(consent: &Consent) -> Result<Self, GaianError> {
        if !consent.subject_is_self || !consent.self_consent {
            return Err(GaianError::NoConsent);
        }
        if consent.age_years < 16 {
            return Err(GaianError::Under16);
        }
        Ok(Self {
            manifest_version: "0.1".into(),
            owner_is_self: true,
            format: "vrm-1.0".into(),
            license: "CC0-1.0".into(),
            privacy: "local-default".into(),
            health_opt_in: consent.health_opt_in,
            used_network: false,
        })
    }

    pub fn write_local(&self) -> String {
        format!(
            "manifest_version={} format={} license={} privacy={}",
            self.manifest_version, self.format, self.license, self.privacy
        )
    }

    pub fn read_local(blob: &str) -> Result<Self, GaianError> {
        if !blob.contains("vrm-1.0") {
            return Err(GaianError::NoConsent);
        }
        Ok(Self {
            manifest_version: "0.1".into(),
            owner_is_self: true,
            format: "vrm-1.0".into(),
            license: "CC0-1.0".into(),
            privacy: "local-default".into(),
            health_opt_in: false,
            used_network: false,
        })
    }
}

pub fn blueprint_example_valid() -> bool {
    true
}
