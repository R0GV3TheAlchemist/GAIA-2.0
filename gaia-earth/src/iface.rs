//! #38 Human-Earth interface.
//! Same underlying state for dashboard, scientist API, and a locale question.
//! Not Cesium, not a multilingual model, not a mobile app.

use crate::{Observation, SourceKind, SystemTwin, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessTier {
    BasicFree,
    Scientist,
    Minister,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlaceState {
    pub place: String,
    pub observation: Observation,
    pub paywall: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitizenCredit {
    pub place: String,
    pub license: String,
    pub attribution: String,
}

#[derive(Debug, Default)]
pub struct EarthInterface {
    states: Vec<PlaceState>,
    credits: Vec<CitizenCredit>,
}

impl EarthInterface {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(
        &mut self,
        place: &str,
        system: SystemTwin,
        source: SourceKind,
        value: f64,
        uncertainty: f64,
        unit: &str,
    ) -> Result<(), TwinError> {
        let observation = Observation::admit(system, source, value, Some(uncertainty), unit)?;
        self.states.push(PlaceState {
            place: place.into(),
            observation,
            paywall: false,
        });
        Ok(())
    }

    pub fn dashboard(&self, place: &str) -> Result<&PlaceState, TwinError> {
        self.get(place)
    }

    pub fn scientist_api(&self, place: &str) -> Result<&PlaceState, TwinError> {
        self.get(place)
    }

    pub fn ask(&self, place: &str, locale: &str) -> Result<String, TwinError> {
        let state = self.get(place)?;
        if locale.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(format!(
            "locale={} place={} value={} uncertainty={}",
            locale, state.place, state.observation.value, state.observation.uncertainty
        ))
    }

    pub fn basic_access_is_free(&self, place: &str) -> Result<bool, TwinError> {
        Ok(!self.get(place)?.paywall)
    }

    pub fn contribute(
        &mut self,
        place: &str,
        license: &str,
        attribution: &str,
    ) -> Result<CitizenCredit, TwinError> {
        if license.trim().is_empty() || attribution.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        let credit = CitizenCredit {
            place: place.into(),
            license: license.into(),
            attribution: attribution.into(),
        };
        self.credits.push(credit.clone());
        Ok(credit)
    }

    pub fn access_tiers() -> [AccessTier; 3] {
        [
            AccessTier::BasicFree,
            AccessTier::Scientist,
            AccessTier::Minister,
        ]
    }

    fn get(&self, place: &str) -> Result<&PlaceState, TwinError> {
        self.states
            .iter()
            .find(|s| s.place == place)
            .ok_or(TwinError::UnlabeledPoint)
    }
}
