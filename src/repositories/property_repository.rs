use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::property::{CreatePropertyRequest, Property, UpdatePropertyRequest},
};

pub async fn list_properties(db: &PgPool, landlord_id: Uuid) -> Result<Vec<Property>, AppError> {
    sqlx::query_as::<_, Property>(
        r#"
        SELECT id, landlord_id, name, slug, description, address_line1, city, state,
               postal_code, country, status, created_at, updated_at
        FROM properties
        WHERE landlord_id = $1 AND status <> 'archived'
        ORDER BY created_at DESC
        "#,
    )
    .bind(landlord_id)
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_property(
    db: &PgPool,
    landlord_id: Uuid,
    slug: &str,
) -> Result<Option<Property>, AppError> {
    sqlx::query_as::<_, Property>(
        r#"
        SELECT id, landlord_id, name, slug, description, address_line1, city, state,
               postal_code, country, status, created_at, updated_at
        FROM properties
        WHERE landlord_id = $1 AND slug = $2 AND status <> 'archived'
        "#,
    )
    .bind(landlord_id)
    .bind(slug)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn create_property(
    db: &PgPool,
    landlord_id: Uuid,
    input: &CreatePropertyRequest,
) -> Result<Property, AppError> {
    sqlx::query_as::<_, Property>(
        r#"
        INSERT INTO properties (
            landlord_id, name, slug, description, address_line1, city, state,
            postal_code, country, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, landlord_id, name, slug, description, address_line1, city, state,
                  postal_code, country, status, created_at, updated_at
        "#,
    )
    .bind(landlord_id)
    .bind(&input.name)
    .bind(&input.slug)
    .bind(&input.description)
    .bind(&input.address_line1)
    .bind(&input.city)
    .bind(&input.state)
    .bind(&input.postal_code)
    .bind(input.country.as_deref().unwrap_or("US"))
    .bind(input.status.as_deref().unwrap_or("draft"))
    .fetch_one(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_property(
    db: &PgPool,
    landlord_id: Uuid,
    slug: &str,
    input: &UpdatePropertyRequest,
) -> Result<Option<Property>, AppError> {
    sqlx::query_as::<_, Property>(
        r#"
        UPDATE properties
        SET name = COALESCE($3, name),
            slug = COALESCE($4, slug),
            description = COALESCE($5, description),
            address_line1 = COALESCE($6, address_line1),
            city = COALESCE($7, city),
            state = COALESCE($8, state),
            postal_code = COALESCE($9, postal_code),
            country = COALESCE($10, country),
            status = COALESCE($11, status)
        WHERE landlord_id = $1 AND slug = $2 AND status <> 'archived'
        RETURNING id, landlord_id, name, slug, description, address_line1, city, state,
                  postal_code, country, status, created_at, updated_at
        "#,
    )
    .bind(landlord_id)
    .bind(slug)
    .bind(input.name.as_deref())
    .bind(input.slug.as_deref())
    .bind(input.description.as_deref())
    .bind(input.address_line1.as_deref())
    .bind(input.city.as_deref())
    .bind(input.state.as_deref())
    .bind(input.postal_code.as_deref())
    .bind(input.country.as_deref())
    .bind(input.status.as_deref())
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn archive_property(
    db: &PgPool,
    landlord_id: Uuid,
    slug: &str,
) -> Result<bool, AppError> {
    let result = sqlx::query(
        "UPDATE properties SET status = 'archived' WHERE landlord_id = $1 AND slug = $2 AND status <> 'archived'",
    )
    .bind(landlord_id)
    .bind(slug)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}
