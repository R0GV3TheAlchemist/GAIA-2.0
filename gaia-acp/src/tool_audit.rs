//! Append-only tool audit log (#933). Entries are not mutable through the public API.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolOutcome {
    Permitted,
    Denied,
    Elevated,
    HumanApprovalRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAuditEntry {
    pub agent_id: String,
    pub tool_id: String,
    pub timestamp_unix: u64,
    pub params_hash: String,
    pub outcome: ToolOutcome,
}

#[derive(Debug, Default)]
pub struct ToolAuditLog {
    entries: Vec<ToolAuditEntry>,
}

impl ToolAuditLog {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn append(&mut self, entry: ToolAuditEntry) -> ToolAuditEntry {
        self.entries.push(entry.clone());
        entry
    }

    pub fn entries(&self) -> &[ToolAuditEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_only_surface() {
        let mut log = ToolAuditLog::default();
        log.append(ToolAuditEntry {
            agent_id: "a".into(),
            tool_id: "t".into(),
            timestamp_unix: 1,
            params_hash: "00".into(),
            outcome: ToolOutcome::Permitted,
        });
        assert_eq!(log.entries().len(), 1);
    }
}
