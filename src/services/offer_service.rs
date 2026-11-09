use sqlx::PgPool;
use sqlx::types::BigDecimal;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::offer::Offer,
    repositories::offer_repository,
    schemas::offer::OfferSchema,
    services::lead_service::{validate_email, validate_length},
};

const OFFER_STATUSES: &[&str] = &["pending", "accepted", "rejected", "withdrawn"];

pub async fn create_public_offer(db: &PgPool, mut input: OfferSchema) -> Result<Offer, AppError> {
    input.contact_name = input.contact_name.trim().to_string();
    input.contact_email = input.contact_email.trim().to_ascii_lowercase();
    if input.contact_name.is_empty() {
        return Err(AppError::BadRequest("contact_name is required".to_string()));
    }
    validate_length(&input.contact_name, "contact_name", 255)?;
    validate_email(&input.contact_email)?;
    if input.amount <= BigDecimal::from(0)
        || input.amount > BigDecimal::from(999_999_999_999_i64) / BigDecimal::from(100)
    {
        return Err(AppError::BadRequest(
            "offer amount must be greater than zero and no more than 9999999999.99".to_string(),
        ));
    }
    if let Some(phone) = input.contact_phone.as_mut() {
        *phone = phone.trim().to_string();
        validate_length(phone, "contact_phone", 50)?;
    }
    offer_repository::create_public_offer(db, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("active listing or matching lead not found".to_string()))
}

pub async fn list_landlord_offers(db: &PgPool, landlord_id: Uuid) -> Result<Vec<Offer>, AppError> {
    offer_repository::list_landlord_offers(db, landlord_id).await
}

pub async fn get_landlord_offer(
    db: &PgPool,
    landlord_id: Uuid,
    offer_id: Uuid,
) -> Result<Offer, AppError> {
    offer_repository::get_landlord_offer(db, landlord_id, offer_id)
        .await?
        .ok_or_else(|| AppError::NotFound("offer not found".to_string()))
}

pub async fn update_offer_status(
    db: &PgPool,
    landlord_id: Uuid,
    offer_id: Uuid,
    status: String,
) -> Result<Offer, AppError> {
    if !OFFER_STATUSES.contains(&status.as_str()) {
        return Err(AppError::BadRequest("invalid offer status".to_string()));
    }

    let current = get_landlord_offer(db, landlord_id, offer_id).await?;
    if !valid_transition(&current.status, &status) {
        return Err(AppError::BadRequest(format!(
            "offer cannot transition from {} to {}",
            current.status, status
        )));
    }

    match offer_repository::update_offer_status(db, landlord_id, offer_id, &current.status, &status)
        .await?
    {
        Some(offer) => Ok(offer),
        None => {
            if offer_repository::get_landlord_offer(db, landlord_id, offer_id)
                .await?
                .is_some()
            {
                Err(AppError::Conflict(
                    "offer status changed; reload and retry".to_string(),
                ))
            } else {
                Err(AppError::NotFound("offer not found".to_string()))
            }
        }
    }
}

fn valid_transition(current: &str, next: &str) -> bool {
    current == "pending" && matches!(next, "accepted" | "rejected" | "withdrawn")
}

#[cfg(test)]
mod tests {
    use super::valid_transition;

    #[test]
    fn offers_can_only_leave_pending_once() {
        assert!(valid_transition("pending", "accepted"));
        assert!(valid_transition("pending", "rejected"));
        assert!(!valid_transition("accepted", "pending"));
        assert!(!valid_transition("rejected", "accepted"));
    }
}
