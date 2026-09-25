//! `gaia-sos` — Safety, Oversight and Soul for the GAIA 2.0 system.
//!
//! This crate provides runtime verification of GAIAN's Five Soul Axioms,
//! distress detection, resonance dissonance flagging, and safety overrides.
//!
//! # Key Modules
//!
//! - [`soul`] — `SoulIntegrity` trait, `StandardSoulChecker`, `ProposedAction`,
//!   `SoulAxiom`, `AxiomViolation`, `SoulCheckResult`

pub mod soul;

pub use soul::{
    ActionBuildError, AxiomViolation, ProposedAction, ProposedActionBuilder,
    SoulAxiom, SoulCheckResult, SoulCheckResultExt, SoulIntegrity,
    StandardSoulChecker, AffectiveContextCompat, ResonanceStateCompat,
    soul_check,
};
