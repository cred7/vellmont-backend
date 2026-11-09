use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type CreatePropertyRequest = crate::schemas::property::PropertySchema;
pub type UpdatePropertyRequest = crate::schemas::property::UpdatePropertySchema;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Property {
    pub id: Uuid,
    pub landlord_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub address_line1: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
