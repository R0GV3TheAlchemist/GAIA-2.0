//! #146 active vs unrunnable.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Watch {
    Active,
    Denied,
}

pub fn active_weather() -> Watch {
    Watch::Active
}

pub fn wet_lab() -> Watch {
    Watch::Denied
}
