//! Minimal guest-runtime proof for #24.
//! No WASI linker is installed: guests receive no filesystem preopens or network APIs.
//! The only host import is `gaia.log`, which copies fixture bytes into an in-memory result.

use crate::{AgentManifest, Capability, RuntimeError};
use wasmtime::{Caller, Engine, Linker, Module, Store};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmOutcome {
    Output(String),
    Trapped { agent: String, reason: String },
}

#[derive(Default)]
struct HostState {
    output: String,
}

pub struct WasmRuntime {
    engine: Engine,
}

impl Default for WasmRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmRuntime {
    pub fn new() -> Self {
        Self { engine: Engine::default() }
    }

    /// Runs only embedded fixture WAT. Dynamic/package module loading is intentionally absent.
    pub fn invoke_fixture(
        &self,
        manifest: &AgentManifest,
        fixture: &str,
        requested: Option<Capability>,
    ) -> Result<WasmOutcome, RuntimeError> {
        if let Some(capability) = requested {
            if !manifest.declared_capabilities.contains(&capability) {
                return Err(RuntimeError::UndeclaredCapability {
                    agent: manifest.name.clone(),
                    capability,
                });
            }
        }
        let wat = match fixture {
            "hello" => HELLO_WAT,
            "trap" => TRAP_WAT,
            _ => {
                return Ok(WasmOutcome::Trapped {
                    agent: manifest.name.clone(),
                    reason: "unknown embedded fixture".into(),
                });
            }
        };
        let module = Module::new(&self.engine, wat).map_err(|error| RuntimeError::LimitRejected {
            agent: manifest.name.clone(),
            reason: format!("fixture module rejected: {error}"),
        })?;
        let mut linker = Linker::new(&self.engine);
        linker
            .func_wrap(
                "gaia",
                "log",
                |mut caller: Caller<'_, HostState>, ptr: i32, len: i32| {
                    if ptr < 0 || len < 0 {
                        return;
                    }
                    let Some(memory) = caller.get_export("memory").and_then(|e| e.into_memory()) else {
                        return;
                    };
                    let data = memory.data(&caller);
                    let start = ptr as usize;
                    let end = start.saturating_add(len as usize);
                    if end <= data.len() {
                        caller.data_mut().output.push_str(&String::from_utf8_lossy(&data[start..end]));
                    }
                },
            )
            .map_err(|error| RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: format!("host bridge rejected: {error}"),
            })?;
        let mut store = Store::new(&self.engine, HostState::default());
        let instance = linker.instantiate(&mut store, &module).map_err(|error| RuntimeError::LimitRejected {
            agent: manifest.name.clone(),
            reason: format!("guest instantiation failed: {error}"),
        })?;
        let run = instance
            .get_typed_func::<(), ()>(&mut store, "run")
            .map_err(|error| RuntimeError::LimitRejected {
                agent: manifest.name.clone(),
                reason: format!("missing fixture entrypoint: {error}"),
            })?;
        match run.call(&mut store, ()) {
            Ok(()) => Ok(WasmOutcome::Output(store.data().output.clone())),
            Err(error) => Ok(WasmOutcome::Trapped {
                agent: manifest.name.clone(),
                reason: error.to_string(),
            }),
        }
    }
}

const HELLO_WAT: &str = r#"
(module
  (import "gaia" "log" (func $log (param i32 i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "hello from wasm")
  (func (export "run")
    i32.const 0
    i32.const 15
    call $log))
"#;

const TRAP_WAT: &str = r#"
(module
  (func (export "run") unreachable))
"#;
