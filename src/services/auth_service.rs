use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::user::{AuthClaims, AuthResponse, LoginRequest, RegisterRequest, User},
    repositories::user_repository::{create_user, find_user_by_email},
};

pub fn decode_token(token: &str, jwt_secret: &str) -> Result<AuthClaims, AppError> {
    let data = decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|_| AppError::Unauthorized)?;

    Ok(data.claims)
}

fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|error| {
            tracing::error!(%error, "failed to hash password");
            AppError::Internal
        })
}

fn verify_password(password: &str, encoded_hash: &str) -> Result<(), AppError> {
    let password_hash = PasswordHash::new(encoded_hash).map_err(|error| {
        tracing::error!(%error, "stored password hash is invalid");
        AppError::Internal
    })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|_| AppError::Unauthorized)
}

fn issue_token(user: &User, jwt_secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = AuthClaims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.clone(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(24)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|error| {
        tracing::error!(%error, "failed to issue authentication token");
        AppError::Internal
    })
}

pub async fn register_user(
    db: &PgPool,
    jwt_secret: &str,
    input: RegisterRequest,
) -> Result<AuthResponse, AppError> {
    if input.email.trim().is_empty() {
        return Err(AppError::BadRequest("email is required".to_string()));
    }

    if input.password.chars().count() < 8 {
        return Err(AppError::BadRequest(
            "password must be at least 8 characters".to_string(),
        ));
    }
    if input.password.len() > 1024 {
        return Err(AppError::BadRequest(
            "password must be no more than 1024 bytes".to_string(),
        ));
    }

    if input.role.as_deref().is_some_and(|role| role != "landlord") {
        return Err(AppError::BadRequest(
            "only landlord accounts can be registered".to_string(),
        ));
    }

    let email = input.email.trim().to_ascii_lowercase();
    if find_user_by_email(db, &email).await?.is_some() {
        return Err(AppError::Conflict("user already exists".to_string()));
    }

    let password_hash = hash_password(&input.password)?;

    let user = User {
        id: Uuid::new_v4(),
        email,
        password_hash,
        full_name: input.full_name,
        role: "landlord".to_string(),
        created_at: Utc::now(),
    };
    let saved = create_user(db, &user).await?;

    let token = issue_token(&saved, jwt_secret)?;

    Ok(AuthResponse {
        token,
        user: saved.to_public(),
    })
}

pub async fn login_user(
    db: &PgPool,
    jwt_secret: &str,
    input: LoginRequest,
) -> Result<AuthResponse, AppError> {
    if input.email.trim().is_empty() {
        return Err(AppError::BadRequest("email is required".to_string()));
    }

    if input.password.is_empty() {
        return Err(AppError::BadRequest("password is required".to_string()));
    }
    if input.password.len() > 1024 {
        return Err(AppError::BadRequest(
            "password must be no more than 1024 bytes".to_string(),
        ));
    }

    let found = find_user_by_email(db, &input.email.trim().to_ascii_lowercase()).await?;

    let user = match found {
        Some(u) => u,
        None => return Err(AppError::Unauthorized),
    };

    verify_password(&input.password, &user.password_hash)?;

    let token = issue_token(&user, jwt_secret)?;

    Ok(AuthResponse {
        token,
        user: user.to_public(),
    })
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};
    use crate::error::AppError;

    #[test]
    fn passwords_are_hashed_and_verified() {
        let password = "a long test password";
        let encoded_hash = hash_password(password).expect("password hashing should succeed");

        assert_ne!(encoded_hash, password);
        assert!(verify_password(password, &encoded_hash).is_ok());
        assert!(matches!(
            verify_password("wrong password", &encoded_hash),
            Err(AppError::Unauthorized)
        ));
    }
}
