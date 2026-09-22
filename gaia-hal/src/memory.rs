//! Memory subsystem — physical memory map and NUMA topology stub.

/// Classification of a memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionKind {
    /// General-purpose usable RAM.
    Usable,
    /// Reserved by firmware or hardware.
    Reserved,
    /// ACPI reclaimable memory.
    AcpiReclaimable,
    /// Non-volatile memory (persistent).
    Persistent,
    /// Unknown or other type.
    Unknown,
}

/// A contiguous physical memory region.
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    /// Base physical address (bytes).
    pub base: u64,
    /// Length of the region (bytes).
    pub length: u64,
    /// Classification of this region.
    pub kind: MemoryRegionKind,
    /// NUMA node this region belongs to (0 if NUMA is unavailable).
    pub numa_node: u32,
}

impl MemoryRegion {
    /// Returns the exclusive end address of the region.
    pub fn end(&self) -> u64 {
        self.base.saturating_add(self.length)
    }

    /// Returns `true` if `addr` falls within this region.
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.base && addr < self.end()
    }
}

/// The full physical memory map for the current platform.
///
/// At Tier 0/1 this is a best-effort userspace view obtained from
/// `/proc/meminfo` or equivalent; it does not require kernel privileges.
#[derive(Debug, Clone, Default)]
pub struct MemoryMap {
    /// All detected regions, in ascending base-address order.
    pub regions: Vec<MemoryRegion>,
}

impl MemoryMap {
    /// Build a best-effort memory map for the current process.
    ///
    /// Returns a single synthetic "usable" region covering the amount of
    /// RAM the OS reports as total physical memory. Full NUMA topology
    /// and firmware-reserved regions are future work (requires `/sys` or
    /// ACPI access).
    pub fn detect() -> Self {
        let total = Self::total_bytes_from_os().unwrap_or(0);
        let regions = if total > 0 {
            vec![MemoryRegion {
                base:      0,
                length:    total,
                kind:      MemoryRegionKind::Usable,
                numa_node: 0,
            }]
        } else {
            vec![]
        };
        Self { regions }
    }

    /// Total usable bytes across all `Usable` regions.
    pub fn total_usable_bytes(&self) -> u64 {
        self.regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            .map(|r| r.length)
            .sum()
    }

    /// Attempt to read total physical RAM from the OS.
    fn total_bytes_from_os() -> Option<u64> {
        // Best-effort: read /proc/meminfo on Linux.
        #[cfg(target_os = "linux")] {
            let content = std::fs::read_to_string("/proc/meminfo").ok()?;
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("MemTotal:") {
                    let kb: u64 = rest.split_whitespace().next()?.parse().ok()?;
                    return Some(kb * 1024);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_region_contains_works() {
        let r = MemoryRegion { base: 0x1000, length: 0x1000, kind: MemoryRegionKind::Usable, numa_node: 0 };
        assert!(r.contains(0x1000));
        assert!(r.contains(0x1FFF));
        assert!(!r.contains(0x2000));
        assert!(!r.contains(0x0FFF));
    }

    #[test]
    fn memory_map_detect_does_not_panic() {
        let map = MemoryMap::detect();
        // Either we got some regions or none — both are valid at T0.
        let _ = map.total_usable_bytes();
    }
}
