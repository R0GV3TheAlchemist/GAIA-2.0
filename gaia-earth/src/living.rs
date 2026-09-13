//! #45 living sensors. Contribution records. Not BirdNET, Wildlife Insights, or an eDNA lab.

use crate::TwinError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LivingKind {
    Citizen,
    Acoustic,
    CameraTrap,
    Edna,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LivingRecord {
    pub kind: LivingKind,
    pub place: String,
    pub taxon: String,
    pub license: String,
    pub observer: String,
    pub model_version: Option<String>,
    pub confidence: Option<f64>,
}

impl LivingRecord {
    pub fn contribute(
        kind: LivingKind,
        place: &str,
        taxon: &str,
        license: &str,
        observer: &str,
        model_version: Option<&str>,
        confidence: Option<f64>,
    ) -> Result<Self, TwinError> {
        if place.trim().is_empty() || license.trim().is_empty() || observer.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        match kind {
            LivingKind::Acoustic | LivingKind::CameraTrap => {
                let version = model_version.ok_or(TwinError::UnlabeledPoint)?;
                let confidence = confidence.ok_or(TwinError::MissingUncertainty)?;
                if confidence < 0.0 || confidence > 1.0 {
                    return Err(TwinError::MissingUncertainty);
                }
                Ok(Self {
                    kind,
                    place: place.into(),
                    taxon: taxon.into(),
                    license: license.into(),
                    observer: observer.into(),
                    model_version: Some(version.into()),
                    confidence: Some(confidence),
                })
            }
            LivingKind::Citizen | LivingKind::Edna => Ok(Self {
                kind,
                place: place.into(),
                taxon: taxon.into(),
                license: license.into(),
                observer: observer.into(),
                model_version: None,
                confidence: None,
            }),
        }
    }

    pub fn map_pin(&self) -> String {
        format!(
            "place={} license={} credit={}",
            self.place, self.license, self.observer
        )
    }
}
