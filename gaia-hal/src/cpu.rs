//! CPU subsystem — feature detection, core topology, and architecture identity.

/// Target architecture identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuArch {
    /// x86-64 (AMD64 / Intel 64).
    X86_64,
    /// 64-bit ARM (AArch64).
    Aarch64,
    /// RISC-V 64-bit.
    RiscV64,
    /// Any other architecture.
    Other,
}

impl CpuArch {
    /// Detect the architecture of the compiled binary at runtime.
    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]   { CpuArch::X86_64  }
        #[cfg(target_arch = "aarch64")]  { CpuArch::Aarch64 }
        #[cfg(target_arch = "riscv64")]  { CpuArch::RiscV64 }
        #[cfg(not(any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64",
        )))] { CpuArch::Other }
    }
}

/// SIMD / vector extension flags for the current CPU.
#[derive(Debug, Clone, Default)]
pub struct SimdFlags {
    // x86_64
    /// SSE2 is available (baseline for x86_64).
    pub sse2:  bool,
    /// AVX2 256-bit integer/float vectors.
    pub avx2:  bool,
    /// AVX-512 foundation.
    pub avx512f: bool,
    // aarch64
    /// ARM Advanced SIMD (NEON).
    pub neon:  bool,
    /// ARM Scalable Vector Extension.
    pub sve:   bool,
}

impl SimdFlags {
    /// Detect SIMD capabilities via `std::arch` feature probing.
    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")] {
            Self {
                sse2:    is_x86_feature_detected!("sse2"),
                avx2:    is_x86_feature_detected!("avx2"),
                avx512f: is_x86_feature_detected!("avx512f"),
                neon:    false,
                sve:     false,
            }
        }
        #[cfg(target_arch = "aarch64")] {
            Self {
                sse2:    false,
                avx2:    false,
                avx512f: false,
                neon:    std::arch::is_aarch64_feature_detected!("neon"),
                sve:     std::arch::is_aarch64_feature_detected!("sve"),
            }
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))] {
            Self::default()
        }
    }
}

/// CPU feature snapshot for the current host.
#[derive(Debug, Clone)]
pub struct CpuFeatures {
    /// Detected architecture.
    pub arch: CpuArch,
    /// SIMD flags.
    pub simd: SimdFlags,
}

/// Full CPU information including topology.
#[derive(Debug, Clone)]
pub struct CpuInfo {
    /// Feature flags.
    pub features: CpuFeatures,
    /// Number of logical CPUs available to this process.
    pub logical_cores: usize,
}

impl CpuInfo {
    /// Detect the CPU information for the current host.
    pub fn detect() -> Self {
        Self {
            features: CpuFeatures {
                arch: CpuArch::detect(),
                simd: SimdFlags::detect(),
            },
            logical_cores: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_known_arch() {
        let arch = CpuArch::detect();
        assert!(
            arch == CpuArch::X86_64
            || arch == CpuArch::Aarch64
            || arch == CpuArch::RiscV64
            || arch == CpuArch::Other,
        );
    }

    #[test]
    fn logical_cores_is_at_least_one() {
        let info = CpuInfo::detect();
        assert!(info.logical_cores >= 1);
    }

    #[test]
    fn simd_flags_detect_does_not_panic() {
        let _ = SimdFlags::detect();
    }
}
