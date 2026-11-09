use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::unit::{CreateUnitRequest, Unit, UpdateUnitRequest},
    repositories::unit_repository,
    utils::slugify,
};

pub async fn get_units(
    db: &PgPool,
    landlord_id: Uuid,
    property_id: Option<Uuid>,
) -> Result<Vec<Unit>, AppError> {
    unit_repository::list_units(db, landlord_id, property_id).await
}

pub async fn get_unit(db: &PgPool, landlord_id: Uuid, unit_id: Uuid) -> Result<Unit, AppError> {
    unit_repository::get_unit(db, landlord_id, unit_id)
        .await?
        .ok_or_else(|| AppError::NotFound("unit not found".to_string()))
}

pub async fn get_public_unit(
    db: &PgPool,
    property_slug: &str,
    unit_slug: &str,
) -> Result<Unit, AppError> {
    unit_repository::get_public_unit(db, property_slug, unit_slug)
        .await?
        .ok_or_else(|| AppError::NotFound("unit not found".to_string()))
}

pub async fn create_unit_service(
    db: &PgPool,
    landlord_id: Uuid,
    mut input: CreateUnitRequest,
) -> Result<Unit, AppError> {
    if input.unit_number.trim().is_empty() {
        return Err(AppError::BadRequest("unit number is required".to_string()));
    }
    let slug = input.slug.as_deref().unwrap_or(input.unit_number.as_str());
    let slug = slugify(slug);
    if slug.is_empty() {
        return Err(AppError::BadRequest(
            "unit slug must contain letters or numbers".to_string(),
        ));
    }
    input.slug = Some(slug);

    unit_repository::create_unit(db, landlord_id, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("property not found".to_string()))
}

pub async fn update_unit_service(
    db: &PgPool,
    landlord_id: Uuid,
    unit_id: Uuid,
    mut input: UpdateUnitRequest,
) -> Result<Unit, AppError> {
    if input.slug.is_none() {
        input.slug = input.unit_number.as_deref().map(slugify);
    } else if let Some(slug) = input.slug.as_mut() {
        *slug = slugify(slug);
    }

    if input.slug.as_deref().is_some_and(str::is_empty) {
        return Err(AppError::BadRequest(
            "unit slug must contain letters or numbers".to_string(),
        ));
    }

    unit_repository::update_unit(db, landlord_id, unit_id, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("unit not found".to_string()))
}

pub async fn archive_unit_service(
    db: &PgPool,
    landlord_id: Uuid,
    unit_id: Uuid,
) -> Result<(), AppError> {
    if unit_repository::archive_unit(db, landlord_id, unit_id).await? {
        Ok(())
    } else {
        Err(AppError::NotFound("unit not found".to_string()))
    }
}
