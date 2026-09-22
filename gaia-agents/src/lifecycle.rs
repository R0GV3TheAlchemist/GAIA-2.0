//! Full agent lifecycle — #722.
//!
//! Implements the 13-stage state machine:
//!
//!   DISCOVER → REGISTER → ATTEST → LOAD → INITIALIZE → GRANT
//!   → RUN → MONITOR → PAUSE → RESUME → REVOKE → TERMINATE → ARCHIVE
//!
//! # Identity (#724 hook)
//!
//! `AgentIdentity` uses a stub DID (`did:gaia:<uuid>`) until #724 lands.
//! Replace `AgentIdentity::stub()` with `gaia_identity::Identity::resolve(did)`
//! and the token HMAC key with the agent's Ed25519 signing key.
//!
//! # Runtime (#719)
//!
//! The WASM binary attestation in `LifecycleManager::attest` currently
//! computes SHA-256 over the bytes slice supplied by the caller.  When
//! the real runtime sandbox is wired, replace the `bytes` parameter with
//! a call to `gaia_runtime::Sandbox::loaded_binary_hash(agent_id)`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::runtime::{AgentManifest, Capability};

// ── Types ──────────────────────────────────────────────────────────────────────

/// Stub DID.  Replace with `gaia_identity::Did` when #724 merges.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentDid(pub String);

impl AgentDid {
    /// Generate a stub DID from a UUID.
    pub fn stub(id: Uuid) -> Self {
        Self(format!("did:gaia:{id}"))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The 13 lifecycle stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleStage {
    Discover,
    Register,
    Attest,
    Load,
    Initialize,
    Grant,
    Run,
    Monitor,
    Pause,
    Resume,
    Revoke,
    Terminate,
    Archive,
}

impl std::fmt::Display for LifecycleStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Discover    => "DISCOVER",
            Self::Register    => "REGISTER",
            Self::Attest      => "ATTEST",
            Self::Load        => "LOAD",
            Self::Initialize  => "INITIALIZE",
            Self::Grant       => "GRANT",
            Self::Run         => "RUN",
            Self::Monitor     => "MONITOR",
            Self::Pause       => "PAUSE",
            Self::Resume      => "RESUME",
            Self::Revoke      => "REVOKE",
            Self::Terminate   => "TERMINATE",
            Self::Archive     => "ARCHIVE",
        };
        write!(f, "{s}")
    }
}

/// Error variants for lifecycle transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidTransition { from: LifecycleStage, to: LifecycleStage },
    AttestationFailed { agent: String, reason: String },
    QuotaExceeded { agent: String, reason: String },
    TokenExpired { token_id: Uuid },
    TokenRevoked { token_id: Uuid },
    AgentNotFound(Uuid),
    AgentRevoked(Uuid),
    CheckpointNotFound(Uuid),
    RegistryError(String),
}

impl std::fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTransition { from, to } =>
                write!(f, "invalid transition {from} → {to}"),
            Self::AttestationFailed { agent, reason } =>
                write!(f, "attestation failed for {agent}: {reason}"),
            Self::QuotaExceeded { agent, reason } =>
                write!(f, "quota exceeded for {agent}: {reason}"),
            Self::TokenExpired { token_id } =>
                write!(f, "capability token {token_id} has expired"),
            Self::TokenRevoked { token_id } =>
                write!(f, "capability token {token_id} has been revoked"),
            Self::AgentNotFound(id) => write!(f, "agent {id} not found"),
            Self::AgentRevoked(id) => write!(f, "agent {id} is revoked"),
            Self::CheckpointNotFound(id) => write!(f, "no checkpoint for agent {id}"),
            Self::RegistryError(msg) => write!(f, "registry error: {msg}"),
        }
    }
}

/// A scoped, time-limited capability token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: Uuid,
    pub agent_did: AgentDid,
    pub capabilities: Vec<Capability>,
    /// Epoch ms after which the token is invalid (None = no expiry in tests).
    pub expires_at_ms: Option<u64>,
    /// HMAC-SHA256 over `id || did || capabilities` — stub key for now.
    pub hmac: Vec<u8>,
    pub revoked: bool,
}

