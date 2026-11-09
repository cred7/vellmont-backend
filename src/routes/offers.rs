use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};

use crate::{
    handlers::offers::{
        create_public_offer, get_landlord_offer, list_landlord_offers, update_offer_status,
    },
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    let protected_state = state.clone();
    Router::new()
        .route("/", post(create_public_offer))
        .route(
            "/",
            get(list_landlord_offers)
                .route_layer(from_fn_with_state(protected_state.clone(), auth_middleware)),
        )
        .route(
            "/{id}",
            get(get_landlord_offer)
                .merge(patch(update_offer_status))
                .route_layer(from_fn_with_state(protected_state, auth_middleware)),
        )
}
