use axum::{routing::get, routing::IntoMakeService, Router};
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub mod configuration;
pub mod routes;

#[derive(Debug)]
pub enum LiliError {
    HyperError(hyper::Error),
    ConfigError(config::ConfigError),
}

impl std::error::Error for LiliError {}

impl std::fmt::Display for LiliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

pub fn run(
    addr: &SocketAddr,
) -> hyper::Result<axum::Server<AddrIncoming, IntoMakeService<Router>>> {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new().route("/", get(root)).layer(cors);
    let server = axum::Server::try_bind(&addr)?.serve(app.into_make_service());

    println!("listening on {}", server.local_addr());

    Ok(server)
}

async fn root() -> &'static str {
    "Hello, World!"
}
