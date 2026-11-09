use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn logging_middleware(request: Request, next: Next) -> Response {
    info!(method = %request.method(), path = %request.uri().path(), "handling request");
    next.run(request).await
}
