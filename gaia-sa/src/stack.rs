//! #206/#208 open tools. Rhino optional. No fake LBC. No v1.0.

pub fn tools() -> [&'static str; 4] {
    ["freecad", "blenderbim", "energyplus", "ladybug"]
}

pub fn lbc_certified() -> bool {
    false
}

pub fn sa_v1_tagged() -> bool {
    false
}
