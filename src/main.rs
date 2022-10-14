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

async fn db_connection() -> sqlx::Pool<sqlx::Postgres> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap(); // TODO: remove
    init_logger();

    let db_pool = db_connection().await;
    let new_msg_delivery_tx = process_loop::run(db_pool.clone());

    let host = std::env::var("API_HOST").unwrap();
    let port = std::env::var("API_PORT").unwrap();
    info!(host, port, "Binding server to");
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();

    axum::Server::bind(&addr)
        .serve(http::router(db_pool, new_msg_delivery_tx).into_make_service())
        .await
        .unwrap();
}
