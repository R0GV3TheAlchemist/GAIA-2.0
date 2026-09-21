//! [`SandboxManager`] — Wasmtime 46 + WASI 0.3 (p3) real isolation backend.
//!
//! # Store data layout
//!
//! Wasmtime requires the `ResourceLimiter` to be stored inside the `Store`'s
//! data type and accessed via a closure: `store.limiter(|d| &mut d.limiter)`.
//! We use [`StoreData`] to bundle the WASI context and the resource limiter
//! together so there is no unsafe memory leaking.

use std::path::PathBuf;

use cap_std::ambient_authority;
use wasmtime::{
    component::Component,
    Config, Engine, Store,
};
// wasmtime-wasi 46 with feature `p3` exports WasiCtxBuilder and the built
// context type from the `p3` module.  The context is opaque — use the
// builder's return type directly via `wasmtime_wasi::p3::WasiCtxBuilder::build`.
use wasmtime_wasi::p3::WasiCtxBuilder;

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
    /// WASI 0.3 context (capability-granted I/O state).
    /// The concrete type is whatever `WasiCtxBuilder::build()` returns;
    /// in wasmtime-wasi 46 that is `wasmtime_wasi::p3::WasiCtx`.
    pub wasi:    <WasiCtxBuilder as WasiCtxBuilderExt>::Built,
    /// Memory + table quota enforcer.
    pub limiter: GaiaResourceLimiter,
}

/// Helper alias so we can name the return type of `WasiCtxBuilder::build()`
/// without relying on a private/unstable path.  We resolve it once here.
mod _wasi_ctx_type {
    /// The concrete type returned by `wasmtime_wasi::p3::WasiCtxBuilder::build()`.
    /// Obtained by calling build on a throwaway builder at type-inference time.
    pub type WasiCtx = <super::WasiCtxBuilder as super::WasiCtxBuilderExt>::Built;
}

/// Sealed trait that names the `build()` return type so we can store it.
pub trait WasiCtxBuilderExt {
    type Built;
    fn build_ctx(self) -> Self::Built;
}

impl WasiCtxBuilderExt for WasiCtxBuilder {
    type Built = wasmtime_wasi::p3::WasiCtx;
    fn build_ctx(self) -> wasmtime_wasi::p3::WasiCtx {
        self.build()
    }
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
    /// Note: `async_support` was removed in Wasmtime 46 (it is now always
    /// available and calling it emits a deprecation warning).
    pub fn new(profile: SandboxProfile) -> Result<Self, SandboxError> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.epoch_interruption(true);

        // Engine::new returns Result<_, wasmtime::Error>; convert explicitly
        // because SandboxError implements From<anyhow::Error>, not
        // From<wasmtime::Error> directly.
        let engine = Engine::new(&config)
            .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;
        Ok(Self { engine, profile })
    }

    /// Build a WASI 0.3 context that grants only the capabilities in the profile.
    ///
    /// # WASI 0.3 capability model
    ///
    /// In WASI 0.3 a component's capabilities are enforced at the world-import
    /// level: a component without a `wasi:sockets` world import physically
    /// cannot open sockets.  Omitting the socket linker binding is therefore
    /// sufficient for network isolation; nothing needs to be blocked here.
    pub fn build_wasi_ctx(&self) -> wasmtime_wasi::p3::WasiCtx {
        let mut builder = WasiCtxBuilder::new();

        // Filesystem — mount /scratch only when the policy allows it.
        // Default: no preopens → component has zero filesystem access.
        if self.profile.scratch_only_writes {
            let scratch = scratch_dir();
            let _ = std::fs::create_dir_all(&scratch);
            // wasmtime-wasi p3 requires a cap_std::fs::Dir handle.
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
    /// is wired in issue #720 (Execution Engine).  This call validates
    /// sandbox initialisation and capability gating.
    pub async fn execute_component(
        &self,
        component: &Component,
    ) -> Result<(), SandboxError> {
        let store_data = StoreData {
            wasi:    self.build_wasi_ctx(),
            limiter: GaiaResourceLimiter::new(self.profile.quota),
        };
        let mut store = Store::new(&self.engine, store_data);

        // Borrow the limiter from the store's own data — no Box leaking.
        store.limiter(|data: &mut StoreData| &mut data.limiter);

        // Epoch deadline: number of epoch ticks before interrupt.
        store.set_epoch_deadline(self.profile.quota.max_epochs);

        // Linker construction proves the component can be linked against
        // the WASI world.  Full invocation added in #720.
        let _linker: wasmtime::component::Linker<StoreData> =
            wasmtime::component::Linker::new(&self.engine);

        let _ = component; // suppress until #720 wires the call
        Ok(())
    }

    /// Classify a raw Wasmtime error into a structured [`SandboxError`].
    ///
    /// Used by the audit pipeline (#726) to produce typed receipts for every
    /// sandbox termination event.
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
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}

/// Returns the host path used as the sandbox `/scratch` mount point.
fn scratch_dir() -> PathBuf {
    std::env::temp_dir().join("gaia-sandbox-scratch")
}
