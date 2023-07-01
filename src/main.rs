use sqlx::PgPool;
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

    let config = lili::configuration::get_config(cli.config)?;
    let pool = PgPool::connect(&config.database.connection_string()).await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.app_port));
    lili::run(&addr, pool)?.await?;
    Ok(())
}
