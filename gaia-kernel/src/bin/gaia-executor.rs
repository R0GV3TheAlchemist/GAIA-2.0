use std::sync::Arc;
use std::time::Duration;

use gaia_kernel::broker::{Broker, Capabilities};
use gaia_kernel::executor::Executor;
use gaia_kernel::{Principal, PrincipalKind};

/// Pull-based continuum node. Opens no inbound port; it only calls broker.pull.
#[tokio::main]
async fn main() {
    let broker = Arc::new(Broker::new());
    broker.enqueue("noop", "{}");
    let principal = Principal::generate(PrincipalKind::Node);
    let exec = Executor::new(&principal, Capabilities::default(), broker);
    println!("gaia-executor node={}", exec.node_id);
    println!("caps cpu={} gpu={} cuda={} rocm={} opencl={}", exec.caps.cpu_cores, exec.caps.gpu, exec.caps.cuda, exec.caps.rocm, exec.caps.opencl);
    if let Some(task) = exec.pull_one() {
        println!("{}", exec.run_task(&task));
    } else {
        println!("idle");
    }
    tokio::time::sleep(Duration::from_millis(1)).await;
}
