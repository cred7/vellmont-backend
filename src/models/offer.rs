use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Offer {
    pub id: Uuid,
    pub listing_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub amount: BigDecimal,
    pub message: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
