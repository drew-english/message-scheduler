mod handlers;
mod layers;

use axum::{extract::Extension, middleware, routing::post, Router};
use tower::ServiceBuilder;

pub fn router(db_pool: sqlx::Pool<sqlx::Postgres>) -> Router {
    Router::new()
        .route("/api/v1/message", post(handlers::create_message))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(layers::logger))
                .layer(Extension(db_pool)),
        )
}
