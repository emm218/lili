#[tokio::main]
async fn main() -> hyper::Result<()> {
    lili::run()?.await
}
