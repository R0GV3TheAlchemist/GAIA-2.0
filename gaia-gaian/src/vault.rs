//! #65 local vault stub. Not libsodium, DID, or Ed25519.

use crate::GaianError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub event: String,
}

#[derive(Debug, Default)]
pub struct Vault {
    embeddings: Vec<String>,
    audit: Vec<AuditEntry>,
    cloud_sync: bool,
}

impl Vault {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn act(&mut self, signed: bool, label: &str) -> Result<(), GaianError> {
        if !signed {
            self.audit.push(AuditEntry {
                event: "unsigned rejected".into(),
            });
            return Err(GaianError::Unsigned);
        }
        self.embeddings.push(label.into());
        self.audit.push(AuditEntry {
            event: format!("stored {label}"),
        });
        Ok(())
    }

    pub fn enable_cloud_sync(&mut self, opt_in: bool) {
        self.cloud_sync = opt_in;
        self.audit.push(AuditEntry {
            event: format!("cloud-sync={opt_in}"),
        });
    }

    pub fn inspect_audit(&self) -> &[AuditEntry] {
        &self.audit
    }

    pub fn embeddings(&self) -> &[String] {
        &self.embeddings
    }

    pub fn wipe(&mut self) -> String {
        self.embeddings.clear();
        self.cloud_sync = false;
        self.audit.push(AuditEntry {
            event: "wiped".into(),
        });
        "deletion-receipt-fixture".into()
    }
}
