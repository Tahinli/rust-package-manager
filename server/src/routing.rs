use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

use crate::{
    database,
    package::package::{Publisher, Version},
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub publisher: Publisher,
    pub version: Version,
}

pub async fn route(State(app_state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
        .route("/package", post(create_package))
        .route("/package/:package_name", get(read_package))
        .route("/package/:package_name", patch(update_package))
        .route("/package/:package_name", delete(delete_package))
        .route("/package/download/:package_name", get(download_package))
        .route("/package/upload", post(upload_package))
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

async fn create_package(Json(package): Json<Package>) -> impl IntoResponse {
    match crate::package::utils::create_package(package).await {
        Some(package) => (StatusCode::CREATED, Json(serde_json::json!(package))),
        None => (StatusCode::BAD_REQUEST, Json(serde_json::json!(""))),
    }
}

async fn read_package(Path(package_name): Path<String>) -> impl IntoResponse {
    match crate::package::utils::read_package(package_name).await {
        Some(package) => (StatusCode::OK, Json(serde_json::json!(package))),
        None => (StatusCode::BAD_REQUEST, Json(serde_json::json!(""))),
    }
}

async fn update_package(
    Path(package_name): Path<String>,
    Json(package): Json<crate::package::package::Package>,
) -> impl IntoResponse {
    match crate::package::utils::update_package(package_name, package).await {
        Some(package) => (StatusCode::ACCEPTED, Json(serde_json::json!(package))),
        None => (StatusCode::BAD_REQUEST, Json(serde_json::json!(""))),
    }
}

async fn delete_package(Path(package_name): Path<String>) -> impl IntoResponse {
    match crate::package::utils::delete_package(package_name).await {
        Some(package) => (StatusCode::NO_CONTENT, Json(serde_json::json!(package))),
        None => (StatusCode::BAD_REQUEST, Json(serde_json::json!(""))),
    }
}

async fn download_package(Path(package_name): Path<String>) -> impl IntoResponse {
    match crate::package::utils::download_package(package_name).await {
        Some(package_file_stream) => (StatusCode::OK, Body::from_stream(package_file_stream)),
        None => (StatusCode::BAD_REQUEST, Body::empty()),
    }
}

async fn upload_package(package_file: Multipart) -> impl IntoResponse {
    match crate::package::utils::upload_package(package_file).await {
        Some(package) => (StatusCode::ACCEPTED, Json(serde_json::json!(package))),
        None => (StatusCode::BAD_REQUEST, Json(serde_json::json!(""))),
    }
}
