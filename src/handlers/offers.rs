use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::offer::Offer,
    schemas::offer::{OfferSchema, UpdateOfferSchema},
    services::offer_service,
    state::AppState,
};

pub async fn create_public_offer(
    State(state): State<AppState>,
    Json(payload): Json<OfferSchema>,
) -> Result<(StatusCode, Json<Offer>), AppError> {
    let offer = offer_service::create_public_offer(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(offer)))
}

pub async fn list_landlord_offers(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Offer>>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        offer_service::list_landlord_offers(&state.db, auth_user.user_id).await?,
    ))
}

pub async fn get_landlord_offer(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Offer>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        offer_service::get_landlord_offer(&state.db, auth_user.user_id, id).await?,
    ))
}

pub async fn update_offer_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOfferSchema>,
) -> Result<Json<Offer>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        offer_service::update_offer_status(&state.db, auth_user.user_id, id, payload.status)
            .await?,
    ))
}
