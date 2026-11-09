use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::listing::{CreateListingRequest, Listing, PublicListing, UpdateListingRequest},
    services::listing_service,
    state::AppState,
};

pub async fn list_public_listings(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicListing>>, AppError> {
    Ok(Json(
        listing_service::list_public_listings(&state.db).await?,
    ))
}

pub async fn get_public_listing(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PublicListing>, AppError> {
    Ok(Json(
        listing_service::get_public_listing(&state.db, id).await?,
    ))
}

pub async fn list_landlord_listings(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Listing>>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        listing_service::list_landlord_listings(&state.db, auth_user.user_id).await?,
    ))
}

pub async fn get_landlord_listing(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Listing>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        listing_service::get_landlord_listing(&state.db, auth_user.user_id, id).await?,
    ))
}

pub async fn create_listing(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateListingRequest>,
) -> Result<(StatusCode, Json<Listing>), AppError> {
    auth_user.require_landlord()?;
    let listing = listing_service::create_listing(&state.db, auth_user.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(listing)))
}

pub async fn update_listing(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateListingRequest>,
) -> Result<Json<Listing>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        listing_service::update_listing(&state.db, auth_user.user_id, id, payload).await?,
    ))
}

pub async fn archive_listing(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    auth_user.require_landlord()?;
    listing_service::archive_listing(&state.db, auth_user.user_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
