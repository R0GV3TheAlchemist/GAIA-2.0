use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct MemoryArgs {
    #[command(subcommand)]
    pub action: MemoryAction,
}

#[derive(Subcommand)]
pub enum MemoryAction {
    /// List memory cubes
    List,
    /// Search memory
    Search {
        #[arg()]
        query: String,
    },
}

pub async fn run(args: MemoryArgs) -> Result<()> {
    match args.action {
        MemoryAction::List => {
            println!("Listing memory cubes...");
            // TODO: GET /memory
        }
        MemoryAction::Search { query } => {
            println!("Searching memory: {query:?}");
            // TODO: GET /memory/search?q=...
        }
    }
    Ok(())
}
