mod handlers;
mod layers;

use axum::{extract::Extension, middleware, routing::get, Router};
use tower::ServiceBuilder;

pub fn router(db_pool: sqlx::Pool<sqlx::Postgres>) -> Router {
    Router::new().route("/test", get(handlers::test)).layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn(layers::logger))
            .layer(Extension(db_pool)),
    )
}
