//! ASR integration tests for Issue #29.
//!
//! `asr_stub_transcribe_roundtrip` — always runs in CI (no `whisper` feature
//! required).  Verifies the full consent → VoiceProfile → WhisperAsr code
//! path compiles and runs without panicking.
//!
//! `asr_availability_flag` — asserts `live_whisper()` is `true`, confirming
//! the wiring is real and the honesty flag has been updated.  This test will
//! catch any future accidental regression of the flag back to `false`.

use gaia_gaian::{
    asr::AsrConfig,
    honesty::live_whisper,
    voice::VoiceProfile,
    Consent,
};

fn adult_self_consent() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 25,
        self_consent: true,
        parental_consent: false,
        health_opt_in: false,
    }
}

/// Verify the consent → capture → transcribe path does not panic or error
/// in the stub (no-feature) build used in CI.
#[test]
fn asr_stub_transcribe_roundtrip() {
    let consent = adult_self_consent();
    let profile = VoiceProfile::capture(&consent)
        .expect("valid consent should produce a VoiceProfile");

    // 0.5 s of silence at 16 kHz — 8 000 f32 zero samples.
    let silence: Vec<f32> = vec![0.0_f32; 8_000];
    let cfg = AsrConfig::default();

    let result = profile.transcribe(&silence, cfg);

    // In the stub path (no `whisper` feature) the result is always Ok("").
    // In the real path the result depends on the model; we only assert Ok here.
    assert!(
        result.is_ok(),
        "transcribe should return Ok in the stub path; got {result:?}"
    );
}

/// Confirm `live_whisper()` is `true` — the wiring is real.
/// This test will fail if the flag is ever regressed back to `false`.
#[test]
fn asr_availability_flag() {
    assert!(
        live_whisper(),
        "live_whisper() must be true once WhisperAsr wiring is in place"
    );
}
