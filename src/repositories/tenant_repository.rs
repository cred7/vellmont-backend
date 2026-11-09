use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::tenant::Tenant,
    schemas::tenant::{TenantSchema, UpdateTenantSchema},
};

pub async fn list_tenants(db: &PgPool, landlord_id: Uuid) -> Result<Vec<Tenant>, AppError> {
    sqlx::query_as::<_, Tenant>(
        r#"
        SELECT tenant.id, tenant.property_id, tenant.unit_id, tenant.full_name, tenant.email,
               tenant.phone, tenant.status, tenant.move_in_date, tenant.created_at, tenant.updated_at
        FROM tenants AS tenant
        INNER JOIN properties AS property ON property.id = tenant.property_id
        WHERE property.landlord_id = $1
        ORDER BY tenant.created_at DESC
        "#,
    )
    .bind(landlord_id)
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

pub async fn get_tenant(
    db: &PgPool,
    landlord_id: Uuid,
    tenant_id: Uuid,
) -> Result<Option<Tenant>, AppError> {
    sqlx::query_as::<_, Tenant>(
        r#"
        SELECT tenant.id, tenant.property_id, tenant.unit_id, tenant.full_name, tenant.email,
               tenant.phone, tenant.status, tenant.move_in_date, tenant.created_at, tenant.updated_at
        FROM tenants AS tenant
        INNER JOIN properties AS property ON property.id = tenant.property_id
        WHERE property.landlord_id = $1 AND tenant.id = $2
        "#,
    )
    .bind(landlord_id)
    .bind(tenant_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn create_tenant(
    db: &PgPool,
    landlord_id: Uuid,
    input: &TenantSchema,
) -> Result<Option<Tenant>, AppError> {
    sqlx::query_as::<_, Tenant>(
        r#"
        INSERT INTO tenants (property_id, unit_id, full_name, email, phone, status, move_in_date)
        SELECT property.id, unit.id, $3, lower(trim($4)), $5, 'pending', $6
        FROM properties AS property
        LEFT JOIN units AS unit ON unit.id = $2 AND unit.property_id = property.id
        WHERE property.id = $1 AND property.landlord_id = $7
          AND ($2::uuid IS NULL OR unit.id IS NOT NULL)
        RETURNING id, property_id, unit_id, full_name, email, phone, status, move_in_date,
                  created_at, updated_at
        "#,
    )
    .bind(input.property_id)
    .bind(input.unit_id)
    .bind(input.full_name.trim())
    .bind(input.email.trim())
    .bind(input.phone.as_deref())
    .bind(input.move_in_date)
    .bind(landlord_id)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn update_tenant(
    db: &PgPool,
    landlord_id: Uuid,
    tenant_id: Uuid,
    input: &UpdateTenantSchema,
) -> Result<Option<Tenant>, AppError> {
    sqlx::query_as::<_, Tenant>(
        r#"
        UPDATE tenants AS tenant
        SET unit_id = COALESCE($3, tenant.unit_id),
            full_name = COALESCE($4, tenant.full_name),
            email = COALESCE(lower(trim($5)), tenant.email),
            phone = COALESCE($6, tenant.phone),
            status = COALESCE($7, tenant.status),
            move_in_date = COALESCE($8, tenant.move_in_date)
        FROM properties AS property
        LEFT JOIN units AS unit ON unit.id = $3 AND unit.property_id = property.id
        WHERE property.id = tenant.property_id AND property.landlord_id = $1
          AND tenant.id = $2 AND ($3::uuid IS NULL OR unit.id IS NOT NULL)
        RETURNING tenant.id, tenant.property_id, tenant.unit_id, tenant.full_name, tenant.email,
                  tenant.phone, tenant.status, tenant.move_in_date, tenant.created_at, tenant.updated_at
        "#,
    )
    .bind(landlord_id)
    .bind(tenant_id)
    .bind(input.unit_id)
    .bind(input.full_name.as_deref().map(str::trim))
    .bind(input.email.as_deref())
    .bind(input.phone.as_deref())
    .bind(input.status.as_deref())
    .bind(input.move_in_date)
    .fetch_optional(db)
    .await
    .map_err(AppError::from)
}

pub async fn terminate_tenant(
    db: &PgPool,
    landlord_id: Uuid,
    tenant_id: Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE tenants AS tenant SET status = 'terminated'
        FROM properties AS property
        WHERE property.id = tenant.property_id AND property.landlord_id = $1
          AND tenant.id = $2 AND tenant.status <> 'terminated'
        "#,
    )
    .bind(landlord_id)
    .bind(tenant_id)
    .execute(db)
    .await?;
    Ok(result.rows_affected() > 0)
}
