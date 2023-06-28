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

    let mut config_builder = config::Config::builder();

    //TODO: add fallback to config in config home
    config_builder = match cli.config {
        Some(path) => config_builder.add_source(config::File::from(path)),
        None => config_builder.add_source(config::File::with_name("config")),
    };

    config_builder.build()?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    lili::run(&addr)?.await?;
    Ok(())
}
