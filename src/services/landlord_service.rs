use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError, models::landlord::LandlordProfile, repositories::landlord_repository,
    schemas::landlord::LandlordSchema, services::lead_service::validate_length,
};

pub async fn get_profile(db: &PgPool, landlord_id: Uuid) -> Result<LandlordProfile, AppError> {
    landlord_repository::get_profile(db, landlord_id)
        .await?
        .ok_or_else(|| AppError::NotFound("landlord profile not found".to_string()))
}

pub async fn update_profile(
    db: &PgPool,
    landlord_id: Uuid,
    mut input: LandlordSchema,
) -> Result<LandlordProfile, AppError> {
    if input
        .full_name
        .as_deref()
        .is_some_and(|value| value.trim().is_empty())
    {
        return Err(AppError::BadRequest(
            "full_name cannot be blank".to_string(),
        ));
    }

    if let Some(name) = input.full_name.as_mut() {
        *name = name.trim().to_string();
        validate_length(name, "full_name", 255)?;
    }
    if let Some(phone) = input.phone.as_mut() {
        *phone = phone.trim().to_string();
        validate_length(phone, "phone", 50)?;
    }

    landlord_repository::update_profile(db, landlord_id, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("landlord profile not found".to_string()))
}
