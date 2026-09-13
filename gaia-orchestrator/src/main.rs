use gaia_memos::MemOs;
use gaia_orchestrator::{Broker, IntentEngine, IntentSigner, LocalRunner, TaskPlanner, TrustAudit};

fn usage() -> &'static str {
    "usage: gaia intent <text> [--accept] [--kill-specialist-a]\n\nWithout --accept, prints an inspectable local-only plan and does not execute.\n--accept runs the accepted plan in local in-process stubs only."
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) != Some("intent") {
        eprintln!("{}", usage());
        std::process::exit(2);
    }
    args.remove(0);
    let accept = take_flag(&mut args, "--accept");
    let kill_specialist_a = take_flag(&mut args, "--kill-specialist-a");
    if args.is_empty() {
        eprintln!("{}", usage());
        std::process::exit(2);
    }

    let text = args.join(" ");
    let mem = MemOs::new();
    let graph = match IntentEngine::local_stub().parse(&text, &mem) {
        Ok(graph) => graph,
        Err(error) => {
            eprintln!("intent error: {error}");
            std::process::exit(1);
        }
    };
    let signer = IntentSigner::generate();
    let signed = signer.sign(&graph);
    let mut plan = TaskPlanner::from_intent(&graph);

    println!("intent_id={}", graph.id);
    println!("trust={}", signed.algorithm);
    println!("issuer={}", signed.issuer_did);
    println!("privacy=local-only");
    println!("{}", plan.inspect());

    if !accept {
        println!("execution=not-started; rerun with --accept after inspection");
        return;
    }

    if let Err(error) = signer.verify(&signed) {
        eprintln!("intent verification failed: {error}");
        std::process::exit(1);
    }
    if let Err(error) = plan.accept_verified(&signed) {
        eprintln!("plan acceptance failed: {error}");
        std::process::exit(1);
    }
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    match LocalRunner::run(
        &plan,
        &mut broker,
        &mut audit,
        &signed,
        kill_specialist_a.then_some("specialist-a"),
    ) {
        Ok(run) => {
            println!("execution=completed");
            println!("completed_nodes={}", run.completed_jobs.len());
            println!("failed_over_nodes={}", run.failed_over_jobs.len());
            println!("audit_events={}", audit.events().len());
            println!("audit_chain_ok={}", audit.chain_ok());
        }
        Err(error) => {
            eprintln!("execution failed: {error}");
            std::process::exit(1);
        }
    }
}

fn take_flag(args: &mut Vec<String>, flag: &str) -> bool {
    if let Some(index) = args.iter().position(|arg| arg == flag) {
        args.remove(index);
        true
    } else {
        false
    }
}
