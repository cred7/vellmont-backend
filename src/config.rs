use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in the environment or .env file");
        let jwt_secret =
            env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment or .env file");

        assert!(
            jwt_secret.len() >= 32,
            "JWT_SECRET must contain at least 32 bytes"
        );

        let port = env::var("PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(8080);

        Self {
            database_url,
            jwt_secret,
            port,
        }
    }
}
