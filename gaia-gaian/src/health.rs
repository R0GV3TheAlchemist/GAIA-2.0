//! #62 health twin stub. Defaults off. Not a wearable or clinical model.

use crate::{Consent, GaianError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthMetric {
    HeartRate,
    Sleep,
    Steps,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HealthTwin {
    pub enabled: bool,
    metrics: Vec<(HealthMetric, f64)>,
    genetic_isolated: bool,
}

impl HealthTwin {
    pub fn default_off() -> Self {
        Self {
            enabled: false,
            metrics: vec![],
            genetic_isolated: false,
        }
    }

    pub fn enable(&mut self, consent: &Consent) -> Result<(), GaianError> {
        if consent.age_years < 16 {
            return Err(GaianError::Under16);
        }
        if !consent.health_opt_in {
            return Err(GaianError::HealthNotOptIn);
        }
        self.enabled = true;
        Ok(())
    }

    pub fn admit(
        &mut self,
        consent: &Consent,
        metric: HealthMetric,
        value: f64,
        metric_consent: bool,
    ) -> Result<(), GaianError> {
        if !self.enabled {
            return Err(GaianError::HealthDefaultOff);
        }
        if !consent.health_opt_in || !metric_consent {
            return Err(GaianError::HealthNotOptIn);
        }
        self.metrics.push((metric, value));
        Ok(())
    }

    pub fn export_insurer(&self) -> Result<(), GaianError> {
        Err(GaianError::ExportForbidden)
    }

    pub fn export_employer(&self) -> Result<(), GaianError> {
        Err(GaianError::ExportForbidden)
    }

    pub fn isolate_genetic_upload(&mut self) {
        self.genetic_isolated = true;
    }

    pub fn genetic_is_isolated(&self) -> bool {
        self.genetic_isolated
    }
}

pub fn future_self(question: &str) -> Result<&'static str, GaianError> {
    let lower = question.to_ascii_lowercase();
    if lower.contains("diagnose") || lower.contains("prescribe") {
        return Err(GaianError::NotMedicalAdvice);
    }
    Ok("advisory fixture; not medical advice")
}

pub fn age_progress_own(consent: &Consent) -> Result<&'static str, GaianError> {
    if !consent.subject_is_self {
        return Err(GaianError::NotSelf);
    }
    if consent.age_years < 16 {
        return Err(GaianError::Under16);
    }
    Ok("own-avatar-age-progress-stub")
}
