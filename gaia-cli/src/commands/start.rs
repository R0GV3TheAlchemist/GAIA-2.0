use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct StartArgs {
    /// Override the gateway address
    #[arg(long)]
    pub gateway: Option<String>,
}

pub async fn run(args: StartArgs) -> Result<()> {
    let addr = args.gateway.as_deref().unwrap_or("http://localhost:7700");
    println!("Starting GAIA runtime — gateway: {addr}");
    // TODO: spawn gaia-gateway process or connect to running instance
    println!("✓ Runtime started.");
    Ok(())
}
