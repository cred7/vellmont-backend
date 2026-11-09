use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type CreateUnitRequest = crate::schemas::unit::UnitSchema;
pub type UpdateUnitRequest = crate::schemas::unit::UpdateUnitSchema;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Unit {
    pub id: Uuid,
    pub property_id: Uuid,
    pub slug: String,
    pub unit_number: String,
    pub bedrooms: i32,
    pub bathrooms: sqlx::types::BigDecimal,
    pub sq_ft: Option<i32>,
    pub monthly_rent: Option<sqlx::types::BigDecimal>,
    pub availability_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
