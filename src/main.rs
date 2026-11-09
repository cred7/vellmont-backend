mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schemas;
mod services;
mod state;
mod utils;

use axum::{Json, Router, extract::State, routing::get};
use config::AppConfig;
use db::create_db_pool;
use error::AppError;
use serde::Serialize;
use std::net::SocketAddr;
use tracing::info;

use crate::state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    database: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env();
    let db = create_db_pool(&config).await?;

    let app = Router::new()
        .route("/health", get(health))
        .nest(
            "/api/v1",
            routes::api::router(AppState {
                db: db.clone(),
                jwt_secret: config.jwt_secret.clone(),
            }),
        )
        .with_state(AppState {
            db,
            jwt_secret: config.jwt_secret,
        });

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("server failed to start");

    Ok(())
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let database = match state.db.acquire().await {
        Ok(_) => "postgresql",
        Err(_) => "postgresql_unreachable",
    };

    Json(HealthResponse {
        status: if database == "postgresql" {
            "ok"
        } else {
            "degraded"
        },
        database,
    })
}
