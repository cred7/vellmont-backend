use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::unit::{CreateUnitRequest, Unit, UpdateUnitRequest},
    services::unit_service::{
        archive_unit_service, create_unit_service, get_public_unit, get_unit, get_units,
        update_unit_service,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct UnitListQuery {
    pub property_id: Option<Uuid>,
}

pub async fn list_units(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<UnitListQuery>,
) -> Result<Json<Vec<Unit>>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        get_units(&state.db, auth_user.user_id, query.property_id).await?,
    ))
}

pub async fn get_unit_by_id(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(unit_id): Path<Uuid>,
) -> Result<Json<Unit>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(get_unit(&state.db, auth_user.user_id, unit_id).await?))
}

pub async fn get_public_unit_by_slug(
    State(state): State<AppState>,
    Path((property_slug, unit_slug)): Path<(String, String)>,
) -> Result<Json<Unit>, AppError> {
    Ok(Json(
        get_public_unit(&state.db, &property_slug, &unit_slug).await?,
    ))
}

pub async fn create_unit(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateUnitRequest>,
) -> Result<(StatusCode, Json<Unit>), AppError> {
    auth_user.require_landlord()?;
    println!("Creating unit with payload: {:?}", payload);
    let unit = create_unit_service(&state.db, auth_user.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(unit)))
}

pub async fn update_unit(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(unit_id): Path<Uuid>,
    Json(payload): Json<UpdateUnitRequest>,
) -> Result<Json<Unit>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        update_unit_service(&state.db, auth_user.user_id, unit_id, payload).await?,
    ))
}

pub async fn archive_unit(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(unit_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    auth_user.require_landlord()?;
    archive_unit_service(&state.db, auth_user.user_id, unit_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
