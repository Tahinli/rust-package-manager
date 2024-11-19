use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use tower_http::cors::CorsLayer;

use crate::AppState;

pub async fn route(app_state: State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

async fn alive() -> impl IntoResponse {
    let alive_json = Json(serde_json::json!({
        "server_status": "alive"
    }));

    (StatusCode::OK, alive_json)
}
