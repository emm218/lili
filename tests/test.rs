use std::net::SocketAddr;

#[tokio::test]
async fn sanity_check() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));

    let url = spawn_app(&addr);

    let response = reqwest::get(url).await.unwrap();
    assert!(response.status().is_success());

    let body = response.text().await;
    assert_eq!(String::from("Hello, World!"), body.unwrap());
}

fn spawn_app(addr: &SocketAddr) -> String {
    let server = lili::run(addr).expect("failed to bind address");

    let local_addr = server.local_addr();

    tokio::spawn(server);

    format!("http://{}", local_addr)
}
