//! Free SOS developer plane. Build is not a paid privilege.

/// Listed default. Not a live processor.
pub const UPGRADE_THRESHOLD_UNITS: u64 = 5_000;
pub const UPGRADE_PRICE_UNITS: u64 = 1_000;
/// 5% listed commission on declared revenue. Not collected here.
pub const COMMISSION_BPS: u16 = 500;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Developer {
    pub declared_revenue: u64,
    pub upgraded: bool,
}

impl Developer {
    pub fn free() -> Self {
        Self {
            declared_revenue: 0,
            upgraded: false,
        }
    }

    pub fn can_build_locally(&self) -> bool {
        true
    }

    pub fn upgrade_offered(&self) -> bool {
        self.declared_revenue >= UPGRADE_THRESHOLD_UNITS && !self.upgraded
    }

    pub fn accept_upgrade(&mut self, paid: u64) -> bool {
        if !self.upgrade_offered() || paid < UPGRADE_PRICE_UNITS {
            return false;
        }
        self.upgraded = true;
        true
    }

    pub fn listed_commission(&self) -> u64 {
        self.declared_revenue.saturating_mul(COMMISSION_BPS as u64) / 10_000
    }

    pub fn paid_amplification() -> bool {
        false
    }
}
