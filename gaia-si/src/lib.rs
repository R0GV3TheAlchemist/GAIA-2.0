//! SI first cuts (#176–#189). Adaptive sensing. Not conscious buildings.

mod act;
mod advise;
mod bio;
mod charter;
mod prefs;
mod stream;
mod twin;

pub use act::{actuate, kill, life_safety};
pub use advise::{advise, si_v1_tagged};
pub use bio::{nature_first, pathogen_protocol, review_field};
pub use charter::{admit_occupancy, articles, cameras_default};
pub use prefs::{hvac_write, Prefs};
pub use stream::{admit, face_field, Stream};
pub use twin::{aggregate, Twin};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiError {
    ActuatorDenied,
    CovertBio,
    IdentifiedOccupant,
    NoPurpose,
    FaceUnsupported,
    PathogenProtocol,
    NeedsReview,
    NeedsTicket,
    ConsciousMarketing,
    BiometricDefault,
}
