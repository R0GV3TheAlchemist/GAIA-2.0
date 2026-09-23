//! Honest flags for AIKD. Not a practice license.
//!
//! These three sentinel functions are intentionally `pub` so that future
//! integration tests and the honesty module in gaia-gaian can read them.
//! The `dead_code` lint is suppressed at module level because the functions
//! are referenced exclusively through the crate's public API surface, not
//! from within this crate.

#[allow(dead_code)]
pub fn practice_license() -> bool { false }
#[allow(dead_code)]
pub fn published_is_measured() -> bool { false }
#[allow(dead_code)]
pub fn aikd_v1_tagged() -> bool { false }
