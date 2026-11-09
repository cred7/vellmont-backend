use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{
    handlers::transactions::{
        create_transaction, get_landlord_transaction, list_landlord_transactions,
        update_transaction_status,
    },
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(list_landlord_transactions).post(create_transaction),
        )
        .route(
            "/{id}",
            get(get_landlord_transaction).patch(update_transaction_status),
        )
        .route_layer(from_fn_with_state(state, auth_middleware))
}
