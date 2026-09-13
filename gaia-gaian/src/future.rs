//! #74 Future Self sketches. Simulated, not fate. Owner mesh only.

use crate::{Consent, GaianError};

#[derive(Debug, Clone, PartialEq)]
pub struct Sketch {
    pub label: String,
    pub horizon_years: u8,
    pub low: f64,
    pub high: f64,
    pub simulated: bool,
}

impl Sketch {
    pub fn of_owner(consent: &Consent, label: &str, years: u8) -> Result<Self, GaianError> {
        if !consent.subject_is_self {
            return Err(GaianError::NotSelf);
        }
        if consent.age_years < 16 {
            return Err(GaianError::Under16);
        }
        if ![5, 10, 20, 30].contains(&years) {
            return Err(GaianError::NoConsent);
        }
        Ok(Self {
            label: label.into(),
            horizon_years: years,
            low: 0.0,
            high: 1.0,
            simulated: true,
        })
    }
}

pub fn compare(a: &Sketch, b: &Sketch) -> Result<[&str; 2], GaianError> {
    if !a.simulated || !b.simulated {
        return Err(GaianError::NoConsent);
    }
    Ok([&a.label, &b.label])
}

pub fn age_other_person() -> Result<(), GaianError> {
    Err(GaianError::NotSelf)
}
