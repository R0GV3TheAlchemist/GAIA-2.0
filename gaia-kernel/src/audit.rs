use serde::{Deserialize, Serialize};

use crate::identity::{sha256_hex, Principal};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub seq: u64,
    pub prev_hash: String,
    pub payload_hash: String,
    pub op: String,
    pub principal_did: String,
    pub detail: String,
    pub signature_hex: String,
}

#[derive(Default)]
pub struct AuditLog {
    records: Vec<AuditRecord>,
}

impl AuditLog {
    pub fn append(&mut self, principal: &Principal, op: &str, detail: &str) -> AuditRecord {
        let prev = self
            .records
            .last()
            .map(|r| hash_record(r))
            .unwrap_or_else(|| "genesis".into());
        let payload = format!("{prev}|{op}|{detail}|{}", principal.did());
        let payload_hash = sha256_hex(payload.as_bytes());
        let signature_hex = hex::encode(principal.sign(payload.as_bytes()));
        let rec = AuditRecord {
            seq: self.records.len() as u64 + 1,
            prev_hash: prev,
            payload_hash,
            op: op.into(),
            principal_did: principal.did(),
            detail: detail.into(),
            signature_hex,
        };
        self.records.push(rec.clone());
        rec
    }

    pub fn prove(&self, op: &str, detail_contains: &str) -> Option<&AuditRecord> {
        self.records
            .iter()
            .find(|r| r.op == op && r.detail.contains(detail_contains))
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn chain_ok(&self) -> bool {
        let mut prev = "genesis".to_string();
        for r in &self.records {
            if r.prev_hash != prev {
                return false;
            }
            prev = hash_record(r);
        }
        true
    }
}

fn hash_record(r: &AuditRecord) -> String {
    sha256_hex(
        format!(
            "{}:{}:{}:{}",
            r.seq, r.payload_hash, r.op, r.signature_hex
        )
        .as_bytes(),
    )
}
