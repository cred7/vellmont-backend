use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type CreateListingRequest = crate::schemas::listing::ListingSchema;
pub type UpdateListingRequest = crate::schemas::listing::UpdateListingSchema;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Listing {
    pub id: Uuid,
    pub property_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub status: String,
    pub asking_price: Option<sqlx::types::BigDecimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PublicListing {
    pub id: Uuid,
    pub property_id: Uuid,
    pub property_slug: String,
    pub property_name: String,
    pub unit_id: Option<Uuid>,
    pub unit_slug: Option<String>,
    pub unit_number: Option<String>,
    pub status: String,
    pub asking_price: Option<sqlx::types::BigDecimal>,
    pub created_at: DateTime<Utc>,
}
