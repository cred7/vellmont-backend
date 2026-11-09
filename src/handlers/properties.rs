use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::property::{CreatePropertyRequest, Property, UpdatePropertyRequest},
    services::property_service::{
        archive_property_service, create_property_service, get_properties, get_property,
        update_property_service,
    },
    state::AppState,
};

pub async fn list_properties(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Property>>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(get_properties(&state.db, auth_user.user_id).await?))
}

pub async fn get_property_by_slug(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
) -> Result<Json<Property>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        get_property(&state.db, auth_user.user_id, &slug).await?,
    ))
}

pub async fn create_property(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreatePropertyRequest>,
) -> Result<(StatusCode, Json<Property>), AppError> {
    auth_user.require_landlord()?;
    let property = create_property_service(&state.db, auth_user.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(property)))
}

pub async fn update_property(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
    Json(payload): Json<UpdatePropertyRequest>,
) -> Result<Json<Property>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        update_property_service(&state.db, auth_user.user_id, &slug, payload).await?,
    ))
}

pub async fn archive_property(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
) -> Result<StatusCode, AppError> {
    auth_user.require_landlord()?;
    archive_property_service(&state.db, auth_user.user_id, &slug).await?;
    Ok(StatusCode::NO_CONTENT)
}
