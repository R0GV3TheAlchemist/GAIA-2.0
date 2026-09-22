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
pub struct StoreData {
    pub wasi:    WasiCtx,
    pub table:   ResourceTable,
    pub limiter: GaiaResourceLimiter,
}

impl WasiView for StoreData {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx:   &mut self.wasi,
            table: &mut self.table,
        }
    }
}

// ── SandboxManager ───────────────────────────────────────────────────────────

pub struct SandboxManager {
    engine:  Engine,
    profile: SandboxProfile,
}

impl SandboxManager {
    pub fn new(profile: SandboxProfile) -> Result<Self, SandboxError> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.epoch_interruption(true);

        let engine = Engine::new(&config)
            .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;
        Ok(Self { engine, profile })
    }

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

    pub fn compile(&self, bytes: &[u8]) -> Result<Component, SandboxError> {
        Component::new(&self.engine, bytes)
            .map_err(|e| SandboxError::CompileError(e.to_string()))
    }

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

        wasmtime_wasi::p3::add_to_linker(&mut linker)
            .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;

        // .to_string() converts wasmtime::Error without requiring StdError impl.
        let instance = linker
            .instantiate_async(&mut store, component)
            .await
            .map_err(|e| Self::classify_trap(&e.to_string()))?;

        let func = instance
            .get_func(&mut store, "run")
            .or_else(|| instance.get_func(&mut store, "wasi:cli/run@0.3.0#run"));

        if let Some(f) = func {
            f.call_async(&mut store, &[], &mut [])
                .await
                .map(|_| ())
                .map_err(|e| Self::classify_trap(&e.to_string()))?;
        }

        Ok(())
    }

    /// Classify an error message string into a [`SandboxError`].
    ///
    /// Accepts `&str` so it is independent of any error type's trait impls.
    /// Production call-sites convert via `.to_string()`. Unit tests do the
    /// same: `SandboxManager::classify_trap(&err.to_string())`.
    pub fn classify_trap(msg: &str) -> SandboxError {
        let lower = msg.to_lowercase();
        if lower.contains("out of memory") || lower.contains("oom") {
            SandboxError::OomTermination
        } else if lower.contains("epoch") || lower.contains("interrupt") {
            SandboxError::Timeout
        } else if lower.contains("eacces") || lower.contains("permission denied") {
            SandboxError::CapabilityDenied(GAIA_CAPABILITY_DENIED.to_owned())
        } else {
            SandboxError::Trap(msg.to_owned())
        }
    }

    pub fn profile(&self) -> &SandboxProfile {
        &self.profile
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}

fn scratch_dir() -> PathBuf {
    std::env::temp_dir().join("gaia-sandbox-scratch")
}
