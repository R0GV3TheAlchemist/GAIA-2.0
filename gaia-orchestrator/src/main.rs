use gaia_memos::MemOs;
use gaia_orchestrator::{IntentEngine, TaskPlanner};

fn usage() -> &'static str {
    "usage: gaia intent <text>\n\nBuilds an inspectable local-only, unsigned plan; it does not execute."
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) != Some("intent") || args.len() < 2 {
        eprintln!("{}", usage());
        std::process::exit(2);
    }

    let text = args[1..].join(" ");
    let mem = MemOs::new();
    let graph = match IntentEngine::local_stub().parse(&text, &mem) {
        Ok(graph) => graph,
        Err(error) => {
            eprintln!("intent error: {error}");
            std::process::exit(1);
        }
    };
    let plan = TaskPlanner::from_intent(&graph);
    println!("intent_id={}", graph.id);
    println!("signed={}", graph.is_signed());
    println!("privacy=local-only");
    println!("execution=not-started; inspect and accept through API first");
    println!("{}", plan.inspect());
}
