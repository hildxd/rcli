use clap::Parser;
use rcli::{Cli, CmdExector};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    cli.cmd.execute().await?;
    Ok(())
}
