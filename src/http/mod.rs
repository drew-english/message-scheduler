mod layers;
mod handlers;

use axum::{Router, middleware, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/test", get(handlers::test))
        .layer(middleware::from_fn(layers::logger))
}
