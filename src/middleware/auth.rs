use axum::{
    extract::{FromRequestParts, Request, State},
    http::{header::AUTHORIZATION, request::Parts},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{error::AppError, services::auth_service::decode_token, state::AppState};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
}

impl AuthUser {
    pub fn require_landlord(&self) -> Result<(), AppError> {
        if self.role == "landlord" {
            Ok(())
        } else {
            Err(AppError::Forbidden)
        }
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AppError::Unauthorized)?;

    let auth = header.to_str().map_err(|_| AppError::Unauthorized)?;
    let (scheme, token) = auth.split_once(' ').ok_or(AppError::Unauthorized)?;
    if !scheme.eq_ignore_ascii_case("bearer") || token.trim().is_empty() {
        return Err(AppError::Unauthorized);
    }

    let claims = decode_token(token.trim(), &state.jwt_secret)?;
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

    request.extensions_mut().insert(AuthUser {
        user_id,
        email: claims.email,
        role: claims.role,
    });
    Ok(next.run(request).await)
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .ok_or(AppError::Unauthorized)
    }
}
