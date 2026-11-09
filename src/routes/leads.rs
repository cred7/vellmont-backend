use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};

use crate::{
    handlers::leads::{
        create_public_lead, get_landlord_lead, list_landlord_leads, update_lead_status,
    },
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    let protected_state = state.clone();
    Router::new()
        .route("/", post(create_public_lead))
        .route(
            "/",
            get(list_landlord_leads)
                .route_layer(from_fn_with_state(protected_state.clone(), auth_middleware)),
        )
        .route(
            "/{id}",
            get(get_landlord_lead)
                .merge(patch(update_lead_status))
                .route_layer(from_fn_with_state(protected_state, auth_middleware)),
        )
}
