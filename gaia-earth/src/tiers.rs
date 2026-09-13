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

    pub fn put(&mut self, cube: TierCube) {
        self.cubes.push(cube);
    }

    pub fn fetch(&self, place: &str, tier: MemoryTier) -> Result<&TierCube, TwinError> {
        self.cubes
            .iter()
            .find(|c| c.place == place && c.tier == tier)
            .ok_or(TwinError::UnlabeledPoint)
    }

    pub fn associate(&mut self, from_id: &str, to_id: &str) -> Result<(), TwinError> {
        let exists = self.cubes.iter().any(|c| c.id == to_id);
        if !exists {
            return Err(TwinError::UnlabeledPoint);
        }
        let cube = self
            .cubes
            .iter_mut()
            .find(|c| c.id == from_id)
            .ok_or(TwinError::UnlabeledPoint)?;
        cube.associations.push(to_id.into());
        Ok(())
    }
}
