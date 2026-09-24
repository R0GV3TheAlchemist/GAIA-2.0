//! GAIA kernel host — local-machine surface exposed to the execution engine.
//!
//! Provides:
//!  - `KernelHost`   — file store, memory cube, audit chain, signing
//!  - `KernelError`  — typed errors for host operations
//!  - `SignedBlob`   — wire type for detached signatures
//!  - `TaskHandle`   — lightweight reference to a running task

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::audit::{AuditChain, AuditRecord};
use crate::identity::{Principal, PrincipalKind};
use gaia_memos::MemoCube;
use gaia_sfs::SemanticFs;

// ── Error type ───────────────────────────────────────────────────────────────

#[derive(Debug, Error, PartialEq, Eq)]
pub enum KernelError {
    /// A required cryptographic signature was absent or invalid.
    #[error("GAIA_KERNEL_DENIED: {0}")]
    Denied(String),

    /// The requested path or resource was not found.
    #[error("GAIA_KERNEL_NOT_FOUND: {0}")]
    NotFound(String),

    /// An I/O or storage error occurred.
    #[error("GAIA_KERNEL_IO: {0}")]
    Io(String),

    /// A capability was registered with fewer than two resolution paths,
    /// creating a monopoly risk (Monopoly Architect adversarial scenario).
    #[error("GAIA_KERNEL_MONOPOLY_RISK: capability has fewer than two resolution paths")]
    MonopolyRisk,
}

// ── Wire types ───────────────────────────────────────────────────────────────

/// A detached cryptographic signature over an arbitrary byte payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedBlob {
    pub algorithm:     String,
    pub public_hex:    String,
    pub signature_hex: String,
}

/// Lightweight reference to a running task on this host.
#[derive(Debug, Clone)]
pub struct TaskHandle {
    pub id:   String,
    pub kind: String,
}

// ── Admitted intent record ────────────────────────────────────────────────────

#[derive(Debug)]
pub struct AdmittedIntent {
    pub state: String,
}

// ── KernelHost ───────────────────────────────────────────────────────────────

/// The local kernel host: file store, episodic memory, audit chain, signing.
pub struct KernelHost {
    pub sfs:   SemanticFs,
    pub audit: AuditChain,
    principal: Principal,
    _root:     PathBuf,
}

impl KernelHost {
    pub fn new(root: PathBuf) -> Result<Self, KernelError> {
        std::fs::create_dir_all(&root)
            .map_err(|e| KernelError::Io(e.to_string()))?;
        let sfs = SemanticFs::open(&root.join("sfs"))
            .map_err(|e| KernelError::Io(e.to_string()))?;
        let audit = AuditChain::open(&root.join("audit.db"))
            .map_err(|e| KernelError::Io(e.to_string()))?;
        let principal = Principal::generate(PrincipalKind::Node);
        Ok(Self { sfs, audit, principal, _root: root })
    }

    // ── Intent ────────────────────────────────────────────────────────────────

    pub fn intent(&mut self, _description: &str) -> Result<AdmittedIntent, KernelError> {
        Ok(AdmittedIntent { state: "admitted".into() })
    }

    // ── File store ────────────────────────────────────────────────────────────

    pub fn store_file(
        &mut self,
        path: &str,
        data: &[u8],
        description: &str,
    ) -> Result<StoredObject, KernelError> {
        let cid = self
            .sfs
            .put(path, data, description)
            .map_err(|e| KernelError::Io(e.to_string()))?;
        self.audit
            .push("sfs.put", path, &cid)
            .map_err(|e| KernelError::Io(e.to_string()))?;
        Ok(StoredObject { cid })
    }

    // ── Memory cube ───────────────────────────────────────────────────────────

    pub fn write_cube(&mut self, content: &str) -> Result<MemoCube, KernelError> {
        MemoCube::write(content).map_err(|e| KernelError::Io(e.to_string()))
    }

    pub fn context(&mut self, query: &str) -> Result<MemoCube, KernelError> {
        MemoCube::recall(query).map_err(|e| KernelError::Io(e.to_string()))
    }

    // ── Signing ───────────────────────────────────────────────────────────────

    pub fn sign(&self, data: &[u8]) -> Result<SignedBlob, KernelError> {
        let sig = self.principal.sign(data);
        Ok(SignedBlob {
            algorithm:     "ed25519".into(),
            public_hex:    self.principal.public_hex(),
            signature_hex: hex::encode(&sig),
        })
    }

    pub fn verify(&self, data: &[u8], blob: &SignedBlob) -> Result<bool, KernelError> {
        let sig_bytes = hex::decode(&blob.signature_hex)
            .map_err(|e| KernelError::Denied(e.to_string()))?;
        let pub_bytes = hex::decode(&blob.public_hex)
            .map_err(|e| KernelError::Denied(e.to_string()))?;
        if blob.public_hex != self.principal.public_hex() {
            return Ok(false);
        }
        Ok(self.principal.verify(data, &sig_bytes, &pub_bytes))
    }

    pub fn reject_unsigned(
        &self,
        blob: Option<&SignedBlob>,
        data: &[u8],
    ) -> Result<(), KernelError> {
        match blob {
            None => Err(KernelError::Denied("no signature provided".into())),
            Some(b) => {
                if b.public_hex != self.principal.public_hex() {
                    return Err(KernelError::Denied("foreign key".into()));
                }
                let ok = self.verify(data, b)?;
                if ok { Ok(()) } else { Err(KernelError::Denied("bad signature".into())) }
            }
        }
    }

    // ── Audit ─────────────────────────────────────────────────────────────────

    pub fn last_audit(&self, op: &str, key: &str) -> Option<AuditRecord> {
        self.audit.last(op, key)
    }

    // ── Capability stubs ──────────────────────────────────────────────────────

    pub fn invoke(&self, agent: &str, _payload: &str) -> Result<String, KernelError> {
        Ok(format!("invoked:{agent}"))
    }

    pub fn observe(&self, sensor: &str) -> Result<String, KernelError> {
        Ok(format!("observed:{sensor}"))
    }

    pub fn declare(&self, name: &str, kind: &str) -> Result<DeclaredResource, KernelError> {
        Ok(DeclaredResource { name: name.into(), kind: kind.into() })
    }
}

// ── Small value types ─────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct StoredObject {
    pub cid: String,
}

#[derive(Debug)]
pub struct DeclaredResource {
    pub name: String,
    pub kind: String,
}
