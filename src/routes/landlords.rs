use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{
    handlers::landlords::{get_profile, update_profile},
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(get_profile).patch(update_profile))
        .route_layer(from_fn_with_state(state, auth_middleware))
}
