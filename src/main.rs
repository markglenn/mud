pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::listen(3000).await
}
