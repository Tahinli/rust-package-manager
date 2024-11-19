use rust_package_manager::{database, routing, AppState};
// use axum::routing::{self};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();
    database::establish_connection().await.unwrap();
    let app_state = AppState {};
    let router = routing::route(axum::extract::State(app_state)).await;
    axum::serve(listener, router).await.unwrap();
}
