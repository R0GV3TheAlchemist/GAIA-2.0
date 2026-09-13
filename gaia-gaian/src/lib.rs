//! GAIAN first cuts through #75. Consent and local stubs.
//! Not Ollama, wearables, or live Earth Twin I/O.

mod agency;
mod channel;
mod digital;
mod future;
mod sensors;

pub use agency::{ScopedAgent, Scope};
pub use channel::{Channel, ChannelCap, Envelope};
pub use digital::{DigitalVault, KnowKind};
pub use future::{age_other_person, compare, Sketch};
pub use sensors::HealthModule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleKind {
    Photo,
    Voice,
    Health,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Consent {
    pub subject_is_self: bool,
    pub age_years: u8,
    pub self_consent: bool,
    pub parental_consent: bool,
    pub health_opt_in: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GaianError {
    Under16,
    NeedsParentalConsent,
    NoConsent,
    NotSelf,
    HealthNotOptIn,
    ThirdPartyLikeness,
    Unsigned,
}

impl std::fmt::Display for GaianError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Under16 => write!(f, "under-16 biometric ingest is refused"),
            Self::NeedsParentalConsent => write!(f, "16-17 requires parental consent"),
            Self::NoConsent => write!(f, "consent record required"),
            Self::NotSelf => write!(f, "reconstruction is self-only"),
            Self::HealthNotOptIn => write!(f, "health is opt-in only"),
            Self::ThirdPartyLikeness => write!(f, "no third-party likeness"),
            Self::Unsigned => write!(f, "unsigned GAIAN action rejected"),
        }
    }
}

pub fn principles() -> [&'static str; 10] {
    [
        "consent first",
        "local default",
        "user-held keys",
        "verifiable deletion",
        "no third-party likeness",
        "no child behavioral profiling",
        "health opt-in only",
        "age-gate",
        "self-only reconstruction",
        "Earth Twin I/O needs consent",
    ]
}

fn allow_ingest(consent: &Consent, kind: SampleKind) -> Result<(), GaianError> {
    if consent.age_years < 16 {
        return Err(GaianError::Under16);
    }
    if consent.age_years < 18 && !consent.parental_consent {
        return Err(GaianError::NeedsParentalConsent);
    }
    if !consent.self_consent {
        return Err(GaianError::NoConsent);
    }
    if !consent.subject_is_self {
        return Err(GaianError::NotSelf);
    }
    if kind == SampleKind::Health && !consent.health_opt_in {
        return Err(GaianError::HealthNotOptIn);
    }
    Ok(())
}

pub fn create_self(consent: &Consent, kind: SampleKind) -> Result<&'static str, GaianError> {
    allow_ingest(consent, kind)?;
    Ok("local-stub-vrm")
}

pub fn animate_owner(consent: &Consent, third_party_face: bool) -> Result<(), GaianError> {
    allow_ingest(consent, SampleKind::Photo)?;
    if third_party_face {
        return Err(GaianError::ThirdPartyLikeness);
    }
    Ok(())
}
