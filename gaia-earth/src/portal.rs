//! #52 public surface. Layer list and contribution pin. Not Cesium.

use crate::TwinError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobeLayer {
    Live,
    Forecast,
    Scenario,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeCursor {
    Past,
    Now,
    Forecast,
    Scenario,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortalPin {
    pub place: String,
    pub license: String,
    pub attribution: String,
    pub layer: GlobeLayer,
}

impl PortalPin {
    pub fn contribute(
        place: &str,
        license: &str,
        attribution: &str,
        layer: GlobeLayer,
    ) -> Result<Self, TwinError> {
        if place.trim().is_empty() || license.trim().is_empty() || attribution.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(Self {
            place: place.into(),
            license: license.into(),
            attribution: attribution.into(),
            layer,
        })
    }
}

pub fn demo_globe() -> [GlobeLayer; 3] {
    [GlobeLayer::Live, GlobeLayer::Forecast, GlobeLayer::Scenario]
}
