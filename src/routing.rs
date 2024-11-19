use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use tower_http::cors::CorsLayer;

use crate::{database, AppState};

pub async fn route(State(app_state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

async fn alive() -> impl IntoResponse {
    let db_status = match database::is_alive().await {
        true => "alive",
        false => "dead",
    };
    let alive_json = Json(serde_json::json!({
        "server_status": "alive",
        "database_status": db_status
    }));

    (StatusCode::OK, alive_json)
}
