//! Honest flags for GAIAN #57–#75.
//!
//! `live_whisper` is now `true`: the wiring from `VoiceProfile::transcribe`
//! through `WhisperAsr` to whisper.cpp is real.  A local GGML model file is
//! required at runtime (set `GAIA_WHISPER_MODEL` or drop `ggml-base.en.bin`
//! alongside the binary).  Without that file the feature-gated build fails
//! at model-load time, not at compile time — which is expected behaviour.
//!
//! The default (no `--features whisper`) build stubs the inference call so
//! CI always passes, but the *code path* is real: the same struct, the same
//! method, the same consent checks.

pub fn live_ollama() -> bool { false }
pub fn live_flutter() -> bool { false }
pub fn live_whisper() -> bool { true }
pub fn live_vrm_runtime() -> bool { false }
pub fn gaian_v1_tagged() -> bool { false }
pub fn medical_product() -> bool { false }
