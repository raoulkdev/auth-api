use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use sqlx::{Pool, Postgres};
use crate::handlers::{create_user};

// Endpoint router
pub fn router(users_database: Arc<Pool<Postgres>> ) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .with_state(users_database)
}