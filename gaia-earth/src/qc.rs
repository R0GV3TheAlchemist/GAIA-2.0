//! #46 quality tiers and a one-field assimilation blend. Not OpenDA or neural DA.

use crate::{Observation, SourceKind, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityClass {
    Reference,
    Operational,
    Mesonet,
    Archive,
    Caution,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CuratedRecord {
    pub observation: Observation,
    pub method: String,
    pub provenance: String,
    pub timestamp_unix: u64,
    pub quality: QualityClass,
}

impl CuratedRecord {
    pub fn ship(
        observation: Observation,
        method: &str,
        provenance: &str,
        timestamp_unix: u64,
        quality: QualityClass,
    ) -> Result<Self, TwinError> {
        if observation.uncertainty < 0.0 {
            return Err(TwinError::MissingUncertainty);
        }
        if method.trim().is_empty() || provenance.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(Self {
            observation,
            method: method.into(),
            provenance: provenance.into(),
            timestamp_unix,
            quality,
        })
    }
}

/// Weighted blend. Not a Kalman filter.
pub fn assimilate(model: Observation, observed: Observation) -> Result<Observation, TwinError> {
    if model.uncertainty <= 0.0 || observed.uncertainty <= 0.0 {
        return Err(TwinError::MissingUncertainty);
    }
    let w_obs = 1.0 / observed.uncertainty;
    let w_mod = 1.0 / model.uncertainty;
    let value = (observed.value * w_obs + model.value * w_mod) / (w_obs + w_mod);
    Observation::admit(
        observed.system,
        SourceKind::Synthetic,
        value,
        Some((observed.uncertainty + model.uncertainty) / 2.0),
        &observed.unit,
    )
}
