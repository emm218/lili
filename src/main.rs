use std::net::SocketAddr;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    lili::run(&addr)?.await
}
