use gaia_gaian::{
    ambient_listen, articles, be_dead_parent, child_level, crisis, empathy_copy, engagement_max,
    wipe_without_instrument,
};

#[test]
fn posthumous_without_consent_and_engagement_max_fail() {
    assert_eq!(articles().len(), 8);
    assert!(empathy_copy().contains("not sentient"));
    assert!(be_dead_parent(false).is_err());
    assert_eq!(wipe_without_instrument(), "wipe");
    assert!(engagement_max().is_err());
    assert!(crisis().contains("no DIY therapy"));
    assert!(child_level(2).is_err());
    child_level(1).unwrap();
    assert!(ambient_listen(true).is_err());
}
