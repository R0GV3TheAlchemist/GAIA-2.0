//! Artificial Twin of Earth (#33 / #38).
//! Observation contract plus Human-Earth surfaces over one state.
//! Not Cesium, not UNEP hooks, not a multilingual agent.

mod iface;

pub use iface::{AccessTier, CitizenCredit, EarthInterface, PlaceState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Measured,
    Synthetic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemTwin {
    Atmosphere,
    Ocean,
    Land,
    Biosphere,
    Cryosphere,
    Lithosphere,
    Anthroposphere,
    Technosphere,
    Magnetosphere,
}

impl SystemTwin {
    pub fn all() -> [SystemTwin; 9] {
        [
            Self::Atmosphere,
            Self::Ocean,
            Self::Land,
            Self::Biosphere,
            Self::Cryosphere,
            Self::Lithosphere,
            Self::Anthroposphere,
            Self::Technosphere,
            Self::Magnetosphere,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    pub system: SystemTwin,
    pub source: SourceKind,
    pub value: f64,
    pub uncertainty: f64,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwinError {
    MissingUncertainty,
    UnlabeledPoint,
    WeaponizedUse,
}

impl std::fmt::Display for TwinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingUncertainty => write!(f, "uncertainty is mandatory"),
            Self::UnlabeledPoint => write!(f, "point must be measured or labeled synthetic"),
            Self::WeaponizedUse => write!(f, "weaponization is refused"),
        }
    }
}

impl Observation {
    pub fn admit(
        system: SystemTwin,
        source: SourceKind,
        value: f64,
        uncertainty: Option<f64>,
        unit: &str,
    ) -> Result<Self, TwinError> {
        let uncertainty = uncertainty.ok_or(TwinError::MissingUncertainty)?;
        if uncertainty < 0.0 {
            return Err(TwinError::MissingUncertainty);
        }
        if unit.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(Self {
            system,
            source,
            value,
            uncertainty,
            unit: unit.into(),
        })
    }
}

pub fn allow_purpose(purpose: &str) -> Result<(), TwinError> {
    let lower = purpose.to_ascii_lowercase();
    if lower.contains("weapon")
        || lower.contains("target individual")
        || lower.contains("surveillance of person")
    {
        return Err(TwinError::WeaponizedUse);
    }
    Ok(())
}
