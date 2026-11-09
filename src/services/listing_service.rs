use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::listing::{CreateListingRequest, Listing, PublicListing, UpdateListingRequest},
    repositories::listing_repository,
};

const LISTING_STATUSES: &[&str] = &["draft", "active", "paused", "rented", "closed"];

pub async fn list_public_listings(db: &PgPool) -> Result<Vec<PublicListing>, AppError> {
    listing_repository::list_public_listings(db).await
}

pub async fn get_public_listing(db: &PgPool, id: Uuid) -> Result<PublicListing, AppError> {
    listing_repository::get_public_listing(db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("listing not found".to_string()))
}

pub async fn list_landlord_listings(
    db: &PgPool,
    landlord_id: Uuid,
) -> Result<Vec<Listing>, AppError> {
    listing_repository::list_landlord_listings(db, landlord_id).await
}

pub async fn get_landlord_listing(
    db: &PgPool,
    landlord_id: Uuid,
    id: Uuid,
) -> Result<Listing, AppError> {
    listing_repository::get_landlord_listing(db, landlord_id, id)
        .await?
        .ok_or_else(|| AppError::NotFound("listing not found".to_string()))
}

pub async fn create_listing(
    db: &PgPool,
    landlord_id: Uuid,
    input: CreateListingRequest,
) -> Result<Listing, AppError> {
    if input
        .status
        .as_deref()
        .is_some_and(|status| status != "draft")
    {
        return Err(AppError::BadRequest(
            "new listings must start in draft status".to_string(),
        ));
    }

    listing_repository::create_listing(db, landlord_id, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("property or unit not found".to_string()))
}

pub async fn update_listing(
    db: &PgPool,
    landlord_id: Uuid,
    id: Uuid,
    input: UpdateListingRequest,
) -> Result<Listing, AppError> {
    if let Some(new_status) = input.status.as_deref() {
        if !LISTING_STATUSES.contains(&new_status) {
            return Err(AppError::BadRequest("invalid listing status".to_string()));
        }
        let current = listing_repository::get_landlord_listing(db, landlord_id, id)
            .await?
            .ok_or_else(|| AppError::NotFound("listing not found".to_string()))?;
        if !valid_transition(&current.status, new_status) {
            return Err(AppError::BadRequest(format!(
                "listing cannot transition from {} to {}",
                current.status, new_status
            )));
        }
    }

    listing_repository::update_listing(db, landlord_id, id, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("listing not found".to_string()))
}

pub async fn archive_listing(db: &PgPool, landlord_id: Uuid, id: Uuid) -> Result<(), AppError> {
    if listing_repository::archive_listing(db, landlord_id, id).await? {
        Ok(())
    } else {
        Err(AppError::NotFound("listing not found".to_string()))
    }
}

fn valid_transition(current: &str, next: &str) -> bool {
    match current {
        "draft" => matches!(next, "active" | "closed"),
        "active" => matches!(next, "paused" | "rented" | "closed"),
        "paused" => matches!(next, "active" | "closed"),
        "rented" => next == "closed",
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::valid_transition;

    #[test]
    fn allows_only_defined_listing_status_transitions() {
        assert!(valid_transition("draft", "active"));
        assert!(valid_transition("active", "paused"));
        assert!(valid_transition("paused", "active"));
        assert!(valid_transition("rented", "closed"));
        assert!(!valid_transition("closed", "active"));
        assert!(!valid_transition("draft", "rented"));
        assert!(!valid_transition("active", "draft"));
    }
}
