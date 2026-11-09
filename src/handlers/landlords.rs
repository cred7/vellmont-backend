use axum::{Json, extract::State};

use crate::{
    error::AppError, middleware::auth::AuthUser, models::landlord::LandlordProfile,
    schemas::landlord::LandlordSchema, services::landlord_service, state::AppState,
};

pub async fn get_profile(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<LandlordProfile>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        landlord_service::get_profile(&state.db, auth_user.user_id).await?,
    ))
}

pub async fn update_profile(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<LandlordSchema>,
) -> Result<Json<LandlordProfile>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        landlord_service::update_profile(&state.db, auth_user.user_id, payload).await?,
    ))
}
