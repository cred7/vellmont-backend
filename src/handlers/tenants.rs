use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::tenant::Tenant,
    schemas::tenant::{TenantSchema, UpdateTenantSchema},
    services::tenant_service,
    state::AppState,
};

pub async fn list_tenants(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Tenant>>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        tenant_service::list_tenants(&state.db, auth_user.user_id).await?,
    ))
}

pub async fn get_tenant(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Tenant>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        tenant_service::get_tenant(&state.db, auth_user.user_id, id).await?,
    ))
}

pub async fn create_tenant(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<TenantSchema>,
) -> Result<(StatusCode, Json<Tenant>), AppError> {
    auth_user.require_landlord()?;
    let tenant = tenant_service::create_tenant(&state.db, auth_user.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(tenant)))
}

pub async fn update_tenant(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTenantSchema>,
) -> Result<Json<Tenant>, AppError> {
    auth_user.require_landlord()?;
    Ok(Json(
        tenant_service::update_tenant(&state.db, auth_user.user_id, id, payload).await?,
    ))
}

pub async fn terminate_tenant(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    auth_user.require_landlord()?;
    tenant_service::terminate_tenant(&state.db, auth_user.user_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
