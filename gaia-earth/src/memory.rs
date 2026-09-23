//! #36 Planetary memory and model ensemble.
//! In-process cubes. Not ERA5, CMIP6, GraphCast, or TerraMind weights.

use crate::{Observation, SourceKind, SystemTwin, TwinError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceTime {
    pub place: String,
    pub timestamp_unix: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanetaryMemCube {
    pub place: String,
    pub timestamp_unix: u64,
    pub observation: Observation,
    pub fingerprint: String,
    pub signed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelOutput {
    pub name: String,
    pub version: String,
    pub value: f64,
    pub uncertainty: f64,
    pub source: SourceKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CubeSet {
    pub cubes: Vec<PlanetaryMemCube>,
    pub models: Vec<ModelOutput>,
}

/// Parameters for [`PlanetaryMemory::store`].
/// Introduced to satisfy `clippy::too_many_arguments` (limit 7).
#[derive(Debug, Clone)]
pub struct MemStoreParams {
    pub place: String,
    pub timestamp_unix: u64,
    pub system: SystemTwin,
    pub source: SourceKind,
    pub value: f64,
    pub uncertainty: f64,
    pub unit: String,
}

#[derive(Debug, Default)]
pub struct PlanetaryMemory {
    cubes: Vec<PlanetaryMemCube>,
}

impl PlanetaryMemory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store(&mut self, p: MemStoreParams) -> Result<PlanetaryMemCube, TwinError> {
        let observation =
            Observation::admit(p.system, p.source, p.value, Some(p.uncertainty), &p.unit)?;
        let fp = fingerprint(&p.place, p.timestamp_unix, p.value, p.uncertainty);
        let cube = PlanetaryMemCube {
            place: p.place,
            timestamp_unix: p.timestamp_unix,
            observation,
            fingerprint: fp,
            signed: true,
        };
        self.cubes.push(cube.clone());
        Ok(cube)
    }

    pub fn query(&self, place: &str, timestamp_unix: u64) -> Result<CubeSet, TwinError> {
        let cubes: Vec<_> = self
            .cubes
            .iter()
            .filter(|c| c.place == place && c.timestamp_unix == timestamp_unix)
            .cloned()
            .collect();
        if cubes.is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(CubeSet {
            cubes,
            models: ensemble_stub(),
        })
    }
}

fn ensemble_stub() -> Vec<ModelOutput> {
    ["GraphCast", "AIFS", "TerraMind"]
        .into_iter()
        .map(|name| ModelOutput {
            name: name.into(),
            version: "fixture-0".into(),
            value: 0.0,
            uncertainty: 1.0,
            source: SourceKind::Synthetic,
        })
        .collect()
}

fn fingerprint(place: &str, time: u64, value: f64, uncertainty: f64) -> String {
    format!("pmc:{place}:{time}:{value}:{uncertainty}")
}
