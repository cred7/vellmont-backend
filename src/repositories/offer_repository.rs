use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, models::offer::Offer, schemas::offer::OfferSchema};

pub async fn create_public_offer(
    db: &PgPool,
    input: &OfferSchema,
) -> Result<Option<Offer>, AppError> {
    sqlx::query_as::<_, Offer>(
        r#"
        INSERT INTO offers (
            listing_id, lead_id, contact_name, contact_email, contact_phone, amount, message, status
        )
        SELECT listing.id, $2, $3, lower(trim($4)), $5, $6, $7, 'pending'
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE listing.id = $1 AND listing.status = 'active' AND property.status = 'active'
          AND ($2::uuid IS NULL OR EXISTS (
              SELECT 1 FROM leads AS lead
              WHERE lead.id = $2 AND lead.listing_id = listing.id
          ))
        RETURNING id, listing_id, lead_id, contact_name, contact_email, contact_phone,
                  amount, message, status, created_at, updated_at
        "#,
    )
    .bind(input.listing_id)
    .bind(input.lead_id)
    .bind(input.contact_name.trim())
    .bind(input.contact_email.trim())
    .bind(input.contact_phone.as_deref())
    .bind(&input.amount)
    .bind(input.message.as_deref())
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn list_landlord_offers(db: &PgPool, landlord_id: Uuid) -> Result<Vec<Offer>, AppError> {
    sqlx::query_as::<_, Offer>(
        r#"
        SELECT offer.id, offer.listing_id, offer.lead_id, offer.contact_name, offer.contact_email,
               offer.contact_phone, offer.amount, offer.message, offer.status,
               offer.created_at, offer.updated_at
        FROM offers AS offer
        INNER JOIN listings AS listing ON listing.id = offer.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1
        ORDER BY offer.created_at DESC
        "#,
    )
    .bind(landlord_id)
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_landlord_offer(
    db: &PgPool,
    landlord_id: Uuid,
    offer_id: Uuid,
) -> Result<Option<Offer>, AppError> {
    sqlx::query_as::<_, Offer>(
        r#"
        SELECT offer.id, offer.listing_id, offer.lead_id, offer.contact_name, offer.contact_email,
               offer.contact_phone, offer.amount, offer.message, offer.status,
               offer.created_at, offer.updated_at
        FROM offers AS offer
        INNER JOIN listings AS listing ON listing.id = offer.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1 AND offer.id = $2
        "#,
    )
    .bind(landlord_id)
    .bind(offer_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_offer_status(
    db: &PgPool,
    landlord_id: Uuid,
    offer_id: Uuid,
    expected_status: &str,
    status: &str,
) -> Result<Option<Offer>, AppError> {
    let mut tx = db.begin().await?;
    let listing_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT listing.id
        FROM offers AS offer
        INNER JOIN listings AS listing ON listing.id = offer.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1 AND offer.id = $2 AND offer.status = $3
        FOR UPDATE OF listing
        "#,
    )
    .bind(landlord_id)
    .bind(offer_id)
    .bind(expected_status)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(listing_id) = listing_id else {
        tx.rollback().await?;
        return Ok(None);
    };

    if status == "accepted" {
        let accepted_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM offers WHERE listing_id = $1 AND status = 'accepted' AND id <> $2)",
        )
        .bind(listing_id)
        .bind(offer_id)
        .fetch_one(&mut *tx)
        .await?;
        if accepted_exists {
            return Err(AppError::Conflict(
                "another offer has already been accepted for this listing".to_string(),
            ));
        }
    }

    let updated = sqlx::query_as::<_, Offer>(
        r#"
        UPDATE offers SET status = $2
        WHERE id = $1 AND status = $3
        RETURNING id, listing_id, lead_id, contact_name, contact_email, contact_phone,
                  amount, message, status, created_at, updated_at
        "#,
    )
    .bind(offer_id)
    .bind(status)
    .bind(expected_status)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(updated)
}
