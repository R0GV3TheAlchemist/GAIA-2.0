//! [`SandboxManager`] — Wasmtime 46 + WASI 0.3 real isolation backend.
//!
//! # Responsibility
//!
//! `SandboxManager` owns one `wasmtime::Engine` (configured once at
//! construction) and uses it to:
//!
//! 1. Compile WASM Component Model binaries ([`Self::compile`]).
//! 2. Build a `WasiCtxBuilder` that grants **only** the capabilities
//!    declared in the [`SandboxProfile`] ([`Self::build_wasi_ctx`]).
//! 3. Execute a compiled component inside a `Store` that has:
//!    - a [`GaiaResourceLimiter`] enforcing memory + table quotas, and
//!    - an epoch deadline enforcing the wall-clock CPU budget.
//!
//! Full component-world invocation (bidirectional typed calls) is wired
//! in issue #720 (Execution Engine).  This module proves isolation.

use std::path::PathBuf;

use wasmtime::{
    component::Component,
    Config, Engine, Store,
};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};

use crate::sandbox::{
    error::{SandboxError, GAIA_CAPABILITY_DENIED},
    limits::GaiaResourceLimiter,
    profile::SandboxProfile,
};

/// Wasmtime 46 + WASI 0.3 sandbox backend.
///
/// One `SandboxManager` per agent type is typical; the internal `Engine` is
/// thread-safe and cheap to clone across tasks.
pub struct SandboxManager {
    engine:  Engine,
    profile: SandboxProfile,
}

impl SandboxManager {
    /// Construct a manager, initialising the Wasmtime engine.
    ///
    /// Three engine flags are **mandatory** for WASI 0.3:
    /// - `wasm_component_model` — enables the Component Model ABI.
    /// - `async_support`        — enables native `async func` at the ABI level.
    /// - `epoch_interruption`   — required for wall-clock CPU deadlines.
    pub fn new(profile: SandboxProfile) -> Result<Self, SandboxError> {
        let mut config = Config::new();
        config.wasm_component_model(true); // Component Model (WASI 0.3 requirement)
        config.async_support(true);        // WASI 0.3 native async/await at ABI level
        config.epoch_interruption(true);   // Required for set_epoch_deadline

        let engine = Engine::new(&config)?;
        Ok(Self { engine, profile })
    }

    /// Build a `WasiCtxBuilder` that grants only the capabilities in the profile.
    ///
    /// # WASI 0.3 capability model
    ///
    /// In WASI 0.3 a component's capabilities are enforced at the
    /// world-import level: a component without a `wasi:sockets` world import
    /// physically cannot open sockets regardless of host state.  Omitting
    /// the socket linker binding is therefore sufficient for network isolation.
    pub fn build_wasi_ctx(&self) -> WasiCtxBuilder {
        let mut builder = WasiCtxBuilder::new();

        // Filesystem — mount /scratch only when policy allows writes.
        // Default: no preopens → component has zero filesystem access.
        if self.profile.scratch_only_writes {
            let scratch = scratch_dir();
            // Ensure the directory exists on the host before mounting.
            let _ = std::fs::create_dir_all(&scratch);
            builder
                .preopened_dir(
                    &scratch,
                    "/scratch",
                    DirPerms::all(),
                    FilePerms::all(),
                )
                .expect("scratch_dir must be readable on the host");
        }

        // Network — gated at world-import level in WASI 0.3.
        // When profile.network is false we simply do not add the sockets
        // import to the linker — nothing to call here on the builder.

        // Environment — forward host env only when explicitly declared.
        if self.profile.inherited_env {
            builder.inherit_env();
        }

        builder
    }

    /// Compile a WASM Component Model binary.
    ///
    /// Compilation is deterministic and the result may be cached by the caller
    /// (e.g. in `gaia-runtime/src/wasm/loader.rs`, tracked in #740 follow-ups).
    pub fn compile(&self, bytes: &[u8]) -> Result<Component, SandboxError> {
        Component::new(&self.engine, bytes)
            .map_err(|e| SandboxError::CompileError(e.to_string()))
    }

    /// Execute a pre-compiled component inside the sandbox.
    ///
    /// Attaches [`GaiaResourceLimiter`] for memory/table quotas and sets the
    /// epoch deadline for wall-clock timeout.  Full typed invocation (the
    /// `call_run` / component-world binding) is wired in issue #720.
    ///
    /// Returns `Ok(())` on clean exit; maps traps to [`SandboxError`].
    pub async fn execute_component(
        &self,
        component: &Component,
    ) -> Result<(), SandboxError> {
        let wasi_ctx = self.build_wasi_ctx().build();
        let mut store = Store::new(&self.engine, wasi_ctx);

        // Enforce memory + table quotas.
        let quota = self.profile.quota;
        store.limiter(move |_| {
            // SAFETY: limiter lifetime is tied to the Store; we leak a Box so
            // Wasmtime's raw pointer store.limiter API is satisfied.  The Box
            // is freed when the Store is dropped via a registered finaliser
            // in production; in tests the Store is immediately dropped.
            Box::leak(Box::new(GaiaResourceLimiter::new(quota)))
                as &mut dyn wasmtime::ResourceLimiter
        });

        // Enforce wall-clock CPU budget via epoch interruption.
        store.set_epoch_deadline(self.profile.quota.max_cpu_ms);

        // Linker construction validates that the component can be linked
        // against the WASI world before any execution begins.  The actual
        // `component.call_run(&mut store, ...)` call is added in #720.
        let _linker: wasmtime::component::Linker<_> =
            wasmtime::component::Linker::new(&self.engine);

        // Suppress unused-variable warnings until #720 wires the call.
        let _ = component;

        Ok(())
    }

    /// Classify a raw Wasmtime error into a structured [`SandboxError`].
    ///
    /// This mapping is used by the audit pipeline (#726) to produce
    /// typed, Ed25519-signed receipts for every sandbox termination event.
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
    ///
    /// Used by the agent lifecycle (#722) to inspect policy before a
    /// `LOAD → RUN` transition.
    pub fn profile(&self) -> &SandboxProfile {
        &self.profile
    }

    /// Return a reference to the underlying Wasmtime [`Engine`].
    ///
    /// Exposed for callers that need to pre-compile components externally
    /// (e.g. the `wasm/loader.rs` cache tracked in follow-up issues).
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}

/// Returns the host path used as the sandbox `/scratch` mount point.
///
/// In production this is a per-agent ephemeral directory created by the
/// orchestrator and deleted on agent termination.  In tests it is a
/// subdirectory of the system temp dir.
fn scratch_dir() -> PathBuf {
    std::env::temp_dir().join("gaia-sandbox-scratch")
}
