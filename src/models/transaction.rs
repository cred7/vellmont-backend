use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RentalTransaction {
    pub id: Uuid,
    pub listing_id: Option<Uuid>,
    pub offer_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub amount: BigDecimal,
    pub status: String,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
