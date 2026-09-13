//! Persist TrustAudit lines to a local file. Not OpenTelemetry.

use crate::trust::TrustAudit;
use std::io::Write;
use std::path::Path;

pub fn persist_audit(path: &Path, audit: &TrustAudit) -> Result<(), String> {
    let mut f = std::fs::File::create(path).map_err(|e| format!("audit create: {e}"))?;
    for e in audit.events() {
        writeln!(
            f,
            "{}|{:?}|{:?}|{}",
            e.intent_id, e.plan_id, e.executor_id, e.event
        )
        .map_err(|e| format!("audit write: {e}"))?;
    }
    Ok(())
}
