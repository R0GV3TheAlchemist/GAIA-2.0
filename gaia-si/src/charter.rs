//! #177/#183 ethics.

use crate::SiError;

pub fn articles() -> [&'static str; 8] {
    [
        "life-serving",
        "transparency",
        "override",
        "equity",
        "privacy",
        "sustainability",
        "resilience",
        "democracy",
    ]
}

pub fn cameras_default() -> bool {
    false
}

pub fn admit_occupancy(class: &str) -> Result<(), SiError> {
    if class.contains("biometric") {
        return Err(SiError::BiometricDefault);
    }
    Ok(())
}
