use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError, models::lead::Lead, repositories::lead_repository, schemas::lead::LeadSchema,
};

const LEAD_STATUSES: &[&str] = &["new", "contacted", "qualified", "converted", "lost"];

pub async fn create_public_lead(db: &PgPool, mut input: LeadSchema) -> Result<Lead, AppError> {
    input.name = input.name.trim().to_string();
    input.email = input.email.trim().to_ascii_lowercase();
    if input.name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    validate_length(&input.name, "name", 255)?;
    validate_email(&input.email)?;
    if let Some(phone) = input.phone.as_mut() {
        *phone = phone.trim().to_string();
        validate_length(phone, "phone", 50)?;
    }
    if let Some(source) = input.source.as_mut() {
        *source = source.trim().to_string();
        validate_length(source, "source", 100)?;
    }

    lead_repository::create_public_lead(db, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("active listing not found".to_string()))
}

pub async fn list_landlord_leads(db: &PgPool, landlord_id: Uuid) -> Result<Vec<Lead>, AppError> {
    lead_repository::list_landlord_leads(db, landlord_id).await
}

pub async fn get_landlord_lead(
    db: &PgPool,
    landlord_id: Uuid,
    lead_id: Uuid,
) -> Result<Lead, AppError> {
    lead_repository::get_landlord_lead(db, landlord_id, lead_id)
        .await?
        .ok_or_else(|| AppError::NotFound("lead not found".to_string()))
}

pub async fn update_lead_status(
    db: &PgPool,
    landlord_id: Uuid,
    lead_id: Uuid,
    status: String,
) -> Result<Lead, AppError> {
    if !LEAD_STATUSES.contains(&status.as_str()) {
        return Err(AppError::BadRequest("invalid lead status".to_string()));
    }
    let current = get_landlord_lead(db, landlord_id, lead_id).await?;
    if !valid_transition(&current.status, &status) {
        return Err(AppError::BadRequest(format!(
            "lead cannot transition from {} to {}",
            current.status, status
        )));
    }
    lead_repository::update_lead_status(db, landlord_id, lead_id, &status)
        .await?
        .ok_or_else(|| AppError::NotFound("lead not found".to_string()))
}

fn valid_transition(current: &str, next: &str) -> bool {
    match current {
        "new" => matches!(next, "contacted" | "qualified" | "lost"),
        "contacted" => matches!(next, "qualified" | "converted" | "lost"),
        "qualified" => matches!(next, "converted" | "lost"),
        _ => false,
    }
}

pub(crate) fn validate_email(email: &str) -> Result<(), AppError> {
    validate_length(email, "email", 255)?;
    let Some((local, domain)) = email.split_once('@') else {
        return Err(AppError::BadRequest(
            "a valid email is required".to_string(),
        ));
    };
    if local.trim().is_empty()
        || !domain.contains('.')
        || domain.starts_with('.')
        || domain.ends_with('.')
    {
        return Err(AppError::BadRequest(
            "a valid email is required".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn validate_length(value: &str, field: &str, max_chars: usize) -> Result<(), AppError> {
    if value.chars().count() > max_chars {
        return Err(AppError::BadRequest(format!(
            "{field} must be no more than {max_chars} characters"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::valid_transition;

    #[test]
    fn lead_statuses_follow_the_crm_lifecycle() {
        assert!(valid_transition("new", "contacted"));
        assert!(valid_transition("contacted", "converted"));
        assert!(valid_transition("qualified", "lost"));
        assert!(!valid_transition("converted", "contacted"));
        assert!(!valid_transition("lost", "qualified"));
    }
}
