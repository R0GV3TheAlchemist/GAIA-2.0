use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EgressClass {
    Allowlisted,
    PublicOrUnknown,
    ForbiddenSsrf,
}

pub fn classify_destination(dest: &str) -> EgressClass {
    let d = dest.trim().to_ascii_lowercase();
    if d.is_empty() {
        return EgressClass::ForbiddenSsrf;
    }
    if looks_ssrf(&d) {
        return EgressClass::ForbiddenSsrf;
    }
    EgressClass::PublicOrUnknown
}

fn looks_ssrf(d: &str) -> bool {
    d.contains("localhost")
        || d.contains("127.0.0.1")
        || d.contains("0.0.0.0")
        || d.contains("[::1]")
        || d.contains("::1")
        || d.contains("169.254.")
        || d.contains("metadata.google.internal")
        || d.contains("169.254.169.254")
        || d.contains("10.")
        || d.contains("192.168.")
        || d.contains("172.16.")
        || d.contains("172.17.")
        || d.contains("172.18.")
        || d.contains("172.19.")
        || d.contains("172.2")
        || d.contains("fc00:")
        || d.contains("fd00:")
        || d.contains("fe80:")
        || d.contains("socks5://")
        || d.contains("tor")
        || d.contains("onion")
        || d.starts_with("file:")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandboxProfile {
    pub root: bool,
    pub host_mounts: bool,
    pub docker_socket: bool,
    pub inherited_env: bool,
    pub network: bool,
    pub scratch_only_writes: bool,
}

impl Default for SandboxProfile {
    fn default() -> Self {
        Self {
            root: false,
            host_mounts: false,
            docker_socket: false,
            inherited_env: false,
            network: false,
            scratch_only_writes: true,
        }
    }
}

impl SandboxProfile {
    pub fn is_baseline(&self) -> bool {
        !self.root
            && !self.host_mounts
            && !self.docker_socket
            && !self.inherited_env
            && !self.network
            && self.scratch_only_writes
    }
}
