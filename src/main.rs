use async_redis_session::RedisSessionStore;
use axum_sessions::async_session::MemoryStore;
use secrecy::ExposeSecret;
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
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let config = lili::configuration::get_config(cli.config)?;
    let pool = PgPool::connect(&config.database.connection_string()).await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    match config.session_store_uri {
        Some(uri) => lili::run(
            &addr,
            pool,
            RedisSessionStore::new(uri.expose_secret().as_str())?,
            config.session_secret,
        ),
        None => lili::run(&addr, pool, MemoryStore::new(), config.session_secret),
    }?
    .await?;

    Ok(())
}
