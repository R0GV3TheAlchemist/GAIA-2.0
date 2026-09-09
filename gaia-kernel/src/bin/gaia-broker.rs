use gaia_kernel::broker::Broker;

fn main() {
    let broker = Broker::new();
    let id = broker.enqueue("noop", "{}");
    println!("gaia-broker queued {id}");
    println!("nodes {}", broker.node_count());
}
