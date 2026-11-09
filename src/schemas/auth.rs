use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterSchema {
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub role: Option<String>,
}
