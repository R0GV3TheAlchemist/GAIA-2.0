use gaia_hspd::{end_session, join_group, Session};

#[test]
fn devices_stop_and_group_is_unlistable() {
    let mut s = Session::start();
    end_session(&mut s);
    assert!(!s.live);
    let g = join_group(true).unwrap();
    assert!(!g.listed);
    assert!(join_group(false).is_err());
}
