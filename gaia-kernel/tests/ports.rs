//! #30 port profiles and HPC pull. No live Slurm cluster.

use gaia_kernel::{matrix, HpcAdapter, PortProfile};

#[test]
fn matrix_documents_linux_x86_64_and_arm64() {
    let rows = matrix();
    assert!(rows.contains(&PortProfile::LinuxX86_64));
    assert!(rows.contains(&PortProfile::LinuxArm64));
    let x86 = PortProfile::LinuxX86_64.documented_build();
    assert!(x86.contains("x86_64-unknown-linux-gnu"));
    let arm = PortProfile::LinuxArm64.documented_build();
    assert!(arm.contains("aarch64-unknown-linux-gnu"));
}

#[test]
fn iot_profile_is_executor_and_sensors_with_a_budget() {
    let fp = PortProfile::Iot.footprint();
    assert!(fp.executor);
    assert!(fp.sensors);
    assert!(!fp.memos);
    assert!(!fp.agents);
    assert_eq!(fp.max_rss_kib, 8 * 1024);
    assert_eq!(fp.max_binary_kib, 2 * 1024);
}

#[test]
fn hpc_adapter_pulls_a_batch_task() {
    let mut hpc = HpcAdapter::new();
    hpc.submit("job-1", "batch", "nodes=2");
    let task = hpc.pull().expect("batch task");
    assert_eq!(task.job_id, "job-1");
    assert_eq!(task.kind, "batch");
    assert_eq!(task.payload, "nodes=2");
    assert!(hpc.pull().is_none());
    assert_eq!(hpc.pulled().len(), 1);
}

#[test]
fn riscv_is_marked_experimental() {
    let fp = PortProfile::RiscVExperimental.footprint();
    assert_eq!(fp.rustc_target, "riscv64gc-unknown-linux-gnu");
    assert!(!fp.agents);
}
