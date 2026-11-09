use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct LeadSchema {
    pub listing_id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateLeadSchema {
    pub status: String,
}
