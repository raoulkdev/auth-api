use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use crate::user::{User, UserPayload};

// Add a new user
pub async fn create_user(State(users_database): State<Arc<Pool<Postgres>>>, Json(payload): Json<UserPayload>) -> impl IntoResponse {
    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(payload.password.as_bytes(), &salt).unwrap().to_string();

    let new_user = sqlx::query_as::<_, User>("INSERT INTO users (username, hashed_password, last_login_at) VALUES ($1, $2, now()) RETURNING *")
        .bind(payload.username)
        .bind(hashed_password)
        .fetch_one(&*users_database)
        .await;

    match new_user {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{err}"))).into_response()
    }
}

