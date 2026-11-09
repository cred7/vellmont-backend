use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, models::lead::Lead, schemas::lead::LeadSchema};

pub async fn create_public_lead(db: &PgPool, input: &LeadSchema) -> Result<Option<Lead>, AppError> {
    sqlx::query_as::<_, Lead>(
        r#"
        INSERT INTO leads (listing_id, property_id, unit_id, name, email, phone, source, status)
        SELECT listing.id, listing.property_id, listing.unit_id, $2, lower(trim($3)), $4, $5, 'new'
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE listing.id = $1 AND listing.status = 'active' AND property.status = 'active'
        RETURNING id, listing_id, property_id, unit_id, name, email, phone, source,
                  status, created_at, updated_at
        "#,
    )
    .bind(input.listing_id)
    .bind(input.name.trim())
    .bind(input.email.trim())
    .bind(input.phone.as_deref())
    .bind(input.source.as_deref())
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn list_landlord_leads(db: &PgPool, landlord_id: Uuid) -> Result<Vec<Lead>, AppError> {
    sqlx::query_as::<_, Lead>(
        r#"
        SELECT lead.id, lead.listing_id, lead.property_id, lead.unit_id, lead.name, lead.email,
               lead.phone, lead.source, lead.status, lead.created_at, lead.updated_at
        FROM leads AS lead
        INNER JOIN properties AS property ON property.id = lead.property_id
        WHERE property.landlord_id = $1
        ORDER BY lead.created_at DESC
        "#,
    )
    .bind(landlord_id)
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_landlord_lead(
    db: &PgPool,
    landlord_id: Uuid,
    lead_id: Uuid,
) -> Result<Option<Lead>, AppError> {
    sqlx::query_as::<_, Lead>(
        r#"
        SELECT lead.id, lead.listing_id, lead.property_id, lead.unit_id, lead.name, lead.email,
               lead.phone, lead.source, lead.status, lead.created_at, lead.updated_at
        FROM leads AS lead
        INNER JOIN properties AS property ON property.id = lead.property_id
        WHERE property.landlord_id = $1 AND lead.id = $2
        "#,
    )
    .bind(landlord_id)
    .bind(lead_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_lead_status(
    db: &PgPool,
    landlord_id: Uuid,
    lead_id: Uuid,
    status: &str,
) -> Result<Option<Lead>, AppError> {
    sqlx::query_as::<_, Lead>(
        r#"
        UPDATE leads AS lead SET status = $3
        FROM properties AS property
        WHERE property.id = lead.property_id AND property.landlord_id = $1 AND lead.id = $2
        RETURNING lead.id, lead.listing_id, lead.property_id, lead.unit_id, lead.name, lead.email,
                  lead.phone, lead.source, lead.status, lead.created_at, lead.updated_at
        "#,
    )
    .bind(landlord_id)
    .bind(lead_id)
    .bind(status)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}
