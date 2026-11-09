use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{
    handlers::units::{archive_unit, create_unit, get_unit_by_id, list_units, update_unit},
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_units).post(create_unit))
        .route(
            "/{unit_id}",
            get(get_unit_by_id).patch(update_unit).delete(archive_unit),
        )
        .route_layer(from_fn_with_state(state, auth_middleware))
}
