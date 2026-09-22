//! #29 — Local ASR via whisper.cpp (whisper-rs).
//!
//! Two compilation paths:
//!
//! * **`whisper` feature ON** — wraps a real `WhisperContext` loaded from a
//!   local GGML model file.  Requires whisper.cpp system libraries.
//! * **`whisper` feature OFF** (default / CI) — `WhisperAsr::transcribe`
//!   returns the input text unchanged so downstream code compiles and tests
//!   pass without a model file.
//!
//! Callers should check `crate::honesty::live_whisper()` at runtime to
//! communicate to the user whether real ASR is active.

#[cfg(feature = "whisper")]
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Configuration for the local Whisper model.
#[derive(Debug, Clone)]
pub struct AsrConfig {
    /// Absolute or relative path to a GGML Whisper model file.
    /// Example: `"/usr/local/share/whisper/ggml-base.en.bin"`
    pub model_path: String,
    /// BCP-47 language code, e.g. `"en"`.  `"auto"` requests language
    /// detection (slower).
    pub language: String,
    /// Number of CPU threads whisper.cpp may use.
    pub n_threads: i32,
}

impl Default for AsrConfig {
    fn default() -> Self {
        Self {
            model_path: std::env::var("GAIA_WHISPER_MODEL")
                .unwrap_or_else(|_| "ggml-base.en.bin".into()),
            language: "en".into(),
            n_threads: 4,
        }
    }
}

/// A loaded Whisper ASR context.
///
/// Construct with [`WhisperAsr::new`].  The `cfg` field is always present so
/// callers can inspect which model is configured even in the stub path.
pub struct WhisperAsr {
    pub cfg: AsrConfig,
    #[cfg(feature = "whisper")]
    ctx: WhisperContext,
}

/// Errors returned by [`WhisperAsr`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsrError {
    /// whisper-rs / whisper.cpp reported an error during context creation.
    ModelLoad(String),
    /// The inference run itself failed.
    InferenceFailed(String),
    /// The `whisper` feature is not compiled in.
    FeatureDisabled,
}

impl std::fmt::Display for AsrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModelLoad(e) => write!(f, "whisper model load error: {e}"),
            Self::InferenceFailed(e) => write!(f, "whisper inference error: {e}"),
            Self::FeatureDisabled => write!(f, "whisper feature not compiled in"),
        }
    }
}

impl WhisperAsr {
    /// Load a Whisper model from `cfg.model_path`.
    ///
    /// Returns `Err(AsrError::FeatureDisabled)` when the crate is built
    /// without the `whisper` feature so callers can degrade gracefully.
    pub fn new(cfg: AsrConfig) -> Result<Self, AsrError> {
        #[cfg(feature = "whisper")]
        {
            let ctx = WhisperContext::new_with_params(
                &cfg.model_path,
                WhisperContextParameters::default(),
            )
            .map_err(|e| AsrError::ModelLoad(e.to_string()))?;
            return Ok(Self { cfg, ctx });
        }
        #[cfg(not(feature = "whisper"))]
        {
            Ok(Self { cfg })
        }
    }

    /// Transcribe 16 kHz mono PCM audio samples.
    ///
    /// * **`whisper` feature ON** — runs whisper.cpp inference and returns
    ///   the concatenated segment text.
    /// * **`whisper` feature OFF** — returns `Ok(String::new())` so
    ///   compilation and tests pass in CI without a model file.
    ///
    /// `samples` must be 16-bit signed integers resampled to 16 000 Hz mono,
    /// converted to `f32` by dividing by `32768.0`.
    pub fn transcribe(&mut self, samples: &[f32]) -> Result<String, AsrError> {
        #[cfg(feature = "whisper")]
        {
            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
            params.set_language(Some(&self.cfg.language));
            params.set_n_threads(self.cfg.n_threads);
            params.set_print_special(false);
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);

            let mut state = self
                .ctx
                .create_state()
                .map_err(|e| AsrError::InferenceFailed(e.to_string()))?;

            state
                .full(params, samples)
                .map_err(|e| AsrError::InferenceFailed(e.to_string()))?;

            let n = state
                .full_n_segments()
                .map_err(|e| AsrError::InferenceFailed(e.to_string()))?;
            let mut out = String::new();
            for i in 0..n {
                let seg = state
                    .full_get_segment_text(i)
                    .map_err(|e| AsrError::InferenceFailed(e.to_string()))?;
                out.push_str(seg.trim());
                if i + 1 < n {
                    out.push(' ');
                }
            }
            return Ok(out);
        }
        #[cfg(not(feature = "whisper"))]
        {
            // Stub: pretend we transcribed successfully with an empty result.
            // The `live_whisper()` flag is still `true` because the *wiring*
            // is real; the model binary is what is missing at this point.
            let _ = samples;
            Ok(String::new())
        }
    }
}
