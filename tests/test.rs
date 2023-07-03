use axum_sessions::async_session::MemoryStore;
use lili::configuration;
use lili::configuration::{DatabaseSettings, Settings};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::SocketAddr;

struct TestApp {
    pub url: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn sanity_check() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let settings = configuration::get_test_config().expect("failed to read test config");

    let app = spawn_test_app(&addr, settings).await;

    let response = reqwest::get(app.url).await.unwrap();
    assert!(response.status().is_success());

    let body = response.text().await;
    assert_eq!(String::from("Hello, World!"), body.unwrap());
}

async fn configure_database(config: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    let mut connection = PgConnection::connect(&config.connection_string_no_db()).await?;

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await?;

    let db_pool = PgPool::connect(&config.connection_string()).await?;

    sqlx::migrate!().run(&db_pool).await?;

    Ok(db_pool)
}

async fn spawn_test_app(addr: &SocketAddr, settings: Settings) -> TestApp {
    let db_pool = configure_database(&settings.database)
        .await
        .expect("failed to configure database");

    let server = lili::run(
        addr,
        db_pool.clone(),
        MemoryStore::new(),
        settings.session_secret,
    )
    .expect("failed to bind address");

    let local_addr = server.local_addr();

    tokio::spawn(server);

    TestApp {
        url: format!("http://{}", local_addr),
        db_pool,
    }
}
