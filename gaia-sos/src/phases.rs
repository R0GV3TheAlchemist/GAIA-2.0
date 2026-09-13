//! Honest status for Super OS #1–#10. Do not treat closed Phase 0/1 as a product.

/// Phase epics closed on GitHub. Does not mean the Super OS ships.
pub fn phase_closed(n: u32) -> bool {
    matches!(n, 2 | 3)
}

pub fn live_llm() -> bool {
    false
}

pub fn live_marketplace() -> bool {
    false
}

pub fn flutter_ui() -> bool {
    false
}

pub fn profiles() -> [&'static str; 4] {
    ["embedded", "desktop", "cloud", "hpc"]
}
