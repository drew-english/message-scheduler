mod handlers;
mod layers;

use axum::{extract::Extension, middleware, routing::{get, post}, Router};
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use tower::ServiceBuilder;

pub fn router(
    db_pool: sqlx::Pool<sqlx::Postgres>,
    msg_delivery_tx: mpsc::UnboundedSender<Option<DateTime<Utc>>>,
) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/v1/message", post(handlers::create_message))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(layers::logger))
                .layer(Extension(db_pool))
                .layer(Extension(msg_delivery_tx)),
        )
}
