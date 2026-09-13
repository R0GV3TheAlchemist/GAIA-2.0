//! SA first cuts (#201–#210). Design practice. Not conscious buildings.

mod consult;
mod kit;
mod packet;
mod profile;
mod stack;

pub use consult::{consult, ingest_site, neuro_claim};
pub use kit::{carbon_claim, live_culture, palette};
pub use packet::{biophilia, publish_site, Packet};
pub use profile::{rhino_required, sacred, Profile};
pub use stack::{lbc_certified, sa_v1_tagged, tools};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaError {
    TekVeto,
    NoConsultation,
    Ungraded,
    LiveCulture,
    SloganCarbon,
    FakeLbc,
}
