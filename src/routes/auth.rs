use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};

use crate::{
    handlers::auth::{login, me, register},
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route(
            "/me",
            get(me).route_layer(from_fn_with_state(state, auth_middleware)),
        )
}
