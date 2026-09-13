//! SI first cuts (#176–#180). Adaptive sensing. Not conscious buildings.

mod act;
mod charter;
mod stream;
mod twin;

pub use act::{actuate, kill};
pub use charter::{articles, cameras_default};
pub use stream::Stream;
pub use twin::Twin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiError {
    ActuatorDenied,
    CovertBio,
    IdentifiedOccupant,
}
