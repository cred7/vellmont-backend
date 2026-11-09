use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySchema {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub address_line1: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePropertySchema {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub address_line1: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub status: Option<String>,
}
