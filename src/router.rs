use std::sync::Arc;
use axum::Router;
use axum::routing::{post};
use sqlx::{Pool, Postgres};
use crate::handlers::{create_user, verify_user};

// Endpoint router
pub fn router(users_database: Arc<Pool<Postgres>> ) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/verify", post(verify_user))
        .with_state(users_database)
}