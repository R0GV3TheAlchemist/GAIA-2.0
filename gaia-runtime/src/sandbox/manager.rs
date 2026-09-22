//! [`SandboxManager`] — Wasmtime 46 + WASI 0.3 (p3) real isolation backend.
//!
//! # Store data layout
//!
//! `wasmtime_wasi::p3::add_to_linker` requires `T: WasiView`.
//! In wasmtime-wasi 46 `WasiView` has a single required method:
//!
//! ```text
//! fn ctx(&mut self) -> WasiCtxView<'_>
//! ```
//!
//! `WasiCtxView` is a struct literal with two fields:
//!   `{ ctx: &mut WasiCtx, table: &mut ResourceTable }`
//! where `ResourceTable` lives in `wasmtime::component`.
//!
//! `StoreData` bundles all three pieces so there is no unsafe memory leaking.

use std::path::PathBuf;

use wasmtime::{
    component::{
        Component,
        Linker,
        ResourceTable,
    },
    Config, Engine, Store,
};
use wasmtime_wasi::{
    DirPerms, FilePerms,
    WasiCtx, WasiCtxBuilder, WasiCtxView,
    WasiView,
};

use crate::sandbox::{
    error::{SandboxError, GAIA_CAPABILITY_DENIED},
    limits::GaiaResourceLimiter,
    profile::SandboxProfile,
};

// ── StoreData ────────────────────────────────────────────────────────────────

/// Data stored inside every Wasmtime `Store` created by this manager.
///
/// Implements [`WasiView`] so that `wasmtime_wasi::p3::add_to_linker` can
/// bind WASI 0.3 host functions against `Store<StoreData>`.
pub struct StoreData {
    /// WASI context (capability-granted I/O state).
    pub wasi:    WasiCtx,
    /// WASI resource table (open file descriptors, sockets, etc.).
    /// Lives in `wasmtime::component` in v46.
    pub table:   ResourceTable,
    /// Memory + table quota enforcer.
    pub limiter: GaiaResourceLimiter,
}

/// `WasiView` is required by `wasmtime_wasi::p3::add_to_linker<T>`.
///
/// In wasmtime-wasi 46 the trait has a single method returning `WasiCtxView`,
/// a struct literal that bundles `ctx` and `table` by mutable reference.
impl WasiView for StoreData {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx:   &mut self.wasi,
            table: &mut self.table,
        }
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
    pub fn new(profile: SandboxProfile) -> Result<Self, SandboxError> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.epoch_interruption(true);

        let engine = Engine::new(&config)
            .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;
        Ok(Self { engine, profile })
    }

    /// Build a WASI context that grants only the capabilities declared in the profile.
    ///
    /// # Errors
    ///
    /// Returns `SandboxError::CapabilityDenied` if a requested filesystem
    /// preopen cannot be established.
    pub fn build_wasi_ctx(&self) -> Result<WasiCtx, SandboxError> {
        let mut builder = WasiCtxBuilder::new();

        if self.profile.scratch_only_writes {
            let scratch = scratch_dir();
            std::fs::create_dir_all(&scratch).map_err(|e| {
                SandboxError::CapabilityDenied(format!(
                    "{GAIA_CAPABILITY_DENIED}: cannot create scratch dir: {e}"
                ))
            })?;
            builder
                .preopened_dir(&scratch, "/scratch", DirPerms::all(), FilePerms::all())
                .map_err(|e| {
                    SandboxError::CapabilityDenied(format!(
                        "{GAIA_CAPABILITY_DENIED}: preopened_dir failed: {e}"
                    ))
                })?;
        }

        if self.profile.inherited_env {
            builder.inherit_env();
        }

        Ok(builder.build())
    }

    /// Compile a WASM Component Model binary.
    pub fn compile(&self, bytes: &[u8]) -> Result<Component, SandboxError> {
        Component::new(&self.engine, bytes)
            .map_err(|e| SandboxError::CompileError(e.to_string()))
    }

    /// Execute a pre-compiled component inside the sandbox.
    ///
    /// Lifecycle:
    /// 1. Build a WASI context gated on the active [`SandboxProfile`].
    /// 2. Build `Store<StoreData>` with `GaiaResourceLimiter` + epoch deadline.
    /// 3. Add WASI 0.3 p3 host bindings via `wasmtime_wasi::p3::add_to_linker`.
    /// 4. Instantiate the component.
    /// 5. Call the `run` export if present; reactor-style components succeed
    ///    without a `run` export.
    ///
    /// Any Wasmtime error is classified by [`Self::classify_trap`].
    pub async fn execute_component(
        &self,
        component: &Component,
    ) -> Result<(), SandboxError> {
        let wasi = self.build_wasi_ctx()?;

        let store_data = StoreData {
            wasi,
            table:   ResourceTable::new(),
            limiter: GaiaResourceLimiter::new(self.profile.quota),
        };
        let mut store = Store::new(&self.engine, store_data);

        store.limiter(|data: &mut StoreData| &mut data.limiter);
        store.set_epoch_deadline(self.profile.quota.max_epochs);

        let mut linker: Linker<StoreData> = Linker::new(&self.engine);

        // Register WASI 0.3 (p3) host functions. Requires StoreData: WasiView.
        wasmtime_wasi::p3::add_to_linker(&mut linker)
            .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;

        // Instantiate the component.
        let instance = linker
            .instantiate_async(&mut store, component)
            .await
            .map_err(|e| Self::classify_trap(&e))?;

        // Invoke the canonical run export if the component exposes one.
        // Reactor-style components (no run export) succeed once instantiation
        // completes.
        let func = instance
            .get_func(&mut store, "run")
            .or_else(|| instance.get_func(&mut store, "wasi:cli/run@0.3.0#run"));

        if let Some(f) = func {
            f.call_async(&mut store, &[], &mut [])
                .await
                .map(|_| ())
                .map_err(|e| Self::classify_trap(&e))?;
        }

        Ok(())
    }

    /// Classify any error (Wasmtime, anyhow, or std) into a [`SandboxError`].
    ///
    /// Accepts `&dyn std::error::Error` so that:
    /// - Production call-sites pass `&wasmtime::Error` directly (it implements
    ///   `std::error::Error`).
    /// - Unit tests construct `anyhow::Error` values via `anyhow::anyhow!()`
    ///   and pass them without any conversion (anyhow also implements the trait).
    ///
    /// Classification is purely string-based so it remains stable across
    /// Wasmtime minor versions.
    pub fn classify_trap(err: &dyn std::error::Error) -> SandboxError {
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
