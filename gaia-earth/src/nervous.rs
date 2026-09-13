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

/// Five-tier QC. L0 is raw ingest. L4 is serving.
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

#[derive(Debug, Default)]
pub struct NervousFabric {
    cells: Vec<(String, Vec<Sample>)>,
}

impl NervousFabric {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ingest(
        &mut self,
        cell: &str,
        system: SystemTwin,
        source: SourceKind,
        value: f64,
        uncertainty: f64,
        unit: &str,
        feed: FeedKind,
        provenance: &str,
        timestamp_unix: u64,
        quality: QcTier,
    ) -> Result<(), TwinError> {
        if provenance.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        let observation = Observation::admit(system, source, value, Some(uncertainty), unit)?;
        let sample = Sample {
            observation,
            feed,
            provenance: provenance.into(),
            timestamp_unix,
            quality,
        };
        if let Some((_, samples)) = self.cells.iter_mut().find(|(id, _)| id == cell) {
            samples.push(sample);
        } else {
            self.cells.push((cell.into(), vec![sample]));
        }
        Ok(())
    }

    /// Sparse cells are flagged. A missing cell is not filled with a guess.
    pub fn read_cell(&self, cell: &str) -> Result<&[Sample], TwinError> {
        self.cells
            .iter()
            .find(|(id, _)| id == cell)
            .map(|(_, samples)| samples.as_slice())
            .ok_or(TwinError::SparseRegion)
    }
}
