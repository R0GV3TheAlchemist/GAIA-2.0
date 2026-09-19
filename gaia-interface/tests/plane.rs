use gaia_interface::*;

#[test]
fn free_user_can_always_build() {
    let poor = Developer::free();
    let rich = Developer {
        declared_revenue: 50_000,
        upgraded: false,
    };
    assert!(poor.can_build_locally());
    assert!(rich.can_build_locally());
    assert!(!Developer::paid_amplification());
}

#[test]
fn upgrade_is_offer_not_lockout() {
    let mut d = Developer::free();
    assert!(!d.upgrade_offered());
    assert!(!d.accept_upgrade(1_000));
    d.declared_revenue = UPGRADE_THRESHOLD_UNITS;
    assert!(d.upgrade_offered());
    assert!(d.accept_upgrade(UPGRADE_PRICE_UNITS));
    assert!(d.upgraded);
    assert!(d.can_build_locally());
}

#[test]
fn commission_listed_on_both_paths() {
    let free = Developer {
        declared_revenue: 5_000,
        upgraded: false,
    };
    let paid = Developer {
        declared_revenue: 5_000,
        upgraded: true,
    };
    assert_eq!(free.listed_commission(), 250);
    assert_eq!(paid.listed_commission(), 250);
}
