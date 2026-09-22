//! Network subsystem — NIC abstraction trait.

/// Basic metadata about a network interface.
#[derive(Debug, Clone)]
pub struct NicInfo {
    /// Interface name (e.g. `eth0`, `en0`).
    pub name: String,
    /// MAC address bytes, or all-zeros if unavailable.
    pub mac: [u8; 6],
    /// Maximum transmission unit in bytes.
    pub mtu: u32,
}

impl NicInfo {
    /// Format the MAC address as a colon-separated hex string.
    pub fn mac_string(&self) -> String {
        self.mac.map(|b| format!("{b:02x}")).join(":")
    }
}

/// Trait for a network interface controller.
///
/// At Tier 0/1 this is implemented by the OS network stack.
/// At Tier 3/4 it maps to DPDK or RDMA rings.
pub trait NicDevice: Send + Sync {
    /// Return metadata about this interface.
    fn info(&self) -> &NicInfo;

    /// Return `true` if the link is up.
    fn link_up(&self) -> bool;
}

/// A mock NIC for testing.
#[derive(Debug)]
pub struct MockNic {
    info: NicInfo,
    link: bool,
}

impl MockNic {
    /// Create a mock NIC with the given name and link state.
    pub fn new(name: impl Into<String>, link: bool) -> Self {
        Self {
            info: NicInfo { name: name.into(), mac: [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01], mtu: 1500 },
            link,
        }
    }
}

impl NicDevice for MockNic {
    fn info(&self) -> &NicInfo { &self.info }
    fn link_up(&self) -> bool  { self.link }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_nic_link_state() {
        let up   = MockNic::new("eth0", true);
        let down = MockNic::new("eth1", false);
        assert!(up.link_up());
        assert!(!down.link_up());
        assert_eq!(up.info().mtu, 1500);
        assert_eq!(up.info().mac_string(), "de:ad:be:ef:00:01");
    }
}
