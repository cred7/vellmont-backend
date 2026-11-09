use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};

use crate::{
    handlers::viewings::{
        cancel_viewing, create_public_viewing, get_landlord_viewing, list_landlord_viewings,
        update_viewing,
    },
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    let protected_state = state.clone();
    Router::new()
        .route("/", post(create_public_viewing))
        .route(
            "/",
            get(list_landlord_viewings)
                .route_layer(from_fn_with_state(protected_state.clone(), auth_middleware)),
        )
        .route(
            "/{id}",
            get(get_landlord_viewing)
                .merge(patch(update_viewing))
                .delete(cancel_viewing)
                .route_layer(from_fn_with_state(protected_state, auth_middleware)),
        )
}
