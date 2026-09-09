//! GAIA 2.0 Phase 0 SDK — typed stubs for the v0.1 primitives.

pub mod client;
pub mod error;
pub mod types;

pub use client::GaiaClient;
pub use error::{GaiaError, Result};
pub use types::*;
