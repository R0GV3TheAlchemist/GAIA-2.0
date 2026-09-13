//! #206/#208/#212 open tools. No fake LBC. Place knowledge sealed.

use crate::SaError;

pub fn tools() -> [&'static str; 4] {
    ["freecad", "blenderbim", "energyplus", "ladybug"]
}

pub fn lbc_certified() -> bool {
    false
}

pub fn sa_v1_tagged() -> bool {
    false
}

pub fn scrape_country() -> Result<(), SaError> {
    Err(SaError::SealedPlace)
}
