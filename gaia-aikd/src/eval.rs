//! #106 calibration notes. Not lm-eval-harness. Not AIKD v1.0.

use crate::meta::aikd_v1_tagged;

pub fn measured_vs_published() -> [&'static str; 2] {
    [
        "published: reference-only",
        "gaia_measured: none-yet",
    ]
}

pub fn overconfident_fixture_dropped(phase1_flagged: bool) -> bool {
    phase1_flagged
}

pub fn release_ready() -> bool {
    !aikd_v1_tagged()
}
