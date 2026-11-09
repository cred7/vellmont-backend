use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::viewing::Viewing,
    schemas::viewing::{UpdateViewingSchema, ViewingSchema},
};

pub async fn create_public_viewing(
    db: &PgPool,
    input: &ViewingSchema,
) -> Result<Option<Viewing>, AppError> {
    sqlx::query_as::<_, Viewing>(
        r#"
        INSERT INTO viewings (
            listing_id, lead_id, contact_name, contact_email, contact_phone,
            scheduled_at, notes, status
        )
        SELECT listing.id, $2, $3, lower(trim($4)), $5, $6, $7, 'requested'
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE listing.id = $1 AND listing.status = 'active' AND property.status = 'active'
          AND ($2::uuid IS NULL OR EXISTS (
              SELECT 1 FROM leads AS lead
              WHERE lead.id = $2 AND lead.listing_id = listing.id
          ))
        RETURNING id, listing_id, lead_id, contact_name, contact_email, contact_phone,
                  scheduled_at, notes, status, created_at, updated_at
        "#,
    )
    .bind(input.listing_id)
    .bind(input.lead_id)
    .bind(input.contact_name.trim())
    .bind(input.contact_email.trim())
    .bind(input.contact_phone.as_deref())
    .bind(input.scheduled_at)
    .bind(input.notes.as_deref())
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn list_landlord_viewings(
    db: &PgPool,
    landlord_id: Uuid,
) -> Result<Vec<Viewing>, AppError> {
    sqlx::query_as::<_, Viewing>(
        r#"
        SELECT viewing.id, viewing.listing_id, viewing.lead_id, viewing.contact_name,
               viewing.contact_email, viewing.contact_phone, viewing.scheduled_at, viewing.notes,
               viewing.status, viewing.created_at, viewing.updated_at
        FROM viewings AS viewing
        INNER JOIN listings AS listing ON listing.id = viewing.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1
        ORDER BY viewing.scheduled_at DESC
        "#,
    )
    .bind(landlord_id)
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_landlord_viewing(
    db: &PgPool,
    landlord_id: Uuid,
    viewing_id: Uuid,
) -> Result<Option<Viewing>, AppError> {
    sqlx::query_as::<_, Viewing>(
        r#"
        SELECT viewing.id, viewing.listing_id, viewing.lead_id, viewing.contact_name,
               viewing.contact_email, viewing.contact_phone, viewing.scheduled_at, viewing.notes,
               viewing.status, viewing.created_at, viewing.updated_at
        FROM viewings AS viewing
        INNER JOIN listings AS listing ON listing.id = viewing.listing_id
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1 AND viewing.id = $2
        "#,
    )
    .bind(landlord_id)
    .bind(viewing_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_viewing(
    db: &PgPool,
    landlord_id: Uuid,
    viewing_id: Uuid,
    input: &UpdateViewingSchema,
) -> Result<Option<Viewing>, AppError> {
    sqlx::query_as::<_, Viewing>(
        r#"
        UPDATE viewings AS viewing
        SET scheduled_at = COALESCE($3, viewing.scheduled_at),
            notes = COALESCE($4, viewing.notes),
            status = COALESCE($5, viewing.status)
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE listing.id = viewing.listing_id AND property.landlord_id = $1
          AND viewing.id = $2
        RETURNING viewing.id, viewing.listing_id, viewing.lead_id, viewing.contact_name,
                  viewing.contact_email, viewing.contact_phone, viewing.scheduled_at, viewing.notes,
                  viewing.status, viewing.created_at, viewing.updated_at
        "#,
    )
    .bind(landlord_id)
    .bind(viewing_id)
    .bind(input.scheduled_at)
    .bind(input.notes.as_deref())
    .bind(input.status.as_deref())
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn cancel_viewing(
    db: &PgPool,
    landlord_id: Uuid,
    viewing_id: Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE viewings AS viewing SET status = 'cancelled'
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE listing.id = viewing.listing_id AND property.landlord_id = $1
          AND viewing.id = $2 AND viewing.status NOT IN ('cancelled', 'completed')
        "#,
    )
    .bind(landlord_id)
    .bind(viewing_id)
    .execute(db)
    .await?;
    Ok(result.rows_affected() > 0)
}
