mod handlers;
mod layers;

use axum::{middleware, routing::get, Router};
use tower::ServiceBuilder;

pub fn router() -> Router {
    Router::new()
        .route("/test", get(handlers::test))
        .layer(ServiceBuilder::new().layer(middleware::from_fn(layers::logger)))
}
