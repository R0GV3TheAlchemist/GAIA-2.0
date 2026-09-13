//! #177 ethics. Phase 0 has no actuator write path.

pub fn articles() -> [&'static str; 8] {
    [
        "override-always",
        "no-covert-biometrics",
        "purpose-limitation",
        "retention-limits",
        "human-ticket-for-actuators",
        "no-pathogen-engineering",
        "cameras-off-default",
        "not-conscious-buildings",
    ]
}

pub fn cameras_default() -> bool {
    false
}
