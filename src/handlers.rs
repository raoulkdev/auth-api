use std::sync::Arc;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
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

    // Check if passwords meet length requirements
    if payload.username.len() <= 3 || payload.password.len() <= 7 {
        return (StatusCode::BAD_REQUEST, Json("Username length must be > 3 and password length must be > 7")).into_response()
    }

    // Add user
    let new_user = sqlx::query_as::<_, User>("INSERT INTO users (username, hashed_password, last_login_at) VALUES ($1, $2, now()) RETURNING *")
        .bind(payload.username)
        .bind(hashed_password)
        .fetch_one(&*users_database)
        .await;

    // Return a new user or error
    match new_user {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => {
            if err.to_string().contains("duplicate key"){
                (StatusCode::CONFLICT, Json("Username already exists!")).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{err}"))).into_response()
            }
        }
    }
}

// Verify user credentials
pub async fn verify_user(State(users_database): State<Arc<Pool<Postgres>>>, Json(payload): Json<UserPayload>) -> impl IntoResponse {
    // Find user with matching username from payload
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(payload.username)
        .fetch_one(&*users_database)
        .await;

    match user {
        Ok(user) => {
            // Verify passwords and return the verified status
            let submitted_password = payload.password;
            let unhashed_password = PasswordHash::new(user.hashed_password.as_str()).unwrap();
            let argon2 = Argon2::default();
            let verified_status = argon2.verify_password(submitted_password.as_bytes(), &unhashed_password).is_ok();
            let mut status_msg = String::new();
            if verified_status {
                status_msg = "Verification Successful".to_string();
            } else {
                status_msg = "Invalid password".to_string();
            }
            (StatusCode::OK, Json(status_msg)).into_response()
        },
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{err}"))).into_response()
    }
}

