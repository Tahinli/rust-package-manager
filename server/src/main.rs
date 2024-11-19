use rust_package_manager_server::{database, routing, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("localhost:2345").await.unwrap();
    database::establish_connection().await.unwrap();
    let app_state = AppState {};
    let router = routing::route(axum::extract::State(app_state)).await;
    axum::serve(listener, router).await.unwrap();
}
