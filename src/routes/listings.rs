use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};

use crate::{
    handlers::listings::{
        archive_listing, create_listing, get_landlord_listing, get_public_listing,
        list_landlord_listings, list_public_listings, update_listing,
    },
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    let protected_state = state.clone();

    Router::new()
        .route("/", get(list_public_listings))
        .route(
            "/",
            post(create_listing)
                .route_layer(from_fn_with_state(protected_state.clone(), auth_middleware)),
        )
        .route(
            "/mine",
            get(list_landlord_listings)
                .route_layer(from_fn_with_state(protected_state.clone(), auth_middleware)),
        )
        .route("/{id}", get(get_public_listing))
        .route(
            "/mine/{id}",
            get(get_landlord_listing)
                .route_layer(from_fn_with_state(protected_state.clone(), auth_middleware)),
        )
        .route(
            "/{id}",
            patch(update_listing)
                .delete(archive_listing)
                .route_layer(from_fn_with_state(protected_state, auth_middleware)),
        )
}
