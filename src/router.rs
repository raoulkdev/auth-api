use axum::Router;
use axum::routing::get;
use crate::handlers::hello_world;

// Endpoint router
pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
}