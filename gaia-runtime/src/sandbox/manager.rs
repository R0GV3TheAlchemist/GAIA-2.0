//! [`SandboxManager`] — Wasmtime 46 + WASI 0.3 (p3) real isolation backend.
//!
//! # Store data layout
//!
//! Wasmtime requires the `ResourceLimiter` to be stored inside the `Store`'s
//! data type and accessed via a closure: `store.limiter(|d| &mut d.limiter)`.
//! We use [`StoreData`] to bundle the WASI context and the resource limiter
//! together so there is no unsafe memory leaking.
//!
//! # Import paths
//!
//! `wasmtime_wasi::p3` is a sub-module that contains WASI 0.3 *bindings* and
//! linker helpers (`add_to_linker`, etc.).  The [`WasiCtx`] and
//! [`WasiCtxBuilder`] types are re-exported from the **crate root** in every
//! version of `wasmtime-wasi`, including 46.  Enabling the `p3` crate feature
//! merely gates the `wasmtime_wasi::p3` sub-module; it does not relocate the
//! context types there.

use std::path::PathBuf;

use cap_std::ambient_authority;
use wasmtime::{
    component::Component,
    Config, Engine, Store,
};
// WasiCtx and WasiCtxBuilder live at the wasmtime_wasi crate root (re-exported
// from the internal `ctx` module) regardless of which feature flags are active.
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

use crate::sandbox::{
    error::{SandboxError, GAIA_CAPABILITY_DENIED},
    limits::GaiaResourceLimiter,
    profile::SandboxProfile,
};

// ── StoreData ────────────────────────────────────────────────────────────────

/// Data stored inside every Wasmtime `Store` created by this manager.
///
/// `wasmtime::Store<StoreData>` gives Wasmtime a single place to reach
/// both the WASI context and the resource limiter without any heap leaking.
pub struct StoreData {
    /// WASI context (capability-granted I/O state).
    pub wasi:    WasiCtx,
    /// Memory + table quota enforcer.
    pub limiter: GaiaResourceLimiter,
}

// ── SandboxManager ───────────────────────────────────────────────────────────

/// Wasmtime 46 + WASI 0.3 sandbox backend.
///
/// One `SandboxManager` per agent type is typical; the internal `Engine` is
/// `Arc`-backed and safe to share across threads/tasks.
pub struct SandboxManager {
    engine:  Engine,
    profile: SandboxProfile,
}

impl SandboxManager {
    /// Construct a manager, initialising the Wasmtime engine.
    ///
    /// Two engine flags are **mandatory** for WASI 0.3:
    /// - `wasm_component_model` — enables the Component Model ABI.
    /// - `epoch_interruption`   — required for `set_epoch_deadline`.
    ///
    /// Note: `async_support` was removed in Wasmtime 46 (always available;
    /// calling it is a deprecated no-op that emits a warning).
    pub fn new(profile: SandboxProfile) -> Result<Self, SandboxError> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.epoch_interruption(true);

        // Engine::new returns Result<_, wasmtime::Error>; convert explicitly
        // because SandboxError implements From<anyhow::Error>.
        let engine = Engine::new(&config)
            .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;
        Ok(Self { engine, profile })
    }

    /// Build a WASI context that grants only the capabilities declared in the profile.
    ///
    /// # WASI 0.3 capability model
    ///
    /// In WASI 0.3 a component's capabilities are enforced at the world-import
    /// level: a component without a `wasi:sockets` world import physically
    /// cannot open sockets.  Omitting the socket linker binding is therefore
    /// sufficient for network isolation.
    pub fn build_wasi_ctx(&self) -> WasiCtx {
        let mut builder = WasiCtxBuilder::new();

        // Filesystem — mount /scratch only when the policy allows it.
        // Default: no preopens → component has zero filesystem access.
        if self.profile.scratch_only_writes {
            let scratch = scratch_dir();
            let _ = std::fs::create_dir_all(&scratch);
            let dir = cap_std::fs::Dir::open_ambient_dir(&scratch, ambient_authority())
                .expect("scratch_dir must be accessible on the host");
            builder.preopened_dir(dir, "/scratch");
        }

        // Environment — forward host env only when explicitly declared.
        if self.profile.inherited_env {
            builder.inherit_env();
        }

        builder.build()
    }

    /// Compile a WASM Component Model binary.
    ///
    /// The result is deterministic and may be cached by the caller.
    pub fn compile(&self, bytes: &[u8]) -> Result<Component, SandboxError> {
        Component::new(&self.engine, bytes)
            .map_err(|e| SandboxError::CompileError(e.to_string()))
    }

    /// Execute a pre-compiled component inside the sandbox.
    ///
    /// - Attaches [`GaiaResourceLimiter`] (memory + table quotas) via
    ///   `store.limiter(|d| &mut d.limiter)`.
    /// - Sets epoch deadline for wall-clock CPU budget.
    ///
    /// Full typed component-world invocation (`call_run` / WIT bindings)
    /// is wired in issue #720 (Execution Engine).
    pub async fn execute_component(
        &self,
        component: &Component,
    ) -> Result<(), SandboxError> {
        let store_data = StoreData {
            wasi:    self.build_wasi_ctx(),
            limiter: GaiaResourceLimiter::new(self.profile.quota),
        };
        let mut store = Store::new(&self.engine, store_data);

        store.limiter(|data: &mut StoreData| &mut data.limiter);
        store.set_epoch_deadline(self.profile.quota.max_epochs);

        let _linker: wasmtime::component::Linker<StoreData> =
            wasmtime::component::Linker::new(&self.engine);

        let _ = component;
        Ok(())
    }

    /// Classify a raw Wasmtime error into a structured [`SandboxError`].
    pub fn classify_trap(err: &anyhow::Error) -> SandboxError {
        let msg = err.to_string().to_lowercase();
        if msg.contains("out of memory") || msg.contains("oom") {
            SandboxError::OomTermination
        } else if msg.contains("epoch") || msg.contains("interrupt") {
            SandboxError::Timeout
        } else if msg.contains("eacces") || msg.contains("permission denied") {
            SandboxError::CapabilityDenied(GAIA_CAPABILITY_DENIED.to_owned())
        } else {
            SandboxError::Trap(err.to_string())
        }
    }

    /// Return a reference to the active [`SandboxProfile`].
    pub fn profile(&self) -> &SandboxProfile {
        &self.profile
    }

    /// Return a reference to the underlying Wasmtime [`Engine`].
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}

/// Returns the host path used as the sandbox `/scratch` mount point.
fn scratch_dir() -> PathBuf {
    std::env::temp_dir().join("gaia-sandbox-scratch")
}
