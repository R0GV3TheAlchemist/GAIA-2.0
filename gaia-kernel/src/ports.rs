//! #30 port profiles. Same primitives, distinct footprints.
//! These are documented targets, not proven CI images for every ISA.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    Aarch64,
    RiscV64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortProfile {
    LinuxX86_64,
    LinuxArm64,
    AppleSilicon,
    RiscVExperimental,
    Iot,
    Hpc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Footprint {
    pub arch: Arch,
    pub rustc_target: &'static str,
    pub max_rss_kib: u32,
    pub max_binary_kib: u32,
    pub executor: bool,
    pub sensors: bool,
    pub memos: bool,
    pub agents: bool,
}

impl PortProfile {
    pub fn footprint(self) -> Footprint {
        match self {
            Self::LinuxX86_64 => Footprint {
                arch: Arch::X86_64,
                rustc_target: "x86_64-unknown-linux-gnu",
                max_rss_kib: 512 * 1024,
                max_binary_kib: 64 * 1024,
                executor: true,
                sensors: true,
                memos: true,
                agents: true,
            },
            Self::LinuxArm64 | Self::AppleSilicon => Footprint {
                arch: Arch::Aarch64,
                rustc_target: if matches!(self, Self::AppleSilicon) {
                    "aarch64-apple-darwin"
                } else {
                    "aarch64-unknown-linux-gnu"
                },
                max_rss_kib: 512 * 1024,
                max_binary_kib: 64 * 1024,
                executor: true,
                sensors: true,
                memos: true,
                agents: true,
            },
            Self::RiscVExperimental => Footprint {
                arch: Arch::RiscV64,
                rustc_target: "riscv64gc-unknown-linux-gnu",
                max_rss_kib: 256 * 1024,
                max_binary_kib: 32 * 1024,
                executor: true,
                sensors: true,
                memos: true,
                agents: false,
            },
            Self::Iot => Footprint {
                arch: Arch::Aarch64,
                rustc_target: "aarch64-unknown-linux-gnu",
                max_rss_kib: 8 * 1024,
                max_binary_kib: 2 * 1024,
                executor: true,
                sensors: true,
                memos: false,
                agents: false,
            },
            Self::Hpc => Footprint {
                arch: Arch::X86_64,
                rustc_target: "x86_64-unknown-linux-gnu",
                max_rss_kib: 32 * 1024 * 1024,
                max_binary_kib: 128 * 1024,
                executor: true,
                sensors: false,
                memos: true,
                agents: true,
            },
        }
    }

    pub fn documented_build(self) -> String {
        let fp = self.footprint();
        format!(
            "rustup target add {} && cargo build -p gaia-kernel --target {}",
            fp.rustc_target, fp.rustc_target
        )
    }
}

/// Compatibility rows for docs and tests.
pub fn matrix() -> Vec<PortProfile> {
    vec![
        PortProfile::LinuxX86_64,
        PortProfile::LinuxArm64,
        PortProfile::AppleSilicon,
        PortProfile::RiscVExperimental,
        PortProfile::Iot,
        PortProfile::Hpc,
    ]
}