impl CapabilityToken {
    /// Issue a new token.  `ttl_ms` is time-to-live in milliseconds.
    pub fn issue(
        agent_did: AgentDid,
        capabilities: Vec<Capability>,
        ttl_ms: Option<u64>,
        now_ms: u64,
    ) -> Self {
        let id = Uuid::new_v4();
        let expires_at_ms = ttl_ms.map(|ttl| now_ms + ttl);
        let hmac = Self::compute_hmac(&id, &agent_did, &capabilities);
        Self {
            id,
            agent_did,
            capabilities,
            expires_at_ms,
            hmac,
            revoked: false,
        }
    }

    pub fn is_valid(&self, now_ms: u64) -> bool {
        if self.revoked {
            return false;
        }
        if let Some(exp) = self.expires_at_ms {
            if now_ms >= exp {
                return false;
            }
        }
        let expected = Self::compute_hmac(&self.id, &self.agent_did, &self.capabilities);
        self.hmac == expected
    }

    fn compute_hmac(id: &Uuid, did: &AgentDid, caps: &[Capability]) -> Vec<u8> {
        let mut h = Sha256::new();
        h.update(id.as_bytes());
        h.update(did.as_str().as_bytes());
        let mut cap_names: Vec<String> = caps
            .iter()
            .map(|c| format!("{c:?}"))
            .collect();
        cap_names.sort();
        h.update(cap_names.join("|").as_bytes());
        h.finalize().to_vec()
    }
}

/// Serialisable checkpoint for pause/resume and node migration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentCheckpoint {
    pub agent_id: Uuid,
    pub stage: LifecycleStage,
    pub manifest: AgentManifest,
    /// Opaque state blob (WASM linear-memory snapshot in production).
    pub state_blob: Vec<u8>,
    pub saved_at_ms: u64,
}

/// Lifecycle audit record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LifecycleEvent {
    pub agent_id: Uuid,
    pub from_stage: Option<LifecycleStage>,
    pub to_stage: LifecycleStage,
    pub note: String,
}

/// Resource telemetry snapshot (MONITOR stage).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceSnapshot {
    pub agent_id: Uuid,
    pub cpu_millis_used: u64,
    pub memory_mib_used: u64,
    pub network_bytes_out: u64,
}

/// Identity record for a registered agent.
#[derive(Debug, Clone)]
pub struct AgentRecord {
    pub id: Uuid,
    pub did: AgentDid,
    pub manifest: AgentManifest,
    pub stage: LifecycleStage,
    /// SHA-256 of the attested WASM binary.
    pub attested_hash: Option<[u8; 32]>,
    pub tokens: Vec<CapabilityToken>,
    pub checkpoint: Option<AgentCheckpoint>,
    pub revoked: bool,
    pub telemetry: Vec<ResourceSnapshot>,
}

impl AgentRecord {
    fn new(id: Uuid, manifest: AgentManifest) -> Self {
        Self {
            did: AgentDid::stub(id),
            id,
            manifest,
            stage: LifecycleStage::Discover,
            attested_hash: None,
            tokens: vec![],
            checkpoint: None,
            revoked: false,
            telemetry: vec![],
        }
    }
}

// ── Transition table ───────────────────────────────────────────────────────────

fn valid_transition(from: LifecycleStage, to: LifecycleStage) -> bool {
    use LifecycleStage::*;
    matches!(
        (from, to),
        (Discover,    Register)  |
        (Register,    Attest)    |
        (Attest,      Load)      |
        (Load,        Initialize)|
        (Initialize,  Grant)     |
        (Grant,       Run)       |
        (Run,         Monitor)   |
        (Monitor,     Run)       |  // continuous monitor loop
        (Run,         Pause)     |
        (Pause,       Resume)    |
        (Resume,      Run)       |
        (Run,         Revoke)    |
        (Monitor,     Revoke)    |
        (Pause,       Revoke)    |
        (Revoke,      Terminate) |
        (Run,         Terminate) |
        (Monitor,     Terminate) |
        (Pause,       Terminate) |
        (Terminate,   Archive)
    )
}

