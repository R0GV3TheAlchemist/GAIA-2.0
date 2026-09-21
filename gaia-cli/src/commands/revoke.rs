use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct RevokeArgs {
    /// Agent name or ID to revoke
    pub agent: String,

    /// Gateway base URL
    #[arg(long)]
    pub gateway: Option<String>,
}

pub async fn run(args: RevokeArgs) -> Result<()> {
    let gateway = args.gateway.as_deref().unwrap_or("http://localhost:7700");
    let client = reqwest::Client::new();
    let url = format!("{gateway}/agents/{}/revoke", args.agent);
    let resp = client.delete(&url).send().await?;
    match resp.status().as_u16() {
        204 => println!("✓ Agent '{}' revoked.", args.agent),
        404 => println!("✗ Agent '{}' not found.", args.agent),
        s   => println!("✗ Unexpected status {s} revoking '{}'.", args.agent),
    }
    Ok(())
}
