use axum::{routing::get, routing::IntoMakeService, Router};
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub fn run(
    addr: &SocketAddr,
) -> hyper::Result<axum::Server<AddrIncoming, IntoMakeService<Router>>> {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new().route("/", get(root)).layer(cors);

    println!("listening on {addr}");

    Ok(axum::Server::try_bind(&addr)?.serve(app.into_make_service()))
}

async fn root() -> &'static str {
    "Hello, World!"
}
