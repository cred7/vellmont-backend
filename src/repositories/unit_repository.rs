use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::unit::{CreateUnitRequest, Unit, UpdateUnitRequest},
};

pub async fn list_units(
    db: &PgPool,
    landlord_id: Uuid,
    property_id: Option<Uuid>,
) -> Result<Vec<Unit>, AppError> {
    sqlx::query_as::<_, Unit>(
        r#"
        SELECT u.id, u.property_id, u.slug, u.unit_number, u.bedrooms, u.bathrooms,
               u.sq_ft, u.monthly_rent, u.availability_status, u.created_at, u.updated_at
        FROM units AS u
        INNER JOIN properties AS p ON p.id = u.property_id
        WHERE p.landlord_id = $1
          AND p.status <> 'archived'
          AND u.availability_status <> 'archived'
          AND ($2::uuid IS NULL OR u.property_id = $2)
        ORDER BY u.property_id, u.unit_number
        "#,
    )
    .bind(landlord_id)
    .bind(property_id)
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_unit(
    db: &PgPool,
    landlord_id: Uuid,
    unit_id: Uuid,
) -> Result<Option<Unit>, AppError> {
    sqlx::query_as::<_, Unit>(
        r#"
        SELECT u.id, u.property_id, u.slug, u.unit_number, u.bedrooms, u.bathrooms,
               u.sq_ft, u.monthly_rent, u.availability_status, u.created_at, u.updated_at
        FROM units AS u
        INNER JOIN properties AS p ON p.id = u.property_id
        WHERE p.landlord_id = $1 AND u.id = $2
          AND p.status <> 'archived' AND u.availability_status <> 'archived'
        "#,
    )
    .bind(landlord_id)
    .bind(unit_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_public_unit(
    db: &PgPool,
    property_slug: &str,
    unit_slug: &str,
) -> Result<Option<Unit>, AppError> {
    sqlx::query_as::<_, Unit>(
        r#"
        SELECT u.id, u.property_id, u.slug, u.unit_number, u.bedrooms, u.bathrooms,
               u.sq_ft, u.monthly_rent, u.availability_status, u.created_at, u.updated_at
        FROM units AS u
        INNER JOIN properties AS p ON p.id = u.property_id
        WHERE p.slug = $1 AND u.slug = $2
          AND p.status = 'active'
          AND u.availability_status <> 'archived'
          AND EXISTS (
              SELECT 1
              FROM listings AS listing
              WHERE listing.property_id = p.id
                AND (listing.unit_id = u.id OR listing.unit_id IS NULL)
                AND listing.status = 'active'
          )
        "#,
    )
    .bind(property_slug)
    .bind(unit_slug)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn create_unit(
    db: &PgPool,
    landlord_id: Uuid,
    input: &CreateUnitRequest,
) -> Result<Option<Unit>, AppError> {
    sqlx::query_as::<_, Unit>(
        r#"
        INSERT INTO units (
            property_id, slug, unit_number, bedrooms, bathrooms, sq_ft,
            monthly_rent, availability_status
        )
        SELECT p.id, $3, $4, $5, $6, $7, $8, $9
        FROM properties AS p
        WHERE p.id = $1 AND p.landlord_id = $2 AND p.status <> 'archived'
        RETURNING id, property_id, slug, unit_number, bedrooms, bathrooms,
                  sq_ft, monthly_rent, availability_status, created_at, updated_at
        "#,
    )
    .bind(input.property_id)
    .bind(landlord_id)
    .bind(input.slug.as_deref().unwrap_or(&input.unit_number))
    .bind(&input.unit_number)
    .bind(input.bedrooms)
    .bind(&input.bathrooms)
    .bind(input.sq_ft)
    .bind(&input.monthly_rent)
    .bind(input.availability_status.as_deref().unwrap_or("available"))
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_unit(
    db: &PgPool,
    landlord_id: Uuid,
    unit_id: Uuid,
    input: &UpdateUnitRequest,
) -> Result<Option<Unit>, AppError> {
    sqlx::query_as::<_, Unit>(
        r#"
        UPDATE units AS u
        SET slug = COALESCE($3, u.slug),
            unit_number = COALESCE($4, u.unit_number),
            bedrooms = COALESCE($5, u.bedrooms),
            bathrooms = COALESCE($6, u.bathrooms),
            sq_ft = COALESCE($7, u.sq_ft),
            monthly_rent = COALESCE($8, u.monthly_rent),
            availability_status = COALESCE($9, u.availability_status)
        FROM properties AS p
        WHERE p.id = u.property_id AND p.landlord_id = $1 AND u.id = $2
          AND p.status <> 'archived' AND u.availability_status <> 'archived'
        RETURNING u.id, u.property_id, u.slug, u.unit_number, u.bedrooms, u.bathrooms,
                  u.sq_ft, u.monthly_rent, u.availability_status, u.created_at, u.updated_at
        "#,
    )
    .bind(landlord_id)
    .bind(unit_id)
    .bind(input.slug.as_deref())
    .bind(input.unit_number.as_deref())
    .bind(input.bedrooms)
    .bind(input.bathrooms.as_ref())
    .bind(input.sq_ft)
    .bind(input.monthly_rent.as_ref())
    .bind(input.availability_status.as_deref())
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn archive_unit(db: &PgPool, landlord_id: Uuid, unit_id: Uuid) -> Result<bool, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE units AS u
        SET availability_status = 'archived'
        FROM properties AS p
        WHERE p.id = u.property_id AND p.landlord_id = $1 AND u.id = $2
          AND u.availability_status <> 'archived'
        "#,
    )
    .bind(landlord_id)
    .bind(unit_id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}
