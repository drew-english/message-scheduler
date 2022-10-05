use axum::{http::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn logger<B>(req: Request<B>, next: Next<B>) -> Response {
    let req_method = req.method().clone();
    let req_uri = req.uri().clone();

    let res = next.run(req).await;

    info!(
        method = req_method.as_str(),
        path = req_uri.path(),
        status = res.status().as_str(),
    );

    res
}
