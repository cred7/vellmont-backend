use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{
    handlers::tenants::{create_tenant, get_tenant, list_tenants, terminate_tenant, update_tenant},
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_tenants).post(create_tenant))
        .route(
            "/{id}",
            get(get_tenant)
                .patch(update_tenant)
                .delete(terminate_tenant),
        )
        .route_layer(from_fn_with_state(state, auth_middleware))
}
