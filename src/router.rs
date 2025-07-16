use crate::handlers::{create_user, verify_user};
use axum::Router;
use axum::routing::post;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

// Endpoint router
pub fn router(users_database: Arc<Pool<Postgres>>) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/verify", post(verify_user))
        .with_state(users_database)
}
