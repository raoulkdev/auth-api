use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

// Get hello world test response
pub async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, Json("Hello World"))
}
