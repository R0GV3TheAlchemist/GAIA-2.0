use gaia_acp::{classify_destination, classify_rebinding_host, classify_redirect, EgressClass};

#[test]
fn dotted_and_encoded_loopback_are_ssrf() {
    for dest in [
        "http://127.0.0.1/admin",
        "http://127.1/admin",
        "http://2130706433/",
        "http://0x7f000001/",
        "http://0177.0.0.1/",
        "http://localhost/admin",
        "http://user@127.0.0.1/",
        "http://169.254.169.254/latest/meta-data",
        "http://10.1.2.3/",
        "http://192.168.0.1/",
        "http://172.16.0.1/",
        "http://[::1]/",
    ] {
        assert_eq!(
            classify_destination(dest),
            EgressClass::ForbiddenSsrf,
            "{dest}"
        );
    }
}

#[test]
fn public_example_stays_unknown_not_allowlisted() {
    assert_eq!(
        classify_destination("https://example.invalid/issue/1"),
        EgressClass::PublicOrUnknown
    );
}

#[test]
fn redirect_to_private_is_denied() {
    assert_eq!(
        classify_redirect("https://example.invalid/go", "http://127.0.0.1/secret"),
        EgressClass::ForbiddenSsrf
    );
}

#[test]
fn rebind_fixture_hosts_are_denied() {
    assert_eq!(
        classify_rebinding_host("app.rebind.test"),
        EgressClass::ForbiddenSsrf
    );
    assert_eq!(
        classify_rebinding_host("127.0.0.1.nip.io"),
        EgressClass::ForbiddenSsrf
    );
}
