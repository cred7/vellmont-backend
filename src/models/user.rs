use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type RegisterRequest = crate::schemas::auth::RegisterSchema;
pub type LoginRequest = crate::schemas::auth::LoginSchema;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub iat: usize,
    pub exp: usize,
}

impl User {
    pub fn to_public(&self) -> UserPublic {
        UserPublic {
            id: self.id,
            email: self.email.clone(),
            full_name: self.full_name.clone(),
            role: self.role.clone(),
            created_at: self.created_at,
        }
    }
}
