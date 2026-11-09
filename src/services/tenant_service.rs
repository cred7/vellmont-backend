use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::tenant::Tenant,
    repositories::tenant_repository,
    schemas::tenant::{TenantSchema, UpdateTenantSchema},
    services::lead_service::{validate_email, validate_length},
};

const TENANT_STATUSES: &[&str] = &["pending", "active", "terminated"];

pub async fn list_tenants(db: &PgPool, landlord_id: Uuid) -> Result<Vec<Tenant>, AppError> {
    tenant_repository::list_tenants(db, landlord_id).await
}

pub async fn get_tenant(
    db: &PgPool,
    landlord_id: Uuid,
    tenant_id: Uuid,
) -> Result<Tenant, AppError> {
    tenant_repository::get_tenant(db, landlord_id, tenant_id)
        .await?
        .ok_or_else(|| AppError::NotFound("tenant not found".to_string()))
}

pub async fn create_tenant(
    db: &PgPool,
    landlord_id: Uuid,
    mut input: TenantSchema,
) -> Result<Tenant, AppError> {
    input.full_name = input.full_name.trim().to_string();
    input.email = input.email.trim().to_ascii_lowercase();
    validate_contact(&input.full_name, &input.email)?;
    if let Some(phone) = input.phone.as_mut() {
        *phone = phone.trim().to_string();
        validate_length(phone, "phone", 50)?;
    }
    tenant_repository::create_tenant(db, landlord_id, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("property or unit not found".to_string()))
}

pub async fn update_tenant(
    db: &PgPool,
    landlord_id: Uuid,
    tenant_id: Uuid,
    mut input: UpdateTenantSchema,
) -> Result<Tenant, AppError> {
    if let Some(name) = input.full_name.as_mut() {
        *name = name.trim().to_string();
        if name.is_empty() {
            return Err(AppError::BadRequest("full_name is required".to_string()));
        }
    }
    if let Some(email) = input.email.as_mut() {
        *email = email.trim().to_ascii_lowercase();
        validate_email(email)?;
    }
    if let Some(status) = input.status.as_deref() {
        if !TENANT_STATUSES.contains(&status) {
            return Err(AppError::BadRequest("invalid tenant status".to_string()));
        }
        let current = get_tenant(db, landlord_id, tenant_id).await?;
        if !valid_transition(&current.status, status) {
            return Err(AppError::BadRequest(format!(
                "tenant cannot transition from {} to {}",
                current.status, status
            )));
        }
    }
    if let Some(phone) = input.phone.as_mut() {
        *phone = phone.trim().to_string();
        validate_length(phone, "phone", 50)?;
    }
    if let Some(name) = input.full_name.as_deref() {
        validate_length(name, "full_name", 255)?;
    }
    tenant_repository::update_tenant(db, landlord_id, tenant_id, &input)
        .await?
        .ok_or_else(|| AppError::NotFound("tenant or unit not found".to_string()))
}

pub async fn terminate_tenant(
    db: &PgPool,
    landlord_id: Uuid,
    tenant_id: Uuid,
) -> Result<(), AppError> {
    if tenant_repository::terminate_tenant(db, landlord_id, tenant_id).await? {
        Ok(())
    } else {
        Err(AppError::NotFound("tenant not found".to_string()))
    }
}

fn validate_contact(name: &str, email: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::BadRequest("full_name is required".to_string()));
    }
    validate_email(email)
}

fn valid_transition(current: &str, next: &str) -> bool {
    match current {
        "pending" => matches!(next, "active" | "terminated"),
        "active" => next == "terminated",
        _ => false,
    }
}
