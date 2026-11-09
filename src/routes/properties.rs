use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{
    handlers::properties::{
        archive_property, create_property, get_property_by_slug, list_properties, update_property,
    },
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_properties).post(create_property))
        .route(
            "/{slug}",
            get(get_property_by_slug)
                .patch(update_property)
                .delete(archive_property),
        )
        .route_layer(from_fn_with_state(state, auth_middleware))
}
