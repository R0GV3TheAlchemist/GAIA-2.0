//! GAIA userspace kernel path (Phase 1).

pub mod audit;
pub mod broker;
pub mod executor;
pub mod host;
pub mod identity;

pub use host::{KernelError, KernelHost, SignedBlob, TaskHandle};
pub use identity::{Principal, PrincipalKind};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broker::Broker;
    use crate::executor::Executor;
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp() -> std::path::PathBuf {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("gaia-kernel-{n}"))
    }

    #[test]
    fn executor_registers_and_pulls_noop() {
        let broker = Arc::new(Broker::new());
        broker.enqueue("noop", "{}");
        let p = Principal::generate(PrincipalKind::Node);
        let exec = Executor::new(&p, Default::default(), broker.clone());
        let task = exec.pull_one().expect("task");
        assert_eq!(task.kind, "noop");
        assert!(exec.run_task(&task).starts_with("noop-ok"));
        assert_eq!(broker.node_count(), 1);
        assert!(exec.pull_one().is_none());
    }

    #[test]
    fn syscalls_sfs_memos_signed_audit() {
        let mut host = KernelHost::new(tmp()).unwrap();
        let intent = host.intent("register this node").unwrap();
        assert_eq!(intent.state, "admitted");

        let obj = host
            .store_file("docs/note.txt", b"gaia semantic file about texas weather", "keep notes")
            .unwrap();
        assert_eq!(obj.cid.len(), 64);
        let hits = host.sfs.search("texas weather", 2);
        assert!(!hits.is_empty());

        let cube = host.write_cube("episodic: talked about texas weather").unwrap();
        let recalled = host.context("texas weather").unwrap();
        assert!(!recalled.content.is_empty());
        let _ = cube;

        let blob = host.sign(b"intent-proof").unwrap();
        assert!(host.verify(b"intent-proof", &blob).unwrap());
        assert!(!host.verify(b"other", &blob).unwrap());

        let proof = host
            .last_audit("sfs.put", "docs/note.txt")
            .expect("audit record");
        assert!(proof.signature_hex.len() > 20);
        assert!(host.audit.chain_ok());

        let err = host.reject_unsigned(None, b"x").unwrap_err();
        assert!(matches!(err, KernelError::Denied(_)));

        let stranger = Principal::generate(PrincipalKind::Agent);
        let foreign = SignedBlob {
            algorithm: "ed25519".into(),
            public_hex: stranger.public_hex(),
            signature_hex: hex::encode(stranger.sign(b"x")),
        };
        let err = host.reject_unsigned(Some(&foreign), b"x").unwrap_err();
        assert!(matches!(err, KernelError::Denied(_)));
    }
}
