//! #41 lake zones. In-process tables. Not MinIO, Iceberg, Spark, or a STAC server.

use crate::{Observation, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LakeZone {
    Raw,
    Curated,
    Enriched,
    Serving,
    Archive,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LakeRow {
    pub id: String,
    pub zone: LakeZone,
    pub observation: Observation,
    pub doi: String,
}

#[derive(Debug, Default)]
pub struct Lake {
    raw: Vec<LakeRow>,
    curated: Vec<LakeRow>,
}

impl Lake {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_raw(&mut self, id: &str, observation: Observation) -> Result<&LakeRow, TwinError> {
        if self.raw.iter().any(|row| row.id == id) {
            return Err(TwinError::ImmutableRaw);
        }
        self.raw.push(LakeRow {
            id: id.into(),
            zone: LakeZone::Raw,
            observation,
            doi: format!("gaia:10.placeholder/{id}"),
        });
        Ok(self.raw.last().unwrap())
    }

    pub fn promote(&mut self, id: &str) -> Result<&LakeRow, TwinError> {
        let raw = self
            .raw
            .iter()
            .find(|row| row.id == id)
            .ok_or(TwinError::UnknownCollection)?
            .clone();
        if self.curated.iter().any(|row| row.id == id) {
            return Ok(self.curated.iter().find(|row| row.id == id).unwrap());
        }
        self.curated.push(LakeRow {
            id: raw.id,
            zone: LakeZone::Curated,
            observation: raw.observation,
            doi: raw.doi,
        });
        Ok(self.curated.last().unwrap())
    }

    pub fn raw(&self) -> &[LakeRow] {
        &self.raw
    }

    pub fn curated(&self) -> &[LakeRow] {
        &self.curated
    }

    pub fn stac_collection(id: &str) -> String {
        format!("stac_version=1.0.0 type=Collection id={id}")
    }

    pub fn openeo_graph(id: &str) -> String {
        format!("openeo load_collection={id} process=mean")
    }
}
