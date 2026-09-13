//! GAIAN first cuts (#57–#75). Consent and local stubs.
//! Not Ollama, rembg, Flutter, WebGPU, or GAIAN v1.0.

mod agency;
mod agent;
mod capture;
mod channel;
mod cognition;
mod digital;
mod future;
mod health;
mod living;
mod package;
mod sensors;
mod vault;
mod voice;
mod wardrobe;

pub use agency::{ScopedAgent, Scope};
pub use agent::{gaian_release_checklist, Agent, AgentAct, Grant};
pub use capture::{equity_eval_labels, server_face_store, CaptureSession};
pub use channel::{Channel, ChannelCap, Envelope};
pub use cognition::{g2g_send, DigitalMe, MemoryTier, Persona, PersonalMemory};
pub use digital::{DigitalVault, KnowKind};
pub use future::{age_other_person, compare, Sketch};
pub use health::{age_progress_own, future_self, HealthMetric, HealthTwin};
pub use living::{animate_third_party_face, SignedSession};
pub use package::{blueprint_example_valid, PersonaPackage};
pub use sensors::HealthModule;
pub use vault::{AuditEntry, Vault};
pub use voice::{capture_app_allows, Appearance, VoiceProfile};
pub use wardrobe::{cultural_preset_warning, load_custom, OutfitCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleKind {
    Photo,
    Voice,
    Health,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsentScope {
    Face,
    Body,
    Voice,
    Health,
    Memory,
    Agent,
    EarthTwinShare,
}

impl ConsentScope {
    pub fn all() -> [ConsentScope; 7] {
        [
            Self::Face,
            Self::Body,
            Self::Voice,
            Self::Health,
            Self::Memory,
            Self::Agent,
            Self::EarthTwinShare,
        ]
    }
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
    BehavioralLearningBlocked,
    HealthDefaultOff,
    ExportForbidden,
    NotMedicalAdvice,
    Unsigned,
    GrantRequired,
    Revoked,
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
            Self::BehavioralLearningBlocked => write!(f, "child path cannot enable behavioral learning"),
            Self::HealthDefaultOff => write!(f, "health twin defaults off"),
            Self::ExportForbidden => write!(f, "insurer/employer export is forbidden"),
            Self::NotMedicalAdvice => write!(f, "future self is advisory, not medical advice"),
            Self::Unsigned => write!(f, "unsigned GAIAN action rejected"),
            Self::GrantRequired => write!(f, "scoped grant required"),
            Self::Revoked => write!(f, "agent revoked"),
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

pub fn constitution_principles() -> [&'static str; 10] {
    [
        "consent",
        "local-default",
        "biometric sovereignty",
        "user-held keys",
        "deletion",
        "transparency",
        "purpose limitation",
        "non-weaponization",
        "equity",
        "child protection",
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
