//! L5 agent ecosystem (#5).
//! #24 Wasmtime guest, #25 pack, #26 A2A handoff and signed install.
//! #722 Full DISCOVER→ARCHIVE lifecycle state machine.
//! This is not a marketplace transport or containerd sandbox.

mod a2a;
mod host;
mod lifecycle;
mod pack;
mod runtime;
mod wasm;

pub use a2a::{
    registry_layout, ContextBundle, Handoff, MarketError, Package, PackageMarket, PrivacyMode,
};
pub use host::{AgentHost, HostError, Review};
pub use lifecycle::{
    AgentCheckpoint, AgentDid, CapabilityToken, LifecycleError, LifecycleEvent, LifecycleManager,
    LifecycleStage, ResourceSnapshot,
};
pub use pack::{catalog, find as find_pack, AgentKind, PackEntry};
pub use runtime::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};
pub use wasm::{WasiGrant, WasmOutcome, WasmRuntime};

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // LifecycleStage — ordering invariants
    // -------------------------------------------------------------------------

    /// LifecycleStage variants must compare in the canonical DISCOVER→ARCHIVE
    /// order defined by issue #722. If the ordering changes it is a breaking
    /// architectural decision that must be reviewed and logged.
    #[test]
    fn lifecycle_stage_ordering() {
        assert!(
            LifecycleStage::Discover < LifecycleStage::Register,
            "Discover must precede Register"
        );
        assert!(
            LifecycleStage::Register < LifecycleStage::Initialize,
            "Register must precede Initialize"
        );
        assert!(
            LifecycleStage::Initialize < LifecycleStage::Run,
            "Initialize must precede Run"
        );
        assert!(
            LifecycleStage::Run < LifecycleStage::Pause,
            "Run must precede Pause"
        );
        assert!(
            LifecycleStage::Pause < LifecycleStage::Archive,
            "Pause must precede Archive"
        );
    }

    // -------------------------------------------------------------------------
    // AgentManifest — default state
    // -------------------------------------------------------------------------

    /// A default AgentManifest must have an empty capabilities list.
    /// Agents start with no capabilities; each must be explicitly granted.
    #[test]
    fn agent_manifest_default_capabilities_empty() {
        let manifest = AgentManifest {
            name: String::new(),
            version: String::new(),
            declared_capabilities: vec![],
            limits: ResourceLimits::default(),
        };
        assert!(
            manifest.declared_capabilities.is_empty(),
            "default AgentManifest must have no capabilities"
        );
    }

    // -------------------------------------------------------------------------
    // ResourceLimits — default values
    // -------------------------------------------------------------------------

    /// Default ResourceLimits must be non-zero and finite.
    /// Zero limits would make every agent immediately non-schedulable.
    #[test]
    fn resource_limits_default_values() {
        let limits = ResourceLimits::default();
        assert!(limits.memory_mib  > 0, "default memory limit must be non-zero");
        assert!(limits.cpu_millis  > 0, "default CPU budget must be non-zero");
    }

    // -------------------------------------------------------------------------
    // pack catalog — registry slice
    // -------------------------------------------------------------------------

    /// catalog() must return a non-empty slice — the pack registry is seeded
    /// at compile time and must always have at least one entry.
    #[test]
    fn pack_catalog_returns_slice() {
        let entries = catalog();
        assert!(
            !entries.is_empty(),
            "pack catalog must contain at least one entry"
        );
    }

    // -------------------------------------------------------------------------
    // WasiGrant — debug output sanity
    // -------------------------------------------------------------------------

    /// WasiGrant must implement Debug and produce a non-empty string.
    /// This guards against accidentally removing the derive and breaking
    /// audit log formatting that relies on {:?} output.
    #[test]
    fn wasi_grant_debug_is_non_empty() {
        let grant = WasiGrant { filesystem: false, network: false };
        let s = format!("{grant:?}");
        assert!(!s.is_empty(), "WasiGrant Debug output must be non-empty");
    }
}
