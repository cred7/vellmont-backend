use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::lead::Lead,
    schemas::lead::{LeadSchema, UpdateLeadSchema},
    services::lead_service,
    state::AppState,
};

pub async fn create_public_lead(
    State(state): State<AppState>,
    Json(payload): Json<LeadSchema>,
) -> Result<(StatusCode, Json<Lead>), AppError> {
    let lead = lead_service::create_public_lead(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(lead)))
}

pub async fn list_landlord_leads(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Lead>>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        lead_service::list_landlord_leads(&state.db, auth_user.user_id).await?,
    ))
}

pub async fn get_landlord_lead(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Lead>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        lead_service::get_landlord_lead(&state.db, auth_user.user_id, id).await?,
    ))
}

pub async fn update_lead_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateLeadSchema>,
) -> Result<Json<Lead>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        lead_service::update_lead_status(&state.db, auth_user.user_id, id, payload.status).await?,
    ))
}
