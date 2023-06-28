#[tokio::test]
async fn sanity_check() {
    spawn_app();

    let response = reqwest::get("http://localhost:3000").await.unwrap();
    assert!(response.status().is_success());

    let body = response.text().await;
    assert_eq!(String::from("Hello, World!"), body.unwrap());
}

fn spawn_app() {
    let server = lili::run().expect("failed to bind address");

    tokio::spawn(server);
}
