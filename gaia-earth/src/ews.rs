//! #54 boundary monitor and threshold warning. Not AdvanTip or live Rockström feeds.

use crate::TwinError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchItem {
    ClimateChange,
    BiosphereIntegrity,
    LandSystemChange,
    FreshwaterChange,
    BiogeochemicalFlows,
    OceanAcidification,
    AtmosphericAerosol,
    StratosphericOzone,
    NovelEntities,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WatchState {
    pub item: WatchItem,
    pub source: String,
    pub timestamp_unix: u64,
    pub uncertainty: f64,
    pub ratio: f64,
}

pub struct BoundaryMonitor {
    states: Vec<WatchState>,
}

impl BoundaryMonitor {
    pub fn seed(now: u64) -> Self {
        Self {
            states: WatchItem::all()
                .into_iter()
                .map(|item| WatchState {
                    item,
                    source: "fixture-indicator".into(),
                    timestamp_unix: now,
                    uncertainty: 0.1,
                    ratio: 0.4,
                })
                .collect(),
        }
    }

    pub fn states(&self) -> &[WatchState] {
        &self.states
    }

    pub fn approach(&mut self, item: WatchItem, ratio: f64) -> Result<bool, TwinError> {
        if ratio < 0.0 {
            return Err(TwinError::MissingUncertainty);
        }
        let state = self
            .states
            .iter_mut()
            .find(|s| s.item == item)
            .ok_or(TwinError::UnlabeledPoint)?;
        state.ratio = ratio;
        Ok(ratio >= 0.8)
    }
}

impl WatchItem {
    pub fn all() -> [WatchItem; 9] {
        [
            Self::ClimateChange,
            Self::BiosphereIntegrity,
            Self::LandSystemChange,
            Self::FreshwaterChange,
            Self::BiogeochemicalFlows,
            Self::OceanAcidification,
            Self::AtmosphericAerosol,
            Self::StratosphericOzone,
            Self::NovelEntities,
        ]
    }
}
