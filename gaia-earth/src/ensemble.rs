//! #48 model registry. Fixture products. Not GraphCast weights or Hugging Face.

use crate::{Lake, Observation, SourceKind, SystemTwin, TwinError};

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
    pub dated: &'static str,
    pub observation: Observation,
}

impl ModelProduct {
    pub fn infer(model: RegisteredModel, system: SystemTwin) -> Result<Self, TwinError> {
        let observation =
            Observation::admit(system, SourceKind::Synthetic, 0.0, Some(1.0), "fixture")?;
        Ok(Self {
            model,
            version: "fixture-0".into(),
            dated: "2026-09-19",
            observation,
        })
    }

    pub fn is_observation(&self) -> bool {
        self.observation.source == SourceKind::Measured
    }

    pub fn into_lake(&self, lake: &mut Lake) -> Result<(), TwinError> {
        let id = match self.model {
            RegisteredModel::GraphCast => "model.graphcast.fixture",
            RegisteredModel::TerraMind => "model.terramind.fixture",
            RegisteredModel::Esfm => "model.esfm.fixture",
            RegisteredModel::Aurora => "model.aurora.fixture",
            RegisteredModel::BiodiversityTwin => "model.biodiversity.fixture",
        };
        lake.write_raw(id, self.observation.clone())?;
        lake.promote(id)?;
        Ok(())
    }
}
