use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct TenantSchema {
    pub property_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub move_in_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTenantSchema {
    pub unit_id: Option<Uuid>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    pub move_in_date: Option<NaiveDate>,
}
