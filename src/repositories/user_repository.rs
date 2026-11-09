use sqlx::PgPool;

use crate::{error::AppError, models::user::User};

pub async fn find_user_by_email(db: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    let normalized = email.trim().to_ascii_lowercase();

    if normalized.is_empty() {
        return Ok(None);
    }

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, full_name, role, created_at
        FROM users
        WHERE lower(email) = $1
        "#,
    )
    .bind(normalized)
    .fetch_optional(db)
    .await?;

    Ok(user)
}

pub async fn create_user(db: &PgPool, user: &User) -> Result<User, AppError> {
    let created = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, password_hash, full_name, role, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, password_hash, full_name, role, created_at
        "#,
    )
    .bind(user.id)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.full_name)
    .bind(&user.role)
    .bind(user.created_at)
    .fetch_one(db)
    .await?;

    Ok(created)
}
