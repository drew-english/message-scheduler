mod core;
mod http;
mod models;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn init() {
    // init logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    init();
    core::process_loop::start();
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(http::router().into_make_service())
        .await
        .unwrap();
}