// ── LifecycleManager ──────────────────────────────────────────────────────────

/// Manages the full lifecycle for a set of agents.
pub struct LifecycleManager {
    pub agents: HashMap<Uuid, AgentRecord>,
    audit_log: Vec<LifecycleEvent>,
    /// Monotonic clock substitute used in tests.
    pub now_ms: u64,
    /// Max CPU ms enforced at GRANT.
    pub policy_max_cpu_millis: u64,
    /// Max memory MiB enforced at GRANT.
    pub policy_max_memory_mib: u64,
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self {
            agents: HashMap::new(),
            audit_log: vec![],
            now_ms: 0,
            policy_max_cpu_millis: 10_000,
            policy_max_memory_mib: 512,
        }
    }
}

impl LifecycleManager {
    // ── DISCOVER ──────────────────────────────────────────────────────────────

    pub fn discover(&mut self, manifest: AgentManifest) -> Uuid {
        let id = Uuid::new_v4();
        let record = AgentRecord::new(id, manifest);
        self.emit_event(id, None, LifecycleStage::Discover, "discovered");
        self.agents.insert(id, record);
        id
    }

    // ── REGISTER ─────────────────────────────────────────────────────────────

    pub fn register(&mut self, id: Uuid) -> Result<(), LifecycleError> {
        let from = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?.stage;
        self.transition(id, from, LifecycleStage::Register, "manifest registered")
    }

    // ── ATTEST ────────────────────────────────────────────────────────────────

    pub fn attest(&mut self, id: Uuid, wasm_bytes: &[u8]) -> Result<[u8; 32], LifecycleError> {
        let from = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?.stage;
        let agent_name = self.agents[&id].manifest.name.clone();

        let mut h = Sha256::new();
        h.update(wasm_bytes);
        let hash: [u8; 32] = h.finalize().into();

        if hash == [0u8; 32] {
            return Err(LifecycleError::AttestationFailed {
                agent: agent_name,
                reason: "binary hash is all-zero (tampered or empty)".into(),
            });
        }

        self.transition(id, from, LifecycleStage::Attest, "wasm attested")?;
        self.agents.get_mut(&id).unwrap().attested_hash = Some(hash);
        Ok(hash)
    }

    pub fn verify_attestation(&self, id: Uuid, wasm_bytes: &[u8]) -> Result<(), LifecycleError> {
        let record = self.agents.get(&id).ok_or(LifecycleError::AgentNotFound(id))?;
        let stored = record.attested_hash.ok_or_else(|| LifecycleError::AttestationFailed {
            agent: record.manifest.name.clone(),
            reason: "no attestation recorded".into(),
        })?;
        let mut h = Sha256::new();
        h.update(wasm_bytes);
        let hash: [u8; 32] = h.finalize().into();
        if hash != stored {
            return Err(LifecycleError::AttestationFailed {
                agent: record.manifest.name.clone(),
                reason: "binary hash does not match attested value".into(),
            });
        }
        Ok(())
    }

    // ── LOAD ──────────────────────────────────────────────────────────────────

    pub fn load(&mut self, id: Uuid) -> Result<(), LifecycleError> {
        let from = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?.stage;
        self.transition(id, from, LifecycleStage::Load, "wasm loaded into sandbox")
    }

    // ── INITIALIZE ────────────────────────────────────────────────────────────

    pub fn initialize(&mut self, id: Uuid) -> Result<(), LifecycleError> {
        let from = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?.stage;
        self.transition(id, from, LifecycleStage::Initialize, "agent initialized")
    }

    // ── GRANT ─────────────────────────────────────────────────────────────────

