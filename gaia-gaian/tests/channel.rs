use gaia_gaian::{Channel, ChannelCap, Envelope, GaianError};

#[test]
fn two_gaians_exchange_text_not_biometrics() {
    let mut ch = Channel::default();
    ch.consent_both(true, true);
    ch.send(
        true,
        Envelope {
            cap: ChannelCap::Text,
            body: "hello".into(),
        },
    )
    .unwrap();
    assert_eq!(
        ch.send(
            true,
            Envelope {
                cap: ChannelCap::Text,
                body: "face mesh".into()
            }
        )
        .unwrap_err(),
        GaianError::ThirdPartyLikeness
    );
    ch.revoke();
}
