use clap::Parser;
use rcli::{Cli, CmdExecutor};

/// Main Function
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Cli::parse();
    opts.cmd.execute().await?;

    Ok(())
}
