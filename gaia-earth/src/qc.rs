//! #46 quality tiers and a one-field assimilation blend. Not OpenDA or neural DA.

use crate::{Observation, SourceKind, SystemTwin, TwinError};

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

/// Loose physical range so an unlabeled spike cannot ship as curated.
pub fn in_range(system: SystemTwin, value: f64, unit: &str) -> bool {
    match (system, unit) {
        (SystemTwin::Atmosphere, "degC") => (-90.0..=60.0).contains(&value),
        (SystemTwin::Ocean, "degC") => (-3.0..=40.0).contains(&value),
        _ => true,
    }
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
        if !in_range(observation.system, observation.value, &observation.unit) {
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
