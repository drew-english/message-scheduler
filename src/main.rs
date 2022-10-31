mod config;
mod core;
mod http;
mod models;

use std::net::SocketAddr;

use crate::core::process_loop;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

async fn db_connection(db_url: &str) -> sqlx::Pool<sqlx::Postgres> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    init_logger();
    let cfg = config::load().unwrap();

    let db_pool = db_connection(&cfg.database_url).await;
    let new_msg_delivery_tx = process_loop::run(db_pool.clone());

    info!(host = cfg.api_host, port = cfg.api_port, "Binding server to");
    let addr = format!("{}:{}", cfg.api_host, cfg.api_port)
        .parse::<SocketAddr>()
        .unwrap();

    axum::Server::bind(&addr)
        .serve(http::router(db_pool, new_msg_delivery_tx).into_make_service())
        .await
        .unwrap();
}