    pub fn grant(
        &mut self,
        id: Uuid,
        capabilities: Vec<Capability>,
        ttl_ms: Option<u64>,
    ) -> Result<CapabilityToken, LifecycleError> {
        // Copy scalars before any borrow of `self.agents`.
        let max_cpu = self.policy_max_cpu_millis;
        let max_mem = self.policy_max_memory_mib;
        let now = self.now_ms;

        {
            let record = self.agents.get(&id).ok_or(LifecycleError::AgentNotFound(id))?;
            for cap in &capabilities {
                if !record.manifest.declared_capabilities.contains(cap) {
                    return Err(LifecycleError::QuotaExceeded {
                        agent: record.manifest.name.clone(),
                        reason: format!("capability {cap:?} not declared in manifest"),
                    });
                }
            }
            if record.manifest.limits.cpu_millis > max_cpu {
                return Err(LifecycleError::QuotaExceeded {
                    agent: record.manifest.name.clone(),
                    reason: "cpu limit exceeds policy".into(),
                });
            }
            if record.manifest.limits.memory_mib > max_mem {
                return Err(LifecycleError::QuotaExceeded {
                    agent: record.manifest.name.clone(),
                    reason: "memory limit exceeds policy".into(),
                });
            }
        }

        let from = self.agents[&id].stage;
        self.transition(id, from, LifecycleStage::Grant, "capability token issued")?;

        let did = self.agents[&id].did.clone();
        let token = CapabilityToken::issue(did, capabilities, ttl_ms, now);
        self.agents.get_mut(&id).unwrap().tokens.push(token.clone());
        Ok(token)
    }

    // ── RUN ───────────────────────────────────────────────────────────────────

    pub fn run(&mut self, id: Uuid) -> Result<(), LifecycleError> {
        // Copy `now_ms` before borrowing agents.
        let now = self.now_ms;
        {
            let record = self.agents.get(&id).ok_or(LifecycleError::AgentNotFound(id))?;
            if record.revoked {
                return Err(LifecycleError::AgentRevoked(id));
            }
            let has_valid = record.tokens.iter().any(|t| t.is_valid(now));
            if !has_valid {
                return Err(LifecycleError::TokenExpired {
                    token_id: record.tokens.first().map(|t| t.id).unwrap_or_else(Uuid::nil),
                });
            }
        }
        let from = self.agents[&id].stage;
        self.transition(id, from, LifecycleStage::Run, "agent running")
    }

    // ── MONITOR ───────────────────────────────────────────────────────────────

    pub fn monitor(
        &mut self,
        id: Uuid,
        snapshot: ResourceSnapshot,
    ) -> Result<(), LifecycleError> {
        let mem_limit = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?
            .manifest.limits.memory_mib;
        if snapshot.memory_mib_used > mem_limit {
            let agent_name = self.agents[&id].manifest.name.clone();
            let from = self.agents[&id].stage;
            let used = snapshot.memory_mib_used;
            let _ = self.transition(
                id, from, LifecycleStage::Terminate,
                &format!("OOM: used {used} MiB, limit {mem_limit} MiB"),
            );
            return Err(LifecycleError::QuotaExceeded {
                agent: agent_name,
                reason: format!("OOM: used {used} MiB exceeds limit"),
            });
        }
        let from = self.agents[&id].stage;
        self.agents.get_mut(&id).unwrap().telemetry.push(snapshot);
        self.transition(id, from, LifecycleStage::Monitor, "telemetry recorded")
    }

    /// Convenience: cycle MONITOR → RUN (used in tests after a monitor call).
    pub fn transition_to_run_from_monitor(&mut self, id: Uuid) -> Result<(), LifecycleError> {
        self.transition(id, LifecycleStage::Monitor, LifecycleStage::Run, "monitor → run")
    }

    // ── PAUSE ─────────────────────────────────────────────────────────────────

    pub fn pause(
        &mut self,
        id: Uuid,
        state_blob: Vec<u8>,
    ) -> Result<AgentCheckpoint, LifecycleError> {
        let from = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?.stage;
        self.transition(id, from, LifecycleStage::Pause, "agent paused")?;
        let now = self.now_ms;
        let record = self.agents.get(&id).unwrap();
        let checkpoint = AgentCheckpoint {
            agent_id: id,
            stage: LifecycleStage::Pause,
            manifest: record.manifest.clone(),
            state_blob,
            saved_at_ms: now,
        };
        self.agents.get_mut(&id).unwrap().checkpoint = Some(checkpoint.clone());
        Ok(checkpoint)
    }

