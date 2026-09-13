//! #39 guardian loop. Fixture boundaries. Not Twin v1.0 and not a live EWS.

use crate::{allow_purpose, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boundary {
    ClimateChange,
    BiosphereIntegrity,
    LandSystemChange,
    FreshwaterChange,
    BiogeochemicalFlows,
    OceanAcidification,
    AtmosphericAerosol,
    StratosphericOzone,
    NovelEntities,
}

impl Boundary {
    pub fn all() -> [Boundary; 9] {
        [
            Self::ClimateChange,
            Self::BiosphereIntegrity,
            Self::LandSystemChange,
            Self::FreshwaterChange,
            Self::BiogeochemicalFlows,
            Self::OceanAcidification,
            Self::AtmosphericAerosol,
            Self::StratosphericOzone,
            Self::NovelEntities,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryStatus {
    Safe,
    Approaching,
    Exceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorrectionStep {
    Alert,
    Diagnose,
    Simulate,
    Recommend,
    Track,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryState {
    pub boundary: Boundary,
    pub status: BoundaryStatus,
    pub ratio: f64,
    pub uncertainty: f64,
    pub method: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Correction {
    pub boundary: Boundary,
    pub steps: [CorrectionStep; 5],
    pub recommendation: String,
}

#[derive(Debug)]
pub struct Guardian {
    states: Vec<BoundaryState>,
}

impl Guardian {
    pub fn seed() -> Self {
        Self {
            states: Boundary::all()
                .into_iter()
                .map(|boundary| BoundaryState {
                    boundary,
                    status: BoundaryStatus::Safe,
                    ratio: 0.4,
                    uncertainty: 0.1,
                    method: "fixture-ratio".into(),
                })
                .collect(),
        }
    }

    pub fn states(&self) -> &[BoundaryState] {
        &self.states
    }

    pub fn set(
        &mut self,
        boundary: Boundary,
        ratio: f64,
        uncertainty: f64,
        method: &str,
    ) -> Result<&BoundaryState, TwinError> {
        if uncertainty < 0.0 {
            return Err(TwinError::MissingUncertainty);
        }
        if method.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        let status = if ratio >= 1.0 {
            BoundaryStatus::Exceeded
        } else if ratio >= 0.8 {
            BoundaryStatus::Approaching
        } else {
            BoundaryStatus::Safe
        };
        let slot = self
            .states
            .iter_mut()
            .find(|s| s.boundary == boundary)
            .ok_or(TwinError::UnlabeledPoint)?;
        slot.ratio = ratio;
        slot.uncertainty = uncertainty;
        slot.method = method.into();
        slot.status = status;
        Ok(slot)
    }

    /// Approaching or exceeded fires the five-step loop. Weaponized purpose is refused.
    pub fn correct(&self, boundary: Boundary, purpose: &str) -> Result<Correction, TwinError> {
        allow_purpose(purpose)?;
        let state = self
            .states
            .iter()
            .find(|s| s.boundary == boundary)
            .ok_or(TwinError::UnlabeledPoint)?;
        if state.status == BoundaryStatus::Safe {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(Correction {
            boundary,
            steps: [
                CorrectionStep::Alert,
                CorrectionStep::Diagnose,
                CorrectionStep::Simulate,
                CorrectionStep::Recommend,
                CorrectionStep::Track,
            ],
            recommendation: "fixture intervention ensemble; not a live policy model".into(),
        })
    }
}
