use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Debug)]
enum LiliError {
    HyperError(hyper::Error),
    ConfigError(config::ConfigError),
}

impl std::error::Error for LiliError {}

impl fmt::Display for LiliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiliError::HyperError(e) => e.fmt(f),
            LiliError::ConfigError(e) => e.fmt(f),
        }
    }
}

impl From<hyper::Error> for LiliError {
    fn from(e: hyper::Error) -> LiliError {
        Self::HyperError(e)
    }
}

impl From<config::ConfigError> for LiliError {
    fn from(e: config::ConfigError) -> LiliError {
        Self::ConfigError(e)
    }
}

#[tokio::main]
async fn main() -> Result<(), LiliError> {
    let cli = Cli::parse();

    let config = lili::configuration::get_configuration(cli.config)?;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.app_port));
    lili::run(&addr)?.await?;
    Ok(())
}
