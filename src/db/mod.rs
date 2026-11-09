use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::config::AppConfig;

pub async fn create_db_pool(config: &AppConfig) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
}
