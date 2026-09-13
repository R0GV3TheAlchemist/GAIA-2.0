//! #73 health sensors stub. Off on first launch. Not a wearable adapter.

use crate::{Consent, GaianError};

#[derive(Debug)]
pub struct HealthModule {
    pub first_launch: bool,
    pub enabled: bool,
    log: Vec<String>,
}

impl HealthModule {
    pub fn first_launch() -> Self {
        Self {
            first_launch: true,
            enabled: false,
            log: vec![],
        }
    }

    pub fn enable_learning(&mut self, consent: &Consent) -> Result<(), GaianError> {
        if consent.age_years < 16 {
            return Err(GaianError::Under16);
        }
        if !consent.health_opt_in {
            return Err(GaianError::HealthNotOptIn);
        }
        self.enabled = true;
        self.first_launch = false;
        Ok(())
    }

    pub fn export(&mut self, purpose: &str) -> Result<(), GaianError> {
        let p = purpose.trim().to_ascii_lowercase();
        if p.is_empty() {
            return Err(GaianError::NoConsent);
        }
        if p.contains("insur") || p.contains("employ") {
            return Err(GaianError::HealthNotOptIn);
        }
        self.log.push(format!("export purpose={purpose}"));
        Ok(())
    }

    pub fn is_diagnostic_device(&self) -> bool {
        false
    }

    pub fn log(&self) -> &[String] {
        &self.log
    }
}
