use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitSchema {
    pub property_id: Uuid,
    pub slug: Option<String>,
    pub unit_number: String,
    pub bedrooms: i32,
    pub bathrooms: BigDecimal,
    pub sq_ft: Option<i32>,
    pub monthly_rent: Option<BigDecimal>,
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUnitSchema {
    pub slug: Option<String>,
    pub unit_number: Option<String>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<BigDecimal>,
    pub sq_ft: Option<i32>,
    pub monthly_rent: Option<BigDecimal>,
    pub availability_status: Option<String>,
}
