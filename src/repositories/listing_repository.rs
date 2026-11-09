use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::listing::{CreateListingRequest, Listing, PublicListing, UpdateListingRequest},
};

pub async fn list_public_listings(db: &PgPool) -> Result<Vec<PublicListing>, AppError> {
    sqlx::query_as::<_, PublicListing>(
        r#"
        SELECT listing.id, property.id AS property_id, property.slug AS property_slug,
               property.name AS property_name, unit.id AS unit_id, unit.slug AS unit_slug,
               unit.unit_number, listing.status, listing.asking_price, listing.created_at
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        LEFT JOIN units AS unit
            ON unit.id = listing.unit_id AND unit.availability_status <> 'archived'
        WHERE listing.status = 'active' AND property.status = 'active'
          AND (listing.unit_id IS NULL OR unit.id IS NOT NULL)
        ORDER BY listing.created_at DESC
        "#,
    )
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_public_listing(
    db: &PgPool,
    listing_id: Uuid,
) -> Result<Option<PublicListing>, AppError> {
    sqlx::query_as::<_, PublicListing>(
        r#"
        SELECT listing.id, property.id AS property_id, property.slug AS property_slug,
               property.name AS property_name, unit.id AS unit_id, unit.slug AS unit_slug,
               unit.unit_number, listing.status, listing.asking_price, listing.created_at
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        LEFT JOIN units AS unit
            ON unit.id = listing.unit_id AND unit.availability_status <> 'archived'
        WHERE listing.id = $1 AND listing.status = 'active' AND property.status = 'active'
          AND (listing.unit_id IS NULL OR unit.id IS NOT NULL)
        "#,
    )
    .bind(listing_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn list_landlord_listings(
    db: &PgPool,
    landlord_id: Uuid,
) -> Result<Vec<Listing>, AppError> {
    sqlx::query_as::<_, Listing>(
        r#"
        SELECT listing.id, listing.property_id, listing.unit_id, listing.status,
               listing.asking_price, listing.created_at
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1
        ORDER BY listing.created_at DESC
        "#,
    )
    .bind(landlord_id)
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_landlord_listing(
    db: &PgPool,
    landlord_id: Uuid,
    listing_id: Uuid,
) -> Result<Option<Listing>, AppError> {
    sqlx::query_as::<_, Listing>(
        r#"
        SELECT listing.id, listing.property_id, listing.unit_id, listing.status,
               listing.asking_price, listing.created_at
        FROM listings AS listing
        INNER JOIN properties AS property ON property.id = listing.property_id
        WHERE property.landlord_id = $1 AND listing.id = $2
        "#,
    )
    .bind(landlord_id)
    .bind(listing_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn create_listing(
    db: &PgPool,
    landlord_id: Uuid,
    input: &CreateListingRequest,
) -> Result<Option<Listing>, AppError> {
    sqlx::query_as::<_, Listing>(
        r#"
        INSERT INTO listings (property_id, unit_id, status, asking_price)
        SELECT property.id, unit.id, 'draft', $4
        FROM properties AS property
        LEFT JOIN units AS unit
            ON unit.id = $2 AND unit.property_id = property.id
        WHERE property.id = $1 AND property.landlord_id = $3
          AND property.status <> 'archived'
          AND ($2::uuid IS NULL OR unit.id IS NOT NULL)
        RETURNING id, property_id, unit_id, status, asking_price, created_at
        "#,
    )
    .bind(input.property_id)
    .bind(input.unit_id)
    .bind(landlord_id)
    .bind(&input.asking_price)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_listing(
    db: &PgPool,
    landlord_id: Uuid,
    listing_id: Uuid,
    input: &UpdateListingRequest,
) -> Result<Option<Listing>, AppError> {
    sqlx::query_as::<_, Listing>(
        r#"
        UPDATE listings AS listing
        SET asking_price = COALESCE($3, listing.asking_price),
            status = COALESCE($4, listing.status)
        FROM properties AS property
        WHERE property.id = listing.property_id AND property.landlord_id = $1
          AND listing.id = $2
        RETURNING listing.id, listing.property_id, listing.unit_id, listing.status,
                  listing.asking_price, listing.created_at
        "#,
    )
    .bind(landlord_id)
    .bind(listing_id)
    .bind(&input.asking_price)
    .bind(input.status.as_deref())
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn archive_listing(
    db: &PgPool,
    landlord_id: Uuid,
    listing_id: Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE listings AS listing
        SET status = 'closed'
        FROM properties AS property
        WHERE property.id = listing.property_id AND property.landlord_id = $1
          AND listing.id = $2 AND listing.status <> 'closed'
        "#,
    )
    .bind(landlord_id)
    .bind(listing_id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}
