use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError, models::landlord::LandlordProfile, schemas::landlord::LandlordSchema,
};

pub async fn get_profile(
    db: &PgPool,
    landlord_id: Uuid,
) -> Result<Option<LandlordProfile>, AppError> {
    sqlx::query_as::<_, LandlordProfile>(
        "SELECT id, email, full_name, phone, created_at FROM users WHERE id = $1 AND role = 'landlord'",
    )
    .bind(landlord_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_profile(
    db: &PgPool,
    landlord_id: Uuid,
    input: &LandlordSchema,
) -> Result<Option<LandlordProfile>, AppError> {
    sqlx::query_as::<_, LandlordProfile>(
        r#"
        UPDATE users
        SET full_name = COALESCE($2, full_name), phone = COALESCE($3, phone)
        WHERE id = $1 AND role = 'landlord'
        RETURNING id, email, full_name, phone, created_at
        "#,
    )
    .bind(landlord_id)
    .bind(input.full_name.as_deref())
    .bind(input.phone.as_deref())
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}
