use anyhow::Result;
use clap::Parser;
use rcli::CmdExecutor;

use rcli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    args.cmd.execute().await?;
    Ok(())
}
