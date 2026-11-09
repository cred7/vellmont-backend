use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingSchema {
    pub property_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub status: Option<String>,
    pub asking_price: Option<BigDecimal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateListingSchema {
    pub asking_price: Option<BigDecimal>,
    pub status: Option<String>,
}
