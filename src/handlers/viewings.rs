use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::viewing::Viewing,
    schemas::viewing::{UpdateViewingSchema, ViewingSchema},
    services::viewing_service,
    state::AppState,
};

pub async fn create_public_viewing(
    State(state): State<AppState>,
    Json(payload): Json<ViewingSchema>,
) -> Result<(StatusCode, Json<Viewing>), AppError> {
    let viewing = viewing_service::create_public_viewing(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(viewing)))
}

pub async fn list_landlord_viewings(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Viewing>>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        viewing_service::list_landlord_viewings(&state.db, auth_user.user_id).await?,
    ))
}

pub async fn get_landlord_viewing(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Viewing>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        viewing_service::get_landlord_viewing(&state.db, auth_user.user_id, id).await?,
    ))
}

pub async fn update_viewing(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateViewingSchema>,
) -> Result<Json<Viewing>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        viewing_service::update_viewing(&state.db, auth_user.user_id, id, payload).await?,
    ))
}

pub async fn cancel_viewing(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    auth_user.require_landlord()?;
    viewing_service::cancel_viewing(&state.db, auth_user.user_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
