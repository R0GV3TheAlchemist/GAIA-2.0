//! #208 GAIAN architecture fields. Sacred is opt-in.

#[derive(Debug, Default)]
pub struct Profile {
    pub sacred: bool,
}

pub fn sacred(declared: bool) -> bool {
    declared
}

pub fn rhino_required() -> bool {
    false
}
