//! Pre-WASM/WASI test seam. Capability values are policy data, not enforcement.
//! Do not describe this module as a sandbox until a WASM/WASI runtime enforces it.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Capability {
    MemoryRead,
    MemoryWrite,
    FilesystemRead,
    FilesystemWrite,
    Network,
    ToolInvoke,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceLimits {
    pub cpu_millis: u64,
    pub memory_mib: u64,
    pub network_allowed: bool,
    pub filesystem_allowed: bool,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            cpu_millis: 1_000,
            memory_mib: 64,
            network_allowed: false,
            filesystem_allowed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentManifest {
    pub name: String,
    pub version: String,
    pub declared_capabilities: Vec<Capability>,
    pub limits: ResourceLimits,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentOutcome {
    Output(String),
    Crashed { agent: String, reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    UndeclaredCapability { agent: String, capability: Capability },
    LimitRejected { agent: String, reason: String },
}

/// Local policy harness. It executes fixture behaviors only and performs no I/O.
pub struct AgentRuntime {
    pub max_cpu_millis: u64,
    pub max_memory_mib: u64,
}

impl Default for AgentRuntime {
    fn default() -> Self {
        Self {
            max_cpu_millis: 10_000,
            max_memory_mib: 512,
        }
    }
}

impl AgentRuntime {
    pub fn admit(&self, manifest: &AgentManifest) -> Result<(), RuntimeError> {
        if manifest.limits.cpu_millis > self.max_cpu_millis {
            return Err(RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: "cpu limit exceeds local policy".into(),
            });
        }
        if manifest.limits.memory_mib > self.max_memory_mib {
            return Err(RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: "memory limit exceeds local policy".into(),
            });
        }
        if manifest.limits.network_allowed
            && !manifest.declared_capabilities.contains(&Capability::Network)
        {
            return Err(RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: "network limit requires declared network capability".into(),
            });
        }
        if manifest.limits.filesystem_allowed
            && !manifest.declared_capabilities.contains(&Capability::FilesystemRead)
            && !manifest.declared_capabilities.contains(&Capability::FilesystemWrite)
        {
            return Err(RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: "filesystem limit requires declared filesystem capability".into(),
            });
        }
        Ok(())
    }

    pub fn invoke(
        &self,
        manifest: &AgentManifest,
        behavior: &str,
        requested: Option<Capability>,
    ) -> Result<AgentOutcome, RuntimeError> {
        self.admit(manifest)?;
        if let Some(capability) = requested {
            if !manifest.declared_capabilities.contains(&capability) {
                return Err(RuntimeError::UndeclaredCapability {
                    agent: manifest.name.clone(),
                    capability,
                });
            }
        }
        if behavior == "crash" {
            return Ok(AgentOutcome::Crashed {
                agent: manifest.name.clone(),
                reason: "fixture crash isolated by runtime result boundary".into(),
            });
        }
        Ok(AgentOutcome::Output(format!("hello from {}", manifest.name)))
    }
}
