//! #48 model registry. Fixture products. Not GraphCast weights or Hugging Face.

use crate::{Observation, SourceKind, SystemTwin, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisteredModel {
    GraphCast,
    TerraMind,
    Esfm,
    Aurora,
    BiodiversityTwin,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelProduct {
    pub model: RegisteredModel,
    pub version: String,
    pub observation: Observation,
}

impl ModelProduct {
    pub fn infer(model: RegisteredModel, system: SystemTwin) -> Result<Self, TwinError> {
        let observation = Observation::admit(
            system,
            SourceKind::Synthetic,
            0.0,
            Some(1.0),
            "fixture",
        )?;
        Ok(Self {
            model,
            version: "fixture-0".into(),
            observation,
        })
    }

    pub fn is_observation(&self) -> bool {
        self.observation.source == SourceKind::Measured
    }
}
