use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserPayload {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    id: Uuid,
    username: String,
    hashed_password: String,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
    last_login_at: chrono::DateTime<Utc>
}