// use axum::{extract::Request, middleware::Next, response::Response};

// pub async fn request_id_middleware(request: Request, next: Next) -> Response {
//     let _request_id = request
//         .headers()
//         .get("x-request-id")
//         .and_then(|v| v.to_str().ok());
//     next.run(request).await
// }
