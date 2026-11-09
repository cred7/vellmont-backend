use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::property::{CreatePropertyRequest, Property, UpdatePropertyRequest},
    repositories::property_repository,
    utils::slugify,
};

pub async fn get_properties(db: &PgPool, landlord_id: Uuid) -> Result<Vec<Property>, AppError> {
    property_repository::list_properties(db, landlord_id).await
}

pub async fn get_property(
    db: &PgPool,
    landlord_id: Uuid,
    slug: &str,
) -> Result<Property, AppError> {
    property_repository::get_property(db, landlord_id, slug)
        .await?
        .ok_or_else(|| AppError::NotFound("property not found".to_string()))
}

pub async fn create_property_service(
    db: &PgPool,
    landlord_id: Uuid,
    mut input: CreatePropertyRequest,
) -> Result<Property, AppError> {
    input.slug = slugify(&input.slug);
    if input.slug.is_empty() {
        return Err(AppError::BadRequest(
            "property slug must contain letters or numbers".to_string(),
        ));
    }
    if input.name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "property name is required".to_string(),
        ));
    }
    if input
        .status
        .as_deref()
        .is_some_and(|status| !matches!(status, "draft" | "active"))
    {
        return Err(AppError::BadRequest(
            "property status must be draft or active".to_string(),
        ));
    }

    property_repository::create_property(db, landlord_id, &input).await
}

pub async fn update_property_service(
    db: &PgPool,
    landlord_id: Uuid,
    slug: &str,
    mut input: UpdatePropertyRequest,
) -> Result<Property, AppError> {
    if let Some(value) = input.slug.as_mut() {
        *value = slugify(value);
        if value.is_empty() {
            return Err(AppError::BadRequest(
                "property slug must contain letters or numbers".to_string(),
            ));
        }
    }
    if input
        .status
        .as_deref()
        .is_some_and(|status| !matches!(status, "draft" | "active"))
    {
        return Err(AppError::BadRequest(
            "property status must be draft or active".to_string(),
        ));
    }

    property_repository::update_property(db, landlord_id, slug, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("property not found".to_string()))
}

pub async fn archive_property_service(
    db: &PgPool,
    landlord_id: Uuid,
    slug: &str,
) -> Result<(), AppError> {
    if property_repository::archive_property(db, landlord_id, slug).await? {
        Ok(())
    } else {
        Err(AppError::NotFound("property not found".to_string()))
    }
}
