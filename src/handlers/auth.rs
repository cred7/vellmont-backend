use axum::{Json, extract::State};

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::user::{AuthResponse, LoginRequest, RegisterRequest},
    services::auth_service::{login_user, register_user},
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = register_user(&state.db, &state.jwt_secret, payload).await?;
    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = login_user(&state.db, &state.jwt_secret, payload).await?;
    Ok(Json(response))
}

pub async fn me(auth_user: AuthUser) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "user_id": auth_user.user_id,
        "email": auth_user.email,
        "role": auth_user.role,
    })))
}
