use axum::{routing::get, routing::IntoMakeService, Router};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub mod authentication;
pub mod configuration;
pub mod logging;
pub mod routes;

pub fn run(
    addr: &SocketAddr,
    pool: PgPool,
) -> hyper::Result<axum::Server<AddrIncoming, IntoMakeService<Router>>> {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .with_state(pool)
        .layer(cors);
    let server = axum::Server::try_bind(&addr)?.serve(app.into_make_service());

    println!("listening on {}", server.local_addr());

    Ok(server)
}

async fn root() -> &'static str {
    "Hello, World!"
}
