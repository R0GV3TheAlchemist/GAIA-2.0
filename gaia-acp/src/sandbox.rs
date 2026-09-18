use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EgressClass {
    Allowlisted,
    PublicOrUnknown,
    ForbiddenSsrf,
}

/// Deterministic destination class. No DNS, no sockets.
pub fn classify_destination(dest: &str) -> EgressClass {
    let d = dest.trim().to_ascii_lowercase();
    if d.is_empty() {
        return EgressClass::ForbiddenSsrf;
    }
    if looks_proxy_or_file(&d) {
        return EgressClass::ForbiddenSsrf;
    }
    let host = extract_host(&d);
    if host_is_forbidden(&host) {
        return EgressClass::ForbiddenSsrf;
    }
    if let Some(ip) = parse_ipv4_any(&host) {
        if ipv4_forbidden(ip) {
            return EgressClass::ForbiddenSsrf;
        }
    }
    EgressClass::PublicOrUnknown
}

/// Simulated redirect: classify the Location, never follow a network hop.
pub fn classify_redirect(from: &str, location: &str) -> EgressClass {
    let _ = from;
    classify_destination(location)
}

/// Static rebind table. Hosts in this list resolve to loopback in fixtures only.
pub fn classify_rebinding_host(host: &str) -> EgressClass {
    let h = host.trim().to_ascii_lowercase();
    if h.ends_with(".rebind.test") || h == "rebind.local" || h.contains("nip.io") {
        return EgressClass::ForbiddenSsrf;
    }
    classify_destination(host)
}

fn looks_proxy_or_file(d: &str) -> bool {
    d.starts_with("file:")
        || d.contains("socks5://")
        || d.contains("socks4://")
        || d.contains(".onion")
        || d.contains("tor+")
}

fn extract_host(d: &str) -> String {
    let rest = if let Some(i) = d.find("://") {
        &d[i + 3..]
    } else {
        d
    };
    let rest = rest.split('/').next().unwrap_or(rest);
    let rest = rest.split('?').next().unwrap_or(rest);
    let hostport = if let Some(at) = rest.rfind('@') {
        &rest[at + 1..]
    } else {
        rest
    };
    let host = if hostport.starts_with('[') {
        hostport
            .trim_start_matches('[')
            .split(']')
            .next()
            .unwrap_or(hostport)
            .to_string()
    } else {
        hostport.split(':').next().unwrap_or(hostport).to_string()
    };
    host.trim_matches('.').to_string()
}

fn host_is_forbidden(host: &str) -> bool {
    host == "localhost"
        || host == "metadata.google.internal"
        || host.ends_with(".internal")
        || host.ends_with(".local")
        || host.contains("169.254.")
        || host.contains(":ffff:127.")
        || host == "::1"
        || host == "0.0.0.0"
}

fn parse_ipv4_any(host: &str) -> Option<u32> {
    if let Ok(n) = parse_dotted(host) {
        return Some(n);
    }
    if let Some(rest) = host.strip_prefix("0x") {
        if let Ok(n) = u32::from_str_radix(rest, 16) {
            return Some(n);
        }
    }
    if host.chars().all(|c| c.is_ascii_digit()) {
        return host.parse::<u32>().ok();
    }
    None
}

fn parse_dotted(host: &str) -> Result<u32, ()> {
    let parts: Vec<&str> = host.split('.').collect();
    if parts.is_empty() || parts.len() > 4 {
        return Err(());
    }
    let mut nums = [0u32; 4];
    for (i, p) in parts.iter().enumerate() {
        nums[i] = parse_octet(p)?;
    }
    let ip = match parts.len() {
        1 => nums[0],
        2 => (nums[0] << 24) | (nums[1] & 0x00ff_ffff),
        3 => (nums[0] << 24) | (nums[1] << 16) | (nums[2] & 0xffff),
        4 => (nums[0] << 24) | (nums[1] << 16) | (nums[2] << 8) | nums[3],
        _ => return Err(()),
    };
    Ok(ip)
}

fn parse_octet(p: &str) -> Result<u32, ()> {
    if p.is_empty() {
        return Err(());
    }
    if let Some(rest) = p.strip_prefix("0x") {
        return u32::from_str_radix(rest, 16).map_err(|_| ());
    }
    if p.starts_with('0') && p.len() > 1 && p.chars().all(|c| ('0'..='7').contains(&c)) {
        return u32::from_str_radix(p, 8).map_err(|_| ());
    }
    p.parse::<u32>().map_err(|_| ())
}

fn ipv4_forbidden(ip: u32) -> bool {
    let a = (ip >> 24) as u8;
    let b = (ip >> 16) as u8;
    if a == 127 || a == 0 || a == 10 || a == 255 {
        return true;
    }
    if a == 169 && b == 254 {
        return true;
    }
    if a == 192 && b == 168 {
        return true;
    }
    if a == 172 && (16..=31).contains(&b) {
        return true;
    }
    false
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
