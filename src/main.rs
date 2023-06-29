use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), lili::LiliError> {
    let cli = Cli::parse();

    let config = lili::configuration::get_configuration(cli.config)?;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.app_port));
    lili::run(&addr)?.await?;
    Ok(())
}
