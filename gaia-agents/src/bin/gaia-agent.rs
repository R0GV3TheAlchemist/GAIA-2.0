use gaia_agents::AgentHost;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let mut host = AgentHost::new();
    match host.exec(&refs) {
        Ok(out) => println!("{out}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
