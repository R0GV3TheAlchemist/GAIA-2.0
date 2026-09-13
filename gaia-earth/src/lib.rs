//! Artificial Twin of Earth (#33–#41).
//! Observation contract, commons catalog, and in-process lake zones.
//! Not MinIO, Iceberg, Cesium, or a live STAC portal.

mod commons;
mod iface;
mod lake;
mod memory;
mod nervous;
mod simulate;

pub use commons::{Collection, Commons, Domain, QualityTier};
pub use iface::{AccessTier, CitizenCredit, EarthInterface, PlaceState};
pub use lake::{Lake, LakeRow, LakeZone};
pub use memory::{CubeSet, ModelOutput, PlaceTime, PlanetaryMemCube, PlanetaryMemory};
pub use nervous::{FeedKind, NervousFabric, QcTier, Sample};
pub use simulate::{Distribution, OutcomeKind, ScenarioEngine, ScenarioRun, SimError, SimMode};

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
    UnknownCollection,
    SparseRegion,
    ImmutableRaw,
}

impl std::fmt::Display for TwinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingUncertainty => write!(f, "uncertainty is mandatory"),
            Self::UnlabeledPoint => write!(f, "point must be measured or labeled synthetic"),
            Self::WeaponizedUse => write!(f, "weaponization is refused"),
            Self::UnknownCollection => write!(f, "unknown collection"),
            Self::SparseRegion => write!(f, "sparse region flagged; value not invented"),
            Self::ImmutableRaw => write!(f, "raw zone is append-only"),
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
