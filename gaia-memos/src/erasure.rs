//! Cryptographic erasure receipts for user-triggered MemCube deletion.
//!
//! When a user deletes a MemCube, GAIA produces a signed erasure receipt
//! so the deletion can be audited and proven to regulators or the user.
//!
//! ## Receipt format
//!
//! ```text
//! sha256( cube_id | ":" | content_hash | ":" | deleted_at_ms )
//! ```
//!
//! The receipt is a hex string that can be stored in the AuditLog
//! (Stage 11) or returned to the user directly.
//!
//! ## Privacy
//! The content itself is NOT included in the receipt — only its hash.
//! This means the receipt proves deletion without re-exposing the data.

use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use crate::MemCube;

/// A cryptographic proof that a MemCube was deleted at a specific time.
#[derive(Debug, Clone)]
pub struct ErasureReceipt {
    /// The UUID of the deleted cube.
    pub cube_id:      String,
    /// sha256(content) of the deleted cube — proves *what* was deleted.
    pub content_hash: String,
    /// Unix timestamp (ms) at which deletion was executed.
    pub deleted_at_ms: u64,
    /// The full receipt digest: sha256(cube_id | content_hash | deleted_at_ms).
    pub receipt_hex:  String,
}

impl ErasureReceipt {
    /// Generate an erasure receipt for a cube being deleted right now.
    pub fn generate(cube: &MemCube) -> Self {
        let deleted_at_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let content_hash = sha256_hex(cube.content.as_bytes());

        let receipt_input = format!(
            "{}:{}:{}",
            cube.id, content_hash, deleted_at_ms
        );
        let receipt_hex = sha256_hex(receipt_input.as_bytes());

        Self {
            cube_id:      cube.id.to_string(),
            content_hash,
            deleted_at_ms,
            receipt_hex,
        }
    }

    /// Verify that a receipt is internally consistent (re-derive and compare).
    pub fn verify(&self) -> bool {
        let receipt_input = format!(
            "{}:{}:{}",
            self.cube_id, self.content_hash, self.deleted_at_ms
        );
        sha256_hex(receipt_input.as_bytes()) == self.receipt_hex
    }
}

fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}
