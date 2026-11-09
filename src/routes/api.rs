use axum::{Router, middleware, routing::get};

use crate::{handlers::units::get_public_unit_by_slug, state::AppState};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", super::auth::router(state.clone()))
        .nest("/landlords", super::landlords::router(state.clone()))
        .nest("/leads", super::leads::router(state.clone()))
        .nest("/properties", super::properties::router(state.clone()))
        .nest("/units", super::units::router(state.clone()))
        .nest("/listings", super::listings::router(state.clone()))
        .nest("/tenants", super::tenants::router(state.clone()))
        .nest("/viewings", super::viewings::router(state.clone()))
        .nest("/offers", super::offers::router(state.clone()))
        .nest("/transactions", super::transactions::router(state.clone()))
        .route(
            "/properties/{property_slug}/units/{unit_slug}",
            get(get_public_unit_by_slug),
        )
        .route_layer(middleware::from_fn(
            crate::middleware::logging::logging_middleware,
        ))
}

#[cfg(test)]
mod tests {
    use super::router;
    use crate::state::AppState;

    #[tokio::test]
    async fn api_routes_register_without_conflicts() {
        let db = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://test:test@127.0.0.1/test")
            .expect("a lazy pool should accept a valid PostgreSQL URL");

        let _router = router(AppState {
            db,
            jwt_secret: "a sufficiently long test secret".to_string(),
        });
    }
}
