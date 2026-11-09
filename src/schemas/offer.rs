use serde::Deserialize;
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct OfferSchema {
    pub listing_id: Uuid,
    pub lead_id: Option<Uuid>,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub amount: BigDecimal,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateOfferSchema {
    pub status: String,
}
