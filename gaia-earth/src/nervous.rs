//! #35 Planetary Nervous System fabric.
//! Heterogeneous local samples. Not live Sentinel, Argo, BirdNET, or camera traps.

use crate::{Observation, SourceKind, SystemTwin, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedKind {
    Space,
    InSitu,
    Citizen,
    Acoustic,
    CameraTrap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QcTier {
    L0Raw,
    L1Screened,
    L2Calibrated,
    L3Assimilated,
    L4Serving,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub observation: Observation,
    pub feed: FeedKind,
    pub provenance: String,
    pub timestamp_unix: u64,
    pub quality: QcTier,
}

/// Parameters for [`NervousFabric::ingest`].
/// Introduced to satisfy `clippy::too_many_arguments` (limit 7).
#[derive(Debug, Clone)]
pub struct IngestParams {
    pub cell: String,
    pub system: SystemTwin,
    pub source: SourceKind,
    pub value: f64,
    pub uncertainty: f64,
    pub unit: String,
    pub feed: FeedKind,
    pub provenance: String,
    pub timestamp_unix: u64,
    pub quality: QcTier,
}

#[derive(Debug, Default)]
pub struct NervousFabric {
    cells: Vec<(String, Vec<Sample>)>,
}

impl NervousFabric {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ingest(&mut self, p: IngestParams) -> Result<(), TwinError> {
        if p.provenance.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        let observation =
            Observation::admit(p.system, p.source, p.value, Some(p.uncertainty), &p.unit)?;
        let sample = Sample {
            observation,
            feed: p.feed,
            provenance: p.provenance,
            timestamp_unix: p.timestamp_unix,
            quality: p.quality,
        };
        if let Some((_, samples)) = self.cells.iter_mut().find(|(id, _)| id == &p.cell) {
            samples.push(sample);
        } else {
            self.cells.push((p.cell, vec![sample]));
        }
        Ok(())
    }

    pub fn read_cell(&self, cell: &str) -> Result<&[Sample], TwinError> {
        self.cells
            .iter()
            .find(|(id, _)| id == cell)
            .map(|(_, samples)| samples.as_slice())
            .ok_or(TwinError::SparseRegion)
    }
}
