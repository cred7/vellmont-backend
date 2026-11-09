use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::transaction::RentalTransaction,
    schemas::transaction::{TransactionSchema, UpdateTransactionSchema},
    services::transaction_service,
    state::AppState,
};

pub async fn list_landlord_transactions(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<RentalTransaction>>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        transaction_service::list_landlord_transactions(&state.db, auth_user.user_id).await?,
    ))
}

pub async fn get_landlord_transaction(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<RentalTransaction>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        transaction_service::get_landlord_transaction(&state.db, auth_user.user_id, id).await?,
    ))
}

pub async fn create_transaction(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<TransactionSchema>,
) -> Result<(StatusCode, Json<RentalTransaction>), AppError> {
    auth_user.require_landlord()?;
    let transaction =
        transaction_service::create_from_accepted_offer(&state.db, auth_user.user_id, payload)
            .await?;
    Ok((StatusCode::CREATED, Json(transaction)))
}

pub async fn update_transaction_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionSchema>,
) -> Result<Json<RentalTransaction>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        transaction_service::update_transaction_status(
            &state.db,
            auth_user.user_id,
            id,
            payload.status,
        )
        .await?,
    ))
}
