//! #50 three-mode job API. Fixture runs. Not OpenIFS, CESM, NEMO, or Slurm.

use crate::simulate::SimMode;
use crate::{Observation, SourceKind, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridScale {
    Global25km,
    Continental5km,
    National1km,
    Urban100m,
    Local1m,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimJob {
    pub mode: SimMode,
    pub region: String,
    pub horizon_days: u32,
    pub ensemble_size: u32,
    pub scale: GridScale,
    pub cadence_secs: u64,
    pub last_tick: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimRun {
    pub job: SimJob,
    pub members: Vec<Observation>,
}

impl SimJob {
    pub fn demo_now(region: &str, now: u64) -> Self {
        Self {
            mode: SimMode::WhatNow,
            region: region.into(),
            horizon_days: 0,
            ensemble_size: 1,
            scale: GridScale::Urban100m,
            cadence_secs: 3600,
            last_tick: now,
        }
    }

    pub fn tick(&mut self, now: u64) {
        if now >= self.last_tick + self.cadence_secs {
            self.last_tick = now;
        }
    }

    pub fn run(&self, observations: &[Observation]) -> Result<SimRun, TwinError> {
        if self.region.trim().is_empty() || self.ensemble_size == 0 {
            return Err(TwinError::UnlabeledPoint);
        }
        let members = (0..self.ensemble_size)
            .map(|_| {
                Observation::admit(
                    crate::SystemTwin::Atmosphere,
                    SourceKind::Synthetic,
                    0.0,
                    Some(1.0),
                    "fixture",
                )
                .unwrap()
            })
            .collect();
        let _ = observations;
        Ok(SimRun {
            job: self.clone(),
            members,
        })
    }
}
