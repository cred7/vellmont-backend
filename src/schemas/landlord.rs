use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LandlordSchema {
    pub full_name: Option<String>,
    pub phone: Option<String>,
}
