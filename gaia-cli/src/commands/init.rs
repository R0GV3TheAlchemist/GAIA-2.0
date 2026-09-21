use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    /// Profile to initialise (e.g. developer, sovereign)
    #[arg(long, default_value = "developer")]
    pub profile: String,

    /// Gateway base URL
    #[arg(long, default_value = "http://localhost:7700")]
    pub gateway: String,
}

pub async fn run(args: InitArgs) -> Result<()> {
    println!("Initialising GAIA profile: {}", args.profile);
    // TODO: write ~/.gaia/config.toml with profile + gateway URL
    println!("✓ Profile '{}' ready. Run `gaia start` to launch.", args.profile);
    Ok(())
}
