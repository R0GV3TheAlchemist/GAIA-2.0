//! Honest flags for GAIAN #57–#75.
//!
//! ## `live_whisper`
//!
//! Returns `true` only when the crate is compiled with
//! `--features whisper`, which links `whisper-rs` and activates the real
//! `WhisperAsr` inference path in `asr.rs`.
//!
//! In the **default build** (no feature flag) this returns `false` because:
//! * no GGML model file is present in CI,
//! * the `asr::WhisperAsr::transcribe` path is the compile-time stub, and
//! * the honesty contract test asserts `!live_whisper()` by default.
//!
//! To activate real Whisper inference locally:
//! ```bash
//! export GAIA_WHISPER_MODEL=/path/to/ggml-base.en.bin
//! cargo test --manifest-path gaia-gaian/Cargo.toml --features whisper
//! ```

pub fn live_ollama() -> bool { false }
pub fn live_flutter() -> bool { false }
/// `true` only when compiled with `--features whisper` (real whisper.cpp linked).
pub fn live_whisper() -> bool { cfg!(feature = "whisper") }
pub fn live_vrm_runtime() -> bool { false }
pub fn gaian_v1_tagged() -> bool { false }
pub fn medical_product() -> bool { false }
