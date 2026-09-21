use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct IntentArgs {
    /// The intent text to submit
    pub text: String,

    /// Stream output as it arrives
    #[arg(long, default_value_t = true)]
    pub stream: bool,

    /// Gateway base URL
    #[arg(long)]
    pub gateway: Option<String>,
}

pub async fn run(args: IntentArgs) -> Result<()> {
    let gateway = args.gateway.as_deref().unwrap_or("http://localhost:7700");
    println!("Submitting intent to {gateway}: {:?}", args.text);
    // TODO: POST /intent  with Accept: text/event-stream when args.stream
    //       pipe each SSE chunk to stdout
    Ok(())
}
