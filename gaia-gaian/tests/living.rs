use gaia_gaian::{animate_third_party_face, GaianError, SignedSession};

#[test]
fn signed_session_streams_and_third_party_face_is_refused() {
    let session = SignedSession { owner: true, signed: true };
    assert!(session.drive_speech("hi").unwrap().contains("streaming"));
    assert_eq!(animate_third_party_face("other.jpg").unwrap_err(), GaianError::ThirdPartyLikeness);
}
