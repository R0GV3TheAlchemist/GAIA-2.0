use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct AuditArgs {
    /// Tail the audit log (follow mode)
    #[arg(long)]
    pub follow: bool,

    /// Maximum number of entries to show
    #[arg(long, default_value_t = 50)]
    pub limit: usize,
}

pub async fn run(args: AuditArgs) -> Result<()> {
    println!("Audit log (limit={}, follow={})", args.limit, args.follow);
    // TODO: GET /audit?limit=N  or WS /audit/stream when --follow
    Ok(())
}
