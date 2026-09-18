//! #47 memory tiers. Same API for paleo and live. Not PostGIS or ERA5.

use crate::{Observation, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTier {
    DeepTime,
    Instrumental,
    Recent30Days,
    RealTime,
    Future,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TierCube {
    pub id: String,
    pub tier: MemoryTier,
    pub place: String,
    pub observation: Observation,
    pub associations: Vec<String>,
}

#[derive(Debug, Default)]
pub struct TierStore {
    cubes: Vec<TierCube>,
}

impl TierStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, cube: TierCube) -> Result<(), TwinError> {
        if self.cubes.iter().any(|existing| existing.id == cube.id) {
            return Err(TwinError::UnlabeledPoint);
        }
        self.cubes.push(cube);
        Ok(())
    }

    pub fn fetch(&self, place: &str, tier: MemoryTier) -> Result<&TierCube, TwinError> {
        self.cubes
            .iter()
            .find(|cube| cube.place == place && cube.tier == tier)
            .ok_or(TwinError::UnlabeledPoint)
    }

    pub fn associate(&mut self, from_id: &str, to_id: &str) -> Result<(), TwinError> {
        if from_id == to_id || !self.cubes.iter().any(|cube| cube.id == to_id) {
            return Err(TwinError::UnlabeledPoint);
        }

        let cube = self
            .cubes
            .iter_mut()
            .find(|cube| cube.id == from_id)
            .ok_or(TwinError::UnlabeledPoint)?;

        if !cube.associations.iter().any(|id| id == to_id) {
            cube.associations.push(to_id.into());
        }
        Ok(())
    }
}
