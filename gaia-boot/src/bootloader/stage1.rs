//! Stage 1 — Memory-map ingestion.
//!
//! Reads the physical memory map provided by firmware (UEFI MemoryMap,
//! multiboot2 mmap tag, or device-tree memory nodes).  The stub returns
//! a representative map suitable for Tier-2 (desktop/server) hardware.

use super::{MemoryKind, MemoryRegion};

/// Ingest the firmware memory map and return a Vec of `MemoryRegion`.
pub fn ingest_memory_map() -> Vec<MemoryRegion> {
    eprintln!("[gaia-boot] phase=1 status=memory_map_ingestion");

    // Stub: representative Tier-2 physical memory layout.
    //   0x0000_0000 –  0x0009_FFFF   640 KiB   conventional RAM (usable)
    //   0x000A_0000 –  0x000F_FFFF   384 KiB   VGA/ROM (reserved)
    //   0x0010_0000 –  0x7FFF_FFFF  ~2 GiB     extended RAM (usable)
    //   0x8000_0000 –  0x8FFF_FFFF  256 MiB    MMIO window
    vec![
        MemoryRegion {
            base_addr: 0x0000_0000,
            length_bytes: 640 * 1024,
            kind: MemoryKind::Usable,
        },
        MemoryRegion {
            base_addr: 0x000A_0000,
            length_bytes: 384 * 1024,
            kind: MemoryKind::Reserved,
        },
        MemoryRegion {
            base_addr: 0x0010_0000,
            length_bytes: 0x7FF0_0000,
            kind: MemoryKind::Usable,
        },
        MemoryRegion {
            base_addr: 0x8000_0000,
            length_bytes: 256 * 1024 * 1024,
            kind: MemoryKind::Mmio,
        },
    ]
}
