use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError, models::transaction::RentalTransaction, repositories::transaction_repository,
    schemas::transaction::TransactionSchema,
};

const TRANSACTION_STATUSES: &[&str] = &["completed", "cancelled"];

pub async fn list_landlord_transactions(
    db: &PgPool,
    landlord_id: Uuid,
) -> Result<Vec<RentalTransaction>, AppError> {
    transaction_repository::list_landlord_transactions(db, landlord_id).await
}

pub async fn get_landlord_transaction(
    db: &PgPool,
    landlord_id: Uuid,
    transaction_id: Uuid,
) -> Result<RentalTransaction, AppError> {
    transaction_repository::get_landlord_transaction(db, landlord_id, transaction_id)
        .await?
        .ok_or_else(|| AppError::NotFound("transaction not found".to_string()))
}

pub async fn create_from_accepted_offer(
    db: &PgPool,
    landlord_id: Uuid,
    input: TransactionSchema,
) -> Result<RentalTransaction, AppError> {
    transaction_repository::create_from_accepted_offer(db, landlord_id, &input)
        .await?
        .ok_or_else(|| {
            AppError::BadRequest(
                "offer must be accepted and belong to an active listing you own".to_string(),
            )
        })
}

pub async fn update_transaction_status(
    db: &PgPool,
    landlord_id: Uuid,
    transaction_id: Uuid,
    status: String,
) -> Result<RentalTransaction, AppError> {
    if !TRANSACTION_STATUSES.contains(&status.as_str()) {
        return Err(AppError::BadRequest(
            "transaction status must be completed or cancelled".to_string(),
        ));
    }
    let current = get_landlord_transaction(db, landlord_id, transaction_id).await?;
    if current.status != "pending" {
        return Err(AppError::BadRequest(
            "only pending transactions can be completed or cancelled".to_string(),
        ));
    }
    transaction_repository::update_landlord_transaction_status(
        db,
        landlord_id,
        transaction_id,
        &status,
    )
    .await?
    .ok_or_else(|| AppError::NotFound("transaction not found".to_string()))
}
