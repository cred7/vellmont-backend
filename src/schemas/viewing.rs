use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct ViewingSchema {
    pub listing_id: Uuid,
    pub lead_id: Option<Uuid>,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateViewingSchema {
    pub scheduled_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub status: Option<String>,
}