    // ── RESUME ────────────────────────────────────────────────────────────────

    pub fn resume(&mut self, id: Uuid) -> Result<AgentCheckpoint, LifecycleError> {
        let (from, checkpoint) = {
            let record = self.agents.get(&id).ok_or(LifecycleError::AgentNotFound(id))?;
            let cp = record
                .checkpoint
                .clone()
                .ok_or(LifecycleError::CheckpointNotFound(id))?;
            (record.stage, cp)
        };
        self.transition(id, from, LifecycleStage::Resume, "checkpoint restored")?;
        self.transition(id, LifecycleStage::Resume, LifecycleStage::Run, "resumed → running")?;
        Ok(checkpoint)
    }

    // ── REVOKE ────────────────────────────────────────────────────────────────

    pub fn revoke(&mut self, id: Uuid) -> Result<(), LifecycleError> {
        let from = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?.stage;
        self.transition(id, from, LifecycleStage::Revoke, "capabilities revoked")?;
        let record = self.agents.get_mut(&id).unwrap();
        for token in &mut record.tokens {
            token.revoked = true;
        }
        record.revoked = true;
        Ok(())
    }

    // ── TERMINATE ─────────────────────────────────────────────────────────────

    pub fn terminate(&mut self, id: Uuid) -> Result<(), LifecycleError> {
        let from = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?.stage;
        self.transition(id, from, LifecycleStage::Terminate, "agent terminated")
    }

    // ── ARCHIVE ───────────────────────────────────────────────────────────────

    pub fn archive(&mut self, id: Uuid) -> Result<(), LifecycleError> {
        let from = self.agents.get(&id)
            .ok_or(LifecycleError::AgentNotFound(id))?.stage;
        self.transition(id, from, LifecycleStage::Archive, "archived to audit log")
    }

    // ── Queries ───────────────────────────────────────────────────────────────

    pub fn stage_of(&self, id: Uuid) -> Option<LifecycleStage> {
        self.agents.get(&id).map(|r| r.stage)
    }

    pub fn audit_log(&self) -> &[LifecycleEvent] {
        &self.audit_log
    }

    pub fn token_for(&self, id: Uuid, token_id: Uuid) -> Option<&CapabilityToken> {
        self.agents
            .get(&id)?
            .tokens
            .iter()
            .find(|t| t.id == token_id)
    }

    pub fn is_revoked(&self, id: Uuid) -> bool {
        self.agents.get(&id).map_or(false, |r| r.revoked)
    }

    pub fn telemetry(&self, id: Uuid) -> &[ResourceSnapshot] {
        self.agents
            .get(&id)
            .map(|r| r.telemetry.as_slice())
            .unwrap_or_default()
    }

    // ── Internals ─────────────────────────────────────────────────────────────

    fn transition(
        &mut self,
        id: Uuid,
        from: LifecycleStage,
        to: LifecycleStage,
        note: &str,
    ) -> Result<(), LifecycleError> {
        if !valid_transition(from, to) {
            return Err(LifecycleError::InvalidTransition { from, to });
        }
        self.agents.get_mut(&id).unwrap().stage = to;
        self.emit_event(id, Some(from), to, note);
        Ok(())
    }

