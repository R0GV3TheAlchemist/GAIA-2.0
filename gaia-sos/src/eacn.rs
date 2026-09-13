//! #198 pull-based discovery. No god coordinator.

use crate::SosError;

pub fn discover(from_intent: bool) -> Result<&'static str, SosError> {
    if from_intent {
        Ok("intent-discover-capsule-result")
    } else {
        Ok("none")
    }
}

pub fn god_coordinator() -> Result<(), SosError> {
    Err(SosError::GodCoordinator)
}
