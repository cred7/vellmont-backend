use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError, models::transaction::RentalTransaction,
    schemas::transaction::TransactionSchema,
};

pub async fn list_landlord_transactions(
    db: &PgPool,
    landlord_id: Uuid,
) -> Result<Vec<RentalTransaction>, AppError> {
    sqlx::query_as::<_, RentalTransaction>(
        r#"
        SELECT txn.id, txn.listing_id, txn.offer_id, txn.tenant_id,
               txn.amount, txn.status, txn.completed_at, txn.created_at, txn.updated_at
        FROM transactions AS txn
        INNER JOIN listings AS listing ON listing.id = txn.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1
        ORDER BY txn.created_at DESC
        "#,
    )
    .bind(landlord_id)
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_landlord_transaction(
    db: &PgPool,
    landlord_id: Uuid,
    transaction_id: Uuid,
) -> Result<Option<RentalTransaction>, AppError> {
    sqlx::query_as::<_, RentalTransaction>(
        r#"
        SELECT txn.id, txn.listing_id, txn.offer_id, txn.tenant_id,
               txn.amount, txn.status, txn.completed_at, txn.created_at, txn.updated_at
        FROM transactions AS txn
        INNER JOIN listings AS listing ON listing.id = txn.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1 AND txn.id = $2
        "#,
    )
    .bind(landlord_id)
    .bind(transaction_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn create_from_accepted_offer(
    db: &PgPool,
    landlord_id: Uuid,
    input: &TransactionSchema,
) -> Result<Option<RentalTransaction>, AppError> {
    sqlx::query_as::<_, RentalTransaction>(
        r#"
        INSERT INTO transactions (listing_id, offer_id, tenant_id, amount, status)
        SELECT offer.listing_id, offer.id, $3, offer.amount, 'pending'
        FROM offers AS offer
        INNER JOIN listings AS listing ON listing.id = offer.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE offer.id = $1 AND offer.status = 'accepted'
          AND listing.status = 'active' AND property.landlord_id = $2
          AND ($3::uuid IS NULL OR EXISTS (
              SELECT 1 FROM tenants AS tenant
              WHERE tenant.id = $3 AND tenant.property_id = property.id
                AND tenant.status <> 'terminated'
          ))
        RETURNING id, listing_id, offer_id, tenant_id, amount, status,
                  completed_at, created_at, updated_at
        "#,
    )
    .bind(input.offer_id)
    .bind(landlord_id)
    .bind(input.tenant_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_landlord_transaction_status(
    db: &PgPool,
    landlord_id: Uuid,
    transaction_id: Uuid,
    status: &str,
) -> Result<Option<RentalTransaction>, AppError> {
    let mut tx = db.begin().await?;
    let current = sqlx::query_as::<_, RentalTransaction>(
        r#"
        SELECT txn.id, txn.listing_id, txn.offer_id, txn.tenant_id,
               txn.amount, txn.status, txn.completed_at, txn.created_at, txn.updated_at
        FROM transactions AS txn
        INNER JOIN listings AS listing ON listing.id = txn.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1 AND txn.id = $2
        FOR UPDATE OF txn
        "#,
    )
    .bind(landlord_id)
    .bind(transaction_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(current) = current else {
        tx.rollback().await?;
        return Ok(None);
    };
    if current.status != "pending" {
        tx.rollback().await?;
        return Err(AppError::Conflict(
            "transaction status changed; only pending transactions can be updated".to_string(),
        ));
    }

    let completed_at = if status == "completed" {
        Some(Utc::now())
    } else {
        None
    };

    let updated = sqlx::query_as::<_, RentalTransaction>(
        r#"
        UPDATE transactions
        SET status = $2, completed_at = $3
        WHERE id = $1
        RETURNING id, listing_id, offer_id, tenant_id, amount, status,
                  completed_at, created_at, updated_at
        "#,
    )
    .bind(transaction_id)
    .bind(status)
    .bind(completed_at)
    .fetch_one(&mut *tx)
    .await?;

    if status == "completed" {
        let result = sqlx::query(
            "UPDATE listings SET status = 'rented' WHERE id = $1 AND status = 'active'",
        )
        .bind(current.listing_id)
        .execute(&mut *tx)
        .await?;
        if result.rows_affected() != 1 {
            return Err(AppError::Conflict(
                "the listing is no longer available to complete this transaction".to_string(),
            ));
        }
    }

    tx.commit().await?;
    Ok(Some(updated))
}
