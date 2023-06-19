mod print;
mod server;

#[tokio::main]
async fn main() {
    server::start().await;
}
