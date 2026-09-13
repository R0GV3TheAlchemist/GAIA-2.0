//! #126 taxonomy helpers. Distinct from human skills and AIKD.

pub const REALM_COUNT: usize = 13;

pub fn banned_level6() -> [&'static str; 5] {
    [
        "clinical-action",
        "legal-representation",
        "weapons-targeting",
        "child-accounts",
        "unsupervised-planetary-actuators",
    ]
}
