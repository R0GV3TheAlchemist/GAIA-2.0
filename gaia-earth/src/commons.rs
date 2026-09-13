//! #34 Data Commons catalog. In-process fixtures. Not Iceberg, MinIO, Kafka, or STAC-on-the-wire.

use crate::{Observation, SourceKind, SystemTwin, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityTier {
    Raw,
    Curated,
    Enriched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    Satellite,
    Weather,
    Biodiversity,
    SeismicOcean,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    pub id: String,
    pub domain: Domain,
    pub system: SystemTwin,
    pub source_name: String,
    pub license: String,
    pub quality: QualityTier,
    pub provenance: String,
}

#[derive(Debug, Default)]
pub struct Commons {
    collections: Vec<Collection>,
}

impl Commons {
    pub fn seed() -> Self {
        Self {
            collections: vec![
                Collection {
                    id: "sentinel-2-l1c-fixture".into(),
                    domain: Domain::Satellite,
                    system: SystemTwin::Land,
                    source_name: "Copernicus Data Space (fixture)".into(),
                    license: "CC-BY-4.0".into(),
                    quality: QualityTier::Curated,
                    provenance: "synthetic fixture labeled as such; not a live Copernicus pull".into(),
                },
                Collection {
                    id: "ground-weather-fixture".into(),
                    domain: Domain::Weather,
                    system: SystemTwin::Atmosphere,
                    source_name: "Open Weather stations (fixture)".into(),
                    license: "CC0".into(),
                    quality: QualityTier::Raw,
                    provenance: "synthetic fixture; not a live station feed".into(),
                },
                Collection {
                    id: "gbif-occurrence-fixture".into(),
                    domain: Domain::Biodiversity,
                    system: SystemTwin::Biosphere,
                    source_name: "GBIF (fixture)".into(),
                    license: "CC-BY-4.0".into(),
                    quality: QualityTier::Curated,
                    provenance: "synthetic fixture; not a live GBIF download".into(),
                },
                Collection {
                    id: "usgs-seismic-fixture".into(),
                    domain: Domain::SeismicOcean,
                    system: SystemTwin::Lithosphere,
                    source_name: "USGS seismic (fixture)".into(),
                    license: "CC0".into(),
                    quality: QualityTier::Raw,
                    provenance: "synthetic fixture; not a live USGS event".into(),
                },
            ],
        }
    }

    pub fn collections(&self) -> &[Collection] {
        &self.collections
    }

    pub fn get(&self, id: &str) -> Result<&Collection, TwinError> {
        self.collections
            .iter()
            .find(|c| c.id == id)
            .ok_or(TwinError::UnknownCollection)
    }

    pub fn query(&self, id: &str, value: f64, uncertainty: f64, unit: &str) -> Result<Observation, TwinError> {
        let collection = self.get(id)?;
        Observation::admit(
            collection.system,
            SourceKind::Synthetic,
            value,
            Some(uncertainty),
            unit,
        )
    }

    /// STAC-shaped record for docs and tests. Not a published catalog.
    pub fn stac_item(&self, id: &str) -> Result<String, TwinError> {
        let collection = self.get(id)?;
        Ok(format!(
            "{{"type":"Feature","stac_version":"1.0.0","id":"{}","properties":{{"license":"{}","quality":"{:?}","provenance":"{}"}}}}",
            collection.id, collection.license, collection.quality, collection.provenance
        ))
    }
}
