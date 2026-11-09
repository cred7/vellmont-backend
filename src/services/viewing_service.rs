use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::viewing::Viewing,
    repositories::viewing_repository,
    schemas::viewing::{UpdateViewingSchema, ViewingSchema},
    services::lead_service::{validate_email, validate_length},
};

const VIEWING_STATUSES: &[&str] = &["requested", "confirmed", "completed", "cancelled"];

pub async fn create_public_viewing(
    db: &PgPool,
    mut input: ViewingSchema,
) -> Result<Viewing, AppError> {
    input.contact_name = input.contact_name.trim().to_string();
    input.contact_email = input.contact_email.trim().to_ascii_lowercase();
    if input.contact_name.is_empty() {
        return Err(AppError::BadRequest("contact_name is required".to_string()));
    }
    validate_length(&input.contact_name, "contact_name", 255)?;
    validate_email(&input.contact_email)?;
    if input.scheduled_at <= Utc::now() {
        return Err(AppError::BadRequest(
            "scheduled_at must be in the future".to_string(),
        ));
    }
    if let Some(phone) = input.contact_phone.as_mut() {
        *phone = phone.trim().to_string();
        validate_length(phone, "contact_phone", 50)?;
    }

    viewing_repository::create_public_viewing(db, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("active listing or matching lead not found".to_string()))
}

pub async fn list_landlord_viewings(
    db: &PgPool,
    landlord_id: Uuid,
) -> Result<Vec<Viewing>, AppError> {
    viewing_repository::list_landlord_viewings(db, landlord_id).await
}

pub async fn get_landlord_viewing(
    db: &PgPool,
    landlord_id: Uuid,
    viewing_id: Uuid,
) -> Result<Viewing, AppError> {
    viewing_repository::get_landlord_viewing(db, landlord_id, viewing_id)
        .await?
        .ok_or_else(|| AppError::NotFound("viewing not found".to_string()))
}

pub async fn update_viewing(
    db: &PgPool,
    landlord_id: Uuid,
    viewing_id: Uuid,
    input: UpdateViewingSchema,
) -> Result<Viewing, AppError> {
    if input
        .scheduled_at
        .is_some_and(|scheduled_at| scheduled_at <= Utc::now())
    {
        return Err(AppError::BadRequest(
            "scheduled_at must be in the future".to_string(),
        ));
    }
    if let Some(status) = input.status.as_deref() {
        if !VIEWING_STATUSES.contains(&status) {
            return Err(AppError::BadRequest("invalid viewing status".to_string()));
        }
        let current = get_landlord_viewing(db, landlord_id, viewing_id).await?;
        if !valid_transition(&current.status, status) {
            return Err(AppError::BadRequest(format!(
                "viewing cannot transition from {} to {}",
                current.status, status
            )));
        }
    }
    viewing_repository::update_viewing(db, landlord_id, viewing_id, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("viewing not found".to_string()))
}

pub async fn cancel_viewing(
    db: &PgPool,
    landlord_id: Uuid,
    viewing_id: Uuid,
) -> Result<(), AppError> {
    if viewing_repository::cancel_viewing(db, landlord_id, viewing_id).await? {
        Ok(())
    } else {
        Err(AppError::NotFound(
            "viewing not found or already closed".to_string(),
        ))
    }
}

fn valid_transition(current: &str, next: &str) -> bool {
    match current {
        "requested" => matches!(next, "confirmed" | "cancelled"),
        "confirmed" => matches!(next, "completed" | "cancelled"),
        _ => false,
    }
}
