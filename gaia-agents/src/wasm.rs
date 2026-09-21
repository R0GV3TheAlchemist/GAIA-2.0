//! Minimal guest-runtime proof for #24.
//! No WASI linker is installed: guests receive no filesystem preopens or network APIs.
//! WASI imports are inspected and refused unless the AIP grant is present.
//! A present grant does not install a WASI host. That is still missing.
//! The only host import is `gaia.log`. Fuel and StoreLimits bind CPU/memory.

use crate::{AgentManifest, AgentRuntime, Capability, RuntimeError};
use wasmtime::{Caller, Config, Engine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmOutcome {
    Output { text: String, chunks: Vec<String> },
    Trapped { agent: String, reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasiGrant {
    pub filesystem: bool,
    pub network: bool,
}

struct HostState {
    output: String,
    chunks: Vec<String>,
    limits: StoreLimits,
}

pub struct WasmRuntime {
    engine: Engine,
    policy: AgentRuntime,
}

impl Default for WasmRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmRuntime {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.consume_fuel(true);
        Self {
            engine: Engine::new(&config).expect("wasmtime engine"),
            policy: AgentRuntime::default(),
        }
    }

    pub fn grants(manifest: &AgentManifest) -> WasiGrant {
        WasiGrant {
            filesystem: manifest.limits.filesystem_allowed
                && (manifest
                    .declared_capabilities
                    .contains(&Capability::FilesystemRead)
                    || manifest
                        .declared_capabilities
                        .contains(&Capability::FilesystemWrite)),
            network: manifest.limits.network_allowed
                && manifest
                    .declared_capabilities
                    .contains(&Capability::Network),
        }
    }

    /// Runs only embedded fixture WAT. Dynamic marketplace package loading is absent.
    pub fn invoke_fixture(
        &self,
        manifest: &AgentManifest,
        fixture: &str,
        requested: Option<Capability>,
    ) -> Result<WasmOutcome, RuntimeError> {
        let wat = match fixture {
            "hello" => HELLO_WAT,
            "trap" => TRAP_WAT,
            "grow" => GROW_WAT,
            "wasi-fs" => WASI_FS_WAT,
            "wasi-net" => WASI_NET_WAT,
            _ => {
                return Ok(WasmOutcome::Trapped {
                    agent: manifest.name.clone(),
                    reason: "unknown embedded fixture".into(),
                });
            }
        };
        self.invoke_wat(manifest, wat, requested)
    }

    /// Load a caller-supplied WAT module. Not a package registry.
    pub fn invoke_wat(
        &self,
        manifest: &AgentManifest,
        wat: &str,
        requested: Option<Capability>,
    ) -> Result<WasmOutcome, RuntimeError> {
        self.policy.admit(manifest)?;
        if let Some(capability) = requested {
            if !manifest.declared_capabilities.contains(&capability) {
                return Err(RuntimeError::UndeclaredCapability {
                    agent: manifest.name.clone(),
                    capability,
                });
            }
        }
        self.instantiate_and_run(manifest, wat)
    }

    fn instantiate_and_run(
        &self,
        manifest: &AgentManifest,
        wat: &str,
    ) -> Result<WasmOutcome, RuntimeError> {
        let module =
            Module::new(&self.engine, wat).map_err(|error| RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: format!("module rejected: {error}"),
            })?;
        self.enforce_wasi_grants(manifest, &module)?;

        let mut linker = Linker::new(&self.engine);
        linker
            .func_wrap(
                "gaia",
                "log",
                |mut caller: Caller<'_, HostState>, ptr: i32, len: i32| {
                    if ptr < 0 || len < 0 {
                        return;
                    }
                    let Some(memory) = caller.get_export("memory").and_then(|e| e.into_memory())
                    else {
                        return;
                    };
                    let data = memory.data(&caller);
                    let start = ptr as usize;
                    let end = start.saturating_add(len as usize);
                    if end <= data.len() {
                        let chunk = String::from_utf8_lossy(&data[start..end]).into_owned();
                        caller.data_mut().output.push_str(&chunk);
                        caller.data_mut().chunks.push(chunk);
                    }
                },
            )
            .map_err(|error| RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: format!("host bridge rejected: {error}"),
            })?;

        let memory_bytes = (manifest.limits.memory_mib as usize).saturating_mul(1024 * 1024);
        let limits = StoreLimitsBuilder::new()
            .memory_size(memory_bytes.max(1))
            .instances(1)
            .tables(1)
            .memories(1)
            .trap_on_grow_failure(true)
            .build();
        let mut store = Store::new(
            &self.engine,
            HostState {
                output: String::new(),
                chunks: Vec::new(),
                limits,
            },
        );
        store.limiter(|state| &mut state.limits);
        let fuel = manifest.limits.cpu_millis.saturating_mul(10_000);
        store
            .set_fuel(fuel)
            .map_err(|error| RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: format!("fuel rejected: {error}"),
            })?;

        let instance = match linker.instantiate(&mut store, &module) {
            Ok(instance) => instance,
            Err(error) => {
                return Ok(WasmOutcome::Trapped {
                    agent: manifest.name.clone(),
                    reason: error.to_string(),
                });
            }
        };
        let run = match instance.get_typed_func::<(), ()>(&mut store, "run") {
            Ok(func) => func,
            Err(error) => {
                return Err(RuntimeError::LimitRejected {
                    agent: manifest.name.clone(),
                    reason: format!("missing fixture entrypoint: {error}"),
                });
            }
        };
        match run.call(&mut store, ()) {
            Ok(()) => Ok(WasmOutcome::Output {
                text: store.data().output.clone(),
                chunks: store.data().chunks.clone(),
            }),
            Err(error) => Ok(WasmOutcome::Trapped {
                agent: manifest.name.clone(),
                reason: error.to_string(),
            }),
        }
    }

    fn enforce_wasi_grants(
        &self,
        manifest: &AgentManifest,
        module: &Module,
    ) -> Result<(), RuntimeError> {
        let grants = Self::grants(manifest);
        let mut wants_fs = false;
        let mut wants_net = false;
        for import in module.imports() {
            if !is_wasi_module(import.module()) {
                continue;
            }
            if is_wasi_network(import.name()) {
                wants_net = true;
            } else {
                wants_fs = true;
            }
        }
        if wants_net && !grants.network {
            return Err(RuntimeError::UndeclaredCapability {
                agent: manifest.name.clone(),
                capability: Capability::Network,
            });
        }
        if wants_fs && !grants.filesystem {
            return Err(RuntimeError::UndeclaredCapability {
                agent: manifest.name.clone(),
                capability: Capability::FilesystemRead,
            });
        }
        if wants_fs || wants_net {
            return Err(RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: "WASI host is not installed; grant recorded but no preopens or sockets"
                    .into(),
            });
        }
        Ok(())
    }
}

