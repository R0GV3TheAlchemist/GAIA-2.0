//! Bootloader — Stage 0 (firmware hand-off) and Stage 1 (memory-map ingestion).

pub mod stage0;
pub mod stage1;

/// Summary produced by the bootloader phases and passed to later boot stages.
#[derive(Debug, Clone)]
pub struct BootManifest {
    /// Firmware/bootloader identifier string.
    pub firmware_id: String,
    /// Cold-start timestamp in nanoseconds since an arbitrary epoch.
    pub cold_start_ns: u64,
    /// Memory regions handed off from firmware.
    pub memory_regions: Vec<MemoryRegion>,
    /// Total detected RAM in bytes.
    pub total_ram_bytes: u64,
}

/// A contiguous physical memory region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRegion {
    pub base_addr: u64,
    pub length_bytes: u64,
    pub kind: MemoryKind,
}

/// Classification of a physical memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryKind {
    Usable,
    Reserved,
    AcpiReclaimable,
    Mmio,
}

/// Run Stage 0 then Stage 1 and return a consolidated `BootManifest`.
pub fn run() -> BootManifest {
    let fw = stage0::handoff();
    let mem = stage1::ingest_memory_map();
    let total_ram_bytes = mem
        .iter()
        .filter(|r| r.kind == MemoryKind::Usable)
        .map(|r| r.length_bytes)
        .sum();
    BootManifest {
        firmware_id: fw.firmware_id,
        cold_start_ns: fw.cold_start_ns,
        memory_regions: mem,
        total_ram_bytes,
    }
}