    fn emit_event(
        &mut self,
        agent_id: Uuid,
        from_stage: Option<LifecycleStage>,
        to_stage: LifecycleStage,
        note: &str,
    ) {
        self.audit_log.push(LifecycleEvent {
            agent_id,
            from_stage,
            to_stage,
            note: note.to_string(),
        });
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::runtime::{AgentManifest, Capability, ResourceLimits};

    fn test_manifest(name: &str) -> AgentManifest {
        AgentManifest {
            name: name.into(),
            version: "0.1.0".into(),
            declared_capabilities: vec![Capability::MemoryRead, Capability::FilesystemRead],
            limits: ResourceLimits {
                cpu_millis: 1_000,
                memory_mib: 64,
                network_allowed: false,
                filesystem_allowed: false,
            },
        }
    }

    fn full_lifecycle(mgr: &mut LifecycleManager, wasm: &[u8]) -> Uuid {
        let id = mgr.discover(test_manifest("test-agent"));
        mgr.register(id).unwrap();
        mgr.attest(id, wasm).unwrap();
        mgr.load(id).unwrap();
        mgr.initialize(id).unwrap();
        mgr.grant(id, vec![Capability::MemoryRead], Some(60_000)).unwrap();
        mgr.run(id).unwrap();
        id
    }

    #[test]
    fn full_lifecycle_discover_to_archive() {
        let mut mgr = LifecycleManager::default();
        let id = full_lifecycle(&mut mgr, b"wasm");

        let snap = ResourceSnapshot {
            agent_id: id,
            cpu_millis_used: 100,
            memory_mib_used: 10,
            network_bytes_out: 0,
        };
        mgr.monitor(id, snap).unwrap();
        mgr.transition_to_run_from_monitor(id).unwrap();
        mgr.pause(id, b"state".to_vec()).unwrap();
        mgr.resume(id).unwrap();
        mgr.revoke(id).unwrap();
        mgr.terminate(id).unwrap();
        mgr.archive(id).unwrap();

        assert_eq!(mgr.stage_of(id), Some(LifecycleStage::Archive));
        let stages: Vec<LifecycleStage> =
            mgr.audit_log().iter().map(|e| e.to_stage).collect();
        for expected in [
            LifecycleStage::Discover, LifecycleStage::Register,
            LifecycleStage::Attest, LifecycleStage::Load,
            LifecycleStage::Initialize, LifecycleStage::Grant,
            LifecycleStage::Run, LifecycleStage::Monitor,
            LifecycleStage::Pause, LifecycleStage::Resume,
            LifecycleStage::Revoke, LifecycleStage::Terminate,
            LifecycleStage::Archive,
        ] {
            assert!(
                stages.contains(&expected),
                "audit log missing stage {expected}"
            );
        }
    }

    #[test]
    fn revocation_completes_well_under_100ms() {
        let mut mgr = LifecycleManager::default();
        let id = full_lifecycle(&mut mgr, b"wasm");
        let t0 = Instant::now();
        mgr.revoke(id).unwrap();
        let elapsed = t0.elapsed();
        assert!(
            elapsed < Duration::from_millis(100),
            "revoke took {:?} (must be < 100ms)",
            elapsed
        );
        assert!(mgr.is_revoked(id));
    }

    #[test]
    fn revoked_tokens_are_invalid() {
        let mut mgr = LifecycleManager::default();
        let id = full_lifecycle(&mut mgr, b"wasm");
        let token_id = mgr.agents[&id].tokens[0].id;
        mgr.revoke(id).unwrap();
        let token = mgr.token_for(id, token_id).unwrap();
        assert!(!token.is_valid(mgr.now_ms));
    }

    #[test]
    fn run_after_revoke_is_rejected() {
        let mut mgr = LifecycleManager::default();
        let id = full_lifecycle(&mut mgr, b"wasm");
        mgr.revoke(id).unwrap();
        mgr.terminate(id).unwrap();
        let id2 = full_lifecycle(&mut mgr, b"wasm");
        mgr.revoke(id2).unwrap();
        assert!(matches!(
            mgr.run(id2),
            Err(LifecycleError::AgentRevoked(_))
        ));
    }

    #[test]
    fn attestation_rejects_tampered_module() {
        let mut mgr = LifecycleManager::default();
        let id = mgr.discover(test_manifest("tamper-test"));
        mgr.register(id).unwrap();
        mgr.attest(id, b"original wasm bytes").unwrap();
        let err = mgr
            .verify_attestation(id, b"tampered wasm bytes")
            .unwrap_err();
        assert!(matches!(err, LifecycleError::AttestationFailed { .. }));
    }

    #[test]
    fn attestation_accepts_original_module() {
        let mut mgr = LifecycleManager::default();
        let id = mgr.discover(test_manifest("attest-ok"));
        mgr.register(id).unwrap();
        mgr.attest(id, b"real wasm bytes").unwrap();
        assert!(mgr.verify_attestation(id, b"real wasm bytes").is_ok());
    }

    #[test]
    fn checkpoint_round_trips_state_blob() {
        let mut mgr = LifecycleManager::default();
        let id = full_lifecycle(&mut mgr, b"wasm");
        let state = b"memory snapshot bytes";
        let cp = mgr.pause(id, state.to_vec()).unwrap();
        assert_eq!(cp.state_blob, state);
        let restored = mgr.resume(id).unwrap();
        assert_eq!(restored.state_blob, state);
        assert_eq!(mgr.stage_of(id), Some(LifecycleStage::Run));
    }

    #[test]
    fn resume_without_checkpoint_fails() {
        let mut mgr = LifecycleManager::default();
        let id = mgr.discover(test_manifest("no-cp"));
        mgr.register(id).unwrap();
        mgr.agents.get_mut(&id).unwrap().stage = LifecycleStage::Pause;
        assert!(matches!(
            mgr.resume(id),
            Err(LifecycleError::CheckpointNotFound(_))
        ));
    }

    #[test]
    fn oom_triggers_clean_terminate_not_panic() {
        let mut mgr = LifecycleManager::default();
        let id = full_lifecycle(&mut mgr, b"wasm");
        let snap = ResourceSnapshot {
            agent_id: id,
            cpu_millis_used: 100,
            memory_mib_used: 65,
            network_bytes_out: 0,
        };
        let err = mgr.monitor(id, snap).unwrap_err();
        assert!(matches!(err, LifecycleError::QuotaExceeded { .. }));
        assert_eq!(mgr.stage_of(id), Some(LifecycleStage::Terminate));
    }

    #[test]
    fn every_transition_appears_in_audit_log() {
        let mut mgr = LifecycleManager::default();
        let id = full_lifecycle(&mut mgr, b"wasm");
        mgr.terminate(id).unwrap();
        mgr.archive(id).unwrap();
        let count = mgr.audit_log().iter().filter(|e| e.agent_id == id).count();
        assert!(count >= 9, "expected ≥ 9 audit events, got {count}");
    }

    #[test]
    fn invalid_transition_returns_error() {
        let mut mgr = LifecycleManager::default();
        let id = mgr.discover(test_manifest("t-test"));
        assert!(matches!(
            mgr.run(id),
            Err(LifecycleError::InvalidTransition { .. })
        ));
    }

    #[test]
    fn expired_token_blocks_run() {
        let mut mgr = LifecycleManager { now_ms: 1_000, ..Default::default() };
        let id = mgr.discover(test_manifest("expiry-test"));
        mgr.register(id).unwrap();
        mgr.attest(id, b"wasm").unwrap();
        mgr.load(id).unwrap();
        mgr.initialize(id).unwrap();
        mgr.grant(id, vec![Capability::MemoryRead], Some(500)).unwrap();
        mgr.now_ms = 2_000;
        let err = mgr.run(id).unwrap_err();
        assert!(matches!(err, LifecycleError::TokenExpired { .. }));
    }

    #[test]
    fn undeclared_capability_rejected_at_grant() {
        let mut mgr = LifecycleManager::default();
        let id = mgr.discover(test_manifest("cap-test"));
        mgr.register(id).unwrap();
        mgr.attest(id, b"wasm").unwrap();
        mgr.load(id).unwrap();
        mgr.initialize(id).unwrap();
        let err = mgr
            .grant(id, vec![Capability::Network], Some(60_000))
            .unwrap_err();
        assert!(matches!(err, LifecycleError::QuotaExceeded { .. }));
    }

    #[test]
    fn tampered_token_hmac_is_invalid() {
        let mut mgr = LifecycleManager::default();
        let id = full_lifecycle(&mut mgr, b"wasm");
        let token_id = mgr.agents[&id].tokens[0].id;
        mgr.agents.get_mut(&id).unwrap().tokens[0].hmac = vec![0u8; 32];
        let token = mgr.token_for(id, token_id).unwrap();
        assert!(!token.is_valid(mgr.now_ms));
    }
}