fn is_wasi_module(module: &str) -> bool {
    module.starts_with("wasi")
}

fn is_wasi_network(name: &str) -> bool {
    name.starts_with("sock_") || name.contains("sock")
}

const HELLO_WAT: &str = r#"
(module
  (import "gaia" "log" (func $log (param i32 i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "hello from wasm")
  (data (i32.const 16) "streamed")
  (func (export "run")
    i32.const 0
    i32.const 15
    call $log
    i32.const 16
    i32.const 8
    call $log))
"#;

const TRAP_WAT: &str = r#"
(module
  (func (export "run") unreachable))
"#;

const GROW_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "run")
    i32.const 64
    memory.grow
    i32.const -1
    i32.eq
    if
      unreachable
    end))
"#;

const WASI_FS_WAT: &str = r#"
(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (func (export "run") nop))
"#;

const WASI_NET_WAT: &str = r#"
(module
  (import "wasi_snapshot_preview1" "sock_accept" (func $sock_accept (param i32 i32 i32) (result i32)))
  (func (export "run") nop))
"#;

// ── Tests (#24 acceptance criteria) ─────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AgentManifest, Capability, ResourceLimits, RuntimeError};

    fn sandbox_manifest(name: &str) -> AgentManifest {
        AgentManifest {
            name: name.into(),
            version: "0.1.0".into(),
            declared_capabilities: vec![],
            limits: ResourceLimits::default(), // 1 000 ms CPU, 64 MiB, no net/fs
        }
    }

    fn manifest_with_caps(name: &str, caps: Vec<Capability>) -> AgentManifest {
        AgentManifest {
            name: name.into(),
            version: "0.1.0".into(),
            declared_capabilities: caps,
            limits: ResourceLimits::default(),
        }
    }

    // ── AC1: Hello-world agent runs in the sandbox ─────────────────────────

    #[test]
    fn hello_world_runs_and_produces_output() {
        let rt = WasmRuntime::new();
        let manifest = sandbox_manifest("hello-agent");
        let result = rt.invoke_fixture(&manifest, "hello", None);
        match result {
            Ok(WasmOutcome::Output { text, chunks }) => {
                assert_eq!(text, "hello from wasmstreamed",
                    "combined output text must match both log calls");
                assert_eq!(chunks.len(), 2,
                    "two gaia.log calls must produce two chunks");
                assert_eq!(chunks[0], "hello from wasm");
                assert_eq!(chunks[1], "streamed");
            }
            other => panic!("expected Output, got {other:?}"),
        }
    }

    #[test]
    fn hello_world_output_is_streamed_in_order() {
        let rt = WasmRuntime::new();
        let manifest = sandbox_manifest("stream-agent");
        let Ok(WasmOutcome::Output { chunks, .. }) =
            rt.invoke_fixture(&manifest, "hello", None)
        else {
            panic!("expected Output");
        };
        assert_eq!(chunks, vec!["hello from wasm", "streamed"],
            "chunks must arrive in call order");
    }

    // ── AC2: Denied capabilities cannot be exercised ───────────────────────

    #[test]
    fn filesystem_wasi_import_denied_without_grant() {
        let rt = WasmRuntime::new();
        // No FilesystemRead/Write in declared_capabilities, filesystem_allowed=false
        let manifest = sandbox_manifest("fs-agent");
        let result = rt.invoke_fixture(&manifest, "wasi-fs", None);
        assert!(
            matches!(result, Err(RuntimeError::UndeclaredCapability { capability: Capability::FilesystemRead, .. })),
            "wasi-fs without grant must return UndeclaredCapability(FilesystemRead), got {result:?}"
        );
    }

    #[test]
    fn network_wasi_import_denied_without_grant() {
        let rt = WasmRuntime::new();
        let manifest = sandbox_manifest("net-agent");
        let result = rt.invoke_fixture(&manifest, "wasi-net", None);
        assert!(
            matches!(result, Err(RuntimeError::UndeclaredCapability { capability: Capability::Network, .. })),
            "wasi-net without grant must return UndeclaredCapability(Network), got {result:?}"
        );
    }

    #[test]
    fn undeclared_capability_request_denied_at_invoke() {
        let rt = WasmRuntime::new();
        // Manifest has no ToolInvoke capability declared.
        let manifest = sandbox_manifest("capability-agent");
        let result = rt.invoke_fixture(&manifest, "hello", Some(Capability::ToolInvoke));
        assert!(
            matches!(result, Err(RuntimeError::UndeclaredCapability { capability: Capability::ToolInvoke, .. })),
            "requesting an undeclared capability must be rejected before execution"
        );
    }

    #[test]
    fn declared_capability_is_admitted() {
        let rt = WasmRuntime::new();
        let manifest = manifest_with_caps("capable-agent", vec![Capability::ToolInvoke]);
        // Should run normally — no WASI imports in hello fixture.
        let result = rt.invoke_fixture(&manifest, "hello", Some(Capability::ToolInvoke));
        assert!(
            matches!(result, Ok(WasmOutcome::Output { .. })),
            "declared capability must be admitted and the agent must run"
        );
    }

    // ── AC3: Runtime crash is isolated to that agent ───────────────────────

    #[test]
    fn trap_is_caught_and_does_not_crash_host() {
        let rt = WasmRuntime::new();
        let manifest = sandbox_manifest("crash-agent");
        let result = rt.invoke_fixture(&manifest, "trap", None);
        match result {
            Ok(WasmOutcome::Trapped { agent, reason }) => {
                assert_eq!(agent, "crash-agent");
                assert!(!reason.is_empty(), "trap reason must be non-empty");
            }
            other => panic!("expected Trapped outcome, host must not panic; got {other:?}"),
        }
    }

    #[test]
    fn memory_grow_beyond_limit_is_caught() {
        let rt = WasmRuntime::new();
        // 1 MiB limit — grow fixture attempts 64 pages (4 MiB), must trap.
        let manifest = AgentManifest {
            name: "grow-agent".into(),
            version: "0.1.0".into(),
            declared_capabilities: vec![],
            limits: ResourceLimits {
                memory_mib: 1,
                ..ResourceLimits::default()
            },
        };
        let result = rt.invoke_fixture(&manifest, "grow", None);
        assert!(
            matches!(result, Ok(WasmOutcome::Trapped { .. })),
            "exceeding the memory limit must produce a Trapped outcome, not a host panic"
        );
    }

    #[test]
    fn crashing_agent_does_not_affect_subsequent_agent() {
        let rt = WasmRuntime::new();
        let crash_manifest = sandbox_manifest("crash-agent");
        let hello_manifest = sandbox_manifest("hello-agent");

        // First agent crashes.
        let crash_result = rt.invoke_fixture(&crash_manifest, "trap", None);
        assert!(matches!(crash_result, Ok(WasmOutcome::Trapped { .. })));

        // Second agent runs cleanly on the same WasmRuntime — host still alive.
        let hello_result = rt.invoke_fixture(&hello_manifest, "hello", None);
        assert!(
            matches!(hello_result, Ok(WasmOutcome::Output { .. })),
            "a subsequent agent must still run after a previous agent trapped"
        );
    }
}
