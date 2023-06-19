mod config;
mod print;
mod server;

#[tokio::main]
async fn main() {
    let cfg = config::get();
    server::start(&cfg).await;
}
