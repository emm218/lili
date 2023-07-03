use axum::{routing::get, routing::IntoMakeService, Router};
use axum_sessions::{async_session::SessionStore, SessionLayer};
use hyper::server::conn::AddrIncoming;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub mod authentication;
pub mod configuration;
pub mod logging;
pub mod routes;

pub fn run<T>(
    addr: &SocketAddr,
    pool: PgPool,
    session_store: T,
    session_secret: Secret<String>,
) -> hyper::Result<axum::Server<AddrIncoming, IntoMakeService<Router>>>
where
    T: SessionStore,
{
    let cors = CorsLayer::new().allow_origin(Any);
    let session_layer = SessionLayer::new(session_store, session_secret.expose_secret().as_bytes());
    let app = Router::new()
        .route("/", get(root))
        .with_state(pool)
        .layer(session_layer)
        .layer(cors);
    let server = axum::Server::try_bind(&addr)?.serve(app.into_make_service());

    println!("listening on {}", server.local_addr());

    Ok(server)
}

async fn root() -> &'static str {
    "Hello, World!"
}
